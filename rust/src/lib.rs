use allo_isolate::Isolate;
use std::{
    collections::HashMap,
    future::Future,
    io,
    pin::Pin,
    sync::OnceLock,
    task::{ready, Poll},
};
use tokio::{
    io::{AsyncRead, AsyncWrite},
    sync::{mpsc, oneshot},
    task::JoinHandle,
};

use prost::{
    bytes::{BufMut, BytesMut},
    Message,
};

mod protocol;

// pub mod hello {
//     tonic::include_proto!("hello");
// }

// static DART_PORT: OnceLock<(mpsc::UnboundedSender<()>, mpsc::UnboundedReceiver<()>)> = OnceLock::new();

#[derive(Debug)]
struct Global {
    isolate: parking_lot::Mutex<Isolate>, // maybe we can remove mutex here ??
    waiter: parking_lot::Mutex<HashMap<u32, oneshot::Sender<protocol::response::Msg>>>,
    streams: parking_lot::RwLock<HashMap<u32, Option<mpsc::UnboundedSender<Vec<u8>>>>>,
    listener: parking_lot::Mutex<HashMap<u32, mpsc::UnboundedSender<MemoryStream>>>,
    request_id: parking_lot::Mutex<u32>,
    runtime: tokio::runtime::Runtime,
}

impl Global {
    fn generate_id(&self) -> u32 {
        let mut lock = self.request_id.lock();
        let id = *lock;
        *lock += 1;
        id
    }

    fn insert_waiter(&self, id: u32, sender: oneshot::Sender<protocol::response::Msg>) {
        self.waiter.lock().insert(id, sender);
    }

    fn insert_stream(&self, sender: mpsc::UnboundedSender<Vec<u8>>) -> u32 {
        let mut lock = self.streams.write();
        let mut id = 0;
        for (k, _) in lock.iter() {
            if *k >= id {
                id = k + 1;
            }
        }

        lock.insert(id, Some(sender));
        id
    }

    fn send_response_to_dart(&self, res: protocol::Response) {
        tracing::trace!("Send response to dart: {:?}", res);
        let bytes = protocol::Message {
            msg: Some(protocol::message::Msg::Response(res)),
        }
        .encode_to_vec();

        if !self.isolate.lock().post(bytes) {
            tracing::error!("Failed to post data to dart");
        }
    }

    fn send_request_to_dart(&self, req: protocol::Request) {
        tracing::trace!("Send request to dart: {:?}", req);
        let bytes = protocol::Message {
            msg: Some(protocol::message::Msg::Request(req)),
        }
        .encode_to_vec();

        if !self.isolate.lock().post(bytes) {
            tracing::error!("Failed to post data to dart");
        }
    }

    fn insert_listener(&self, sender: mpsc::UnboundedSender<MemoryStream>) -> u32 {
        let mut lock = self.listener.lock();
        let mut id = 0;
        for (k, _) in lock.iter() {
            if *k >= id {
                id = k + 1;
            }
        }

        lock.insert(id, sender);
        id
    }

    fn send_stream_to_listener(&self, id: u32, stream: MemoryStream) -> Result<(), MemoryStream> {
        match self.listener.lock().get(&id) {
            Some(listener) => listener.send(stream).map_err(|v| v.0),
            None => {
                tracing::error!("Failed to send stream to listener");
                Err(stream)
            }
        }
    }

    fn remove_listener(&self, id: u32) -> Option<mpsc::UnboundedSender<MemoryStream>> {
        self.listener.lock().remove(&id)
    }

    fn send_stream_data(&self, id: u32, data: Vec<u8>) -> bool {
        let lock = self.streams.read();
        let stream = lock.get(&id);

        if let Some(Some(stream)) = stream {
            return stream.send(data).is_ok();
        }
        tracing::error!("Try to send data to a non-exist stream id: {}", id);
        false
    }

    fn close_stream(&self, id: u32) -> Option<mpsc::UnboundedSender<Vec<u8>>> {
        let mut lock = self.streams.write();
        let stream = lock.get_mut(&id);

        if let Some(stream) = stream {
            // return stream.as_ref().unwrap().send(data).is_ok();
            return std::mem::take(stream);
        }
        None
    }

    fn remove_stream(&self, id: u32) -> Option<mpsc::UnboundedSender<Vec<u8>>> {
        self.streams.write().remove(&id)?
    }
}

pub struct MemoryStream {
    id: u32,
    receiver: mpsc::UnboundedReceiver<Vec<u8>>,
    write_receiver: Option<oneshot::Receiver<protocol::response::Msg>>,
    buf: BytesMut,
}

#[derive(Clone)]
pub struct MemoryInfo {
    pub id: u32,
}

impl AsyncRead for MemoryStream {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        if !self.buf.is_empty() {
            let min = std::cmp::min(self.buf.len(), buf.remaining());
            buf.put(self.buf.split_to(min));
        }
        match ready!(self.receiver.poll_recv(cx)) {
            Some(data) => {
                let remaining = buf.remaining();
                if data.len() > remaining {
                    buf.put(&data[..remaining]);
                    self.buf.put(&data[remaining..])
                } else {
                    buf.put(data.as_ref());
                }
            }
            None => {
                tracing::error!("Memory got None message: {}", self.id);
                return Poll::Ready(Err(io::Error::new(
                    io::ErrorKind::ConnectionAborted,
                    "None msg receive",
                )));
            }
        }
        Poll::Ready(Ok(()))
    }
}

impl AsyncWrite for MemoryStream {
    fn poll_write(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<Result<usize, std::io::Error>> {
        loop {
            if let Some(ref mut receiver) = self.write_receiver {
                return match ready!(Pin::new(receiver).poll(cx)) {
                    Ok(msg) => {
                        self.write_receiver = None;
                        match msg {
                            protocol::response::Msg::Ok(_) => Poll::Ready(Ok(buf.len())),
                            protocol::response::Msg::Error(error) => Poll::Ready(Err(
                                io::Error::new(io::ErrorKind::Other, format!("{:?}", error)),
                            )),
                        }
                    }
                    Err(err) => {
                        self.write_receiver = None;
                        Poll::Ready(Err(io::Error::new(io::ErrorKind::Other, err)))
                    }
                };
            }
            let (sender, receiver) = oneshot::channel();
            let lock = GLOBAL.read();
            let g = lock.get().unwrap();
            let id = g.generate_id();
            let msg = protocol::Request {
                id,
                msg: Some(protocol::request::Msg::ChannelData(protocol::ChannelData {
                    data: buf.to_vec(),
                    channel_id: self.id,
                })),
            };
            g.insert_waiter(id, sender);
            g.send_request_to_dart(msg);
            self.write_receiver = Some(receiver);
        }
        // GLOBAL.get().unwrap().insert_waiter(, sender)
        // GLOBAL.get().unwrap().send_channel_data(self.id, buf.to_vec());
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        std::task::Poll::Ready(Ok(()))
    }
}

impl Drop for MemoryStream {
    fn drop(&mut self) {
        GLOBAL.read().get().unwrap().remove_stream(self.id);
    }
}

impl tonic::transport::server::Connected for MemoryStream {
    type ConnectInfo = MemoryInfo;

    fn connect_info(&self) -> Self::ConnectInfo {
        MemoryInfo { id: self.id }
    }
}

impl MemoryStream {
    pub fn id(&self) -> u32 {
        self.id
    }
}

impl tokio_stream::Stream for MemoryListener {
    type Item = Result<MemoryStream, io::Error>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        match ready!(self.receiver.poll_recv(cx)) {
            Some(res) => {
                tracing::info!("New connection: {}", res.id);
                Poll::Ready(Some(Ok(res)))
            }
            None => Poll::Ready(None),
        }
    }
}

pub struct MemoryListener {
    id: u32,
    receiver: mpsc::UnboundedReceiver<MemoryStream>,
}

impl Drop for MemoryListener {
    fn drop(&mut self) {
        GLOBAL.read().get().unwrap().remove_listener(self.id);
    }
}

impl MemoryListener {
    pub fn port(&self) -> u32 {
        self.id
    }

    pub fn bind(port: Option<u32>) -> io::Result<Self> {
        let (sender, receiver) = mpsc::unbounded_channel();
        let lock = GLOBAL.read();
        let g = lock.get().expect("Uninitlize");
        let id = match port {
            Some(id) => {
                let mut lock = g.listener.lock();
                if lock.contains_key(&id) {
                    return Err(io::Error::new(io::ErrorKind::AddrInUse, "Port is in use"));
                }

                lock.insert(id, sender);

                id
            }
            None => g.insert_listener(sender),
        };

        Ok(Self { id, receiver })
    }

    pub async fn accept(&mut self) -> Option<MemoryStream> {
        self.receiver.recv().await
    }
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn crosscall_write_to_rust(data: *mut u8, len: usize) {
    let buf = unsafe { std::slice::from_raw_parts_mut(data, len) };

    let msg = protocol::Message::decode(buf as &[u8]).unwrap();
    match msg.msg {
        Some(protocol::message::Msg::Response(res)) => {
            let Some(sender) = GLOBAL
                .read()
                .get()
                .expect("Uninitialize")
                .waiter
                .lock()
                .remove(&res.id)
            else {
                tracing::error!("No such waiter is waiting for response: {}", res.id);
                return;
            };
            if let Some(msg) = res.msg {
                if let Err(err) = sender.send(msg) {
                    tracing::warn!("Failed to send response: {:?}", err);
                }
            } else {
                tracing::error!("Got None msg in response");
            }
        }
        Some(protocol::message::Msg::Request(req)) => {
            match req.msg {
                None => {
                    tracing::error!("Got None msg in request");
                }
                Some(protocol::request::Msg::NewChannel(new_channel)) => {
                    let (sender, receiver) = mpsc::unbounded_channel();

                    let lock = GLOBAL.read();
                    let g = lock.get().unwrap();
                    let id = g.insert_stream(sender);

                    let stream = MemoryStream {
                        id,
                        receiver,
                        write_receiver: None,
                        buf: Default::default(),
                    };

                    if g.send_stream_to_listener(new_channel.listener_id, stream)
                        .is_ok()
                    {
                        g.send_response_to_dart(protocol::Response {
                            id: req.id,
                            msg: Some(protocol::response::Msg::Ok(protocol::Ok {
                                msg: Some(protocol::ok::Msg::ChannelId(protocol::ChannelId {
                                    channel_id: id,
                                })),
                            })),
                        });
                    } else {
                        g.send_response_to_dart(protocol::Response {
                            id: req.id,
                            msg: Some(protocol::response::Msg::Error(protocol::Error {
                                code: -1,
                                msg: "Not bind".to_string(),
                            })),
                        });
                    }

                    // g.send_response_to_dart(&protocol::Response { id, msg: })
                }
                Some(protocol::request::Msg::ChannelData(data)) => {
                    let lock = GLOBAL.read();
                    let g = lock.get().unwrap();
                    if g.send_stream_data(data.channel_id, data.data) {
                        g.send_response_to_dart(protocol::Response {
                            id: req.id,
                            msg: Some(protocol::response::Msg::Ok(protocol::Ok { msg: None })),
                        })
                    } else {
                        g.send_response_to_dart(protocol::Response {
                            id: req.id,
                            msg: Some(protocol::response::Msg::Error(protocol::Error {
                                code: -2,
                                msg: "Channel not found".to_string(),
                            })),
                        })
                    }
                }
                Some(protocol::request::Msg::ChannelClose(close)) => {
                    if GLOBAL
                        .read()
                        .get()
                        .unwrap()
                        .close_stream(close.channel_id)
                        .is_none()
                    {
                        tracing::warn!("Try to close a non-exist stream: {}", close.channel_id);
                    }
                } // _ => {}
            }
        }
        None => {
            tracing::error!("Got None message fro dart");
        }
    }
}

static GLOBAL: parking_lot::RwLock<OnceLock<Global>> = parking_lot::RwLock::new(OnceLock::new());

#[no_mangle]
pub extern "C" fn crosscall_destroy() {
    GLOBAL.write().take();
}

#[no_mangle]
pub extern "C" fn crosscall_rust_initialize(port: i64, thread: u32) {
    let isolate = parking_lot::Mutex::new(Isolate::new(port));

    let runtime = if thread == 1 {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
    } else if thread > 1 {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .worker_threads(thread as usize)
            .build()
    } else {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
    };

    let g = Global {
        isolate,
        waiter: Default::default(),
        streams: Default::default(),
        listener: Default::default(),
        request_id: Default::default(),
        runtime: runtime.expect("Failed to start tokio async runtime"),
    };

    // let guard = g.runtime.enter();

    // g.runtime.spawn(async move {

    // });
    // drop(guard);
    GLOBAL.read().set(g).expect("Failed to set global resource");
}

#[inline]
pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let lock = GLOBAL.read();
    let g = lock.get().unwrap();
    let guard = g.runtime.enter();

    let res = g.runtime.spawn(future);

    drop(guard);

    res
}

#[macro_export]
macro_rules! generate_endpoint {
    () => {
        #[no_mangle]
        extern "C" fn crosscall_start() {
            ::crosscall::spawn(start());
        }
    };
}
