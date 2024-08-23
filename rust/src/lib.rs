use allo_isolate::{Isolate, ZeroCopyBuffer};
use std::{
    collections::HashMap,
    future::Future,
    io,
    pin::Pin,
    sync::OnceLock,
    task::{ready, Poll}, time::Duration,
};
use tokio::{
    io::{AsyncRead, AsyncWrite},
    sync::{broadcast, mpsc::{self, error::TryRecvError}, oneshot},
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
    isolate: Isolate, 
    waiter: parking_lot::Mutex<HashMap<u32, Waiter>>,
    streams: parking_lot::RwLock<HashMap<u32, mpsc::UnboundedSender<Vec<u8>>>>,
    listener: parking_lot::Mutex<HashMap<u32, mpsc::UnboundedSender<MemoryStream>>>,
    request_id: parking_lot::Mutex<u32>,
    shutdown: broadcast::Sender<mpsc::Sender<()>>,
    runtime: parking_lot::RwLock<Option<tokio::runtime::Runtime>>,
}

impl Global {
    fn generate_id(&self) -> u32 {
        let mut lock = self.request_id.lock();
        let id = *lock;
        *lock = id.wrapping_add(1);
        id
    }

    fn insert_waiter(&self, id: u32, sender: Waiter) {
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

        lock.insert(id, sender);
        id
    }

    fn send_response_to_dart(&self, res: protocol::Response) {
        tracing::trace!("Send response to dart: {:?}", res);
        let bytes = protocol::Message {
            msg: Some(protocol::message::Msg::Response(res)),
        }
        .encode_to_vec();

        if !self.isolate.post(ZeroCopyBuffer(bytes)) {
            tracing::error!("Failed to post data to dart");
        }
    }

    fn send_request_to_dart(&self, req: protocol::Request) {
        tracing::trace!("Send request to dart: {:?}", req);
        let bytes = protocol::Message {
            msg: Some(protocol::message::Msg::Request(req)),
        }
        .encode_to_vec();

        if !self.isolate.post(ZeroCopyBuffer(bytes)) {
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

        if let Some(stream) = stream {
            return stream.send(data).is_ok();
        }
        tracing::error!("Try to send data to a non-exist stream id: {}", id);
        false
    }

    // fn close_stream(&self, id: u32) -> Option<mpsc::UnboundedSender<Vec<u8>>> {
    //     let mut lock = self.streams.write();
    //     let stream = lock.get_mut(&id);

    //     if let Some(stream) = stream {
    //         // return stream.as_ref().unwrap().send(data).is_ok();
    //         return std::mem::take(stream);
    //     }
    //     None
    // }

    fn remove_stream(&self, id: u32) -> Option<mpsc::UnboundedSender<Vec<u8>>> {
        self.streams.write().remove(&id)
    }
}

pub struct MemoryStream {
    id: u32,
    receiver: mpsc::UnboundedReceiver<Vec<u8>>,
    write_waiter: Option<oneshot::Receiver<protocol::response::Msg>>,
    shutdown_waiter: Option<oneshot::Receiver<protocol::response::Msg>>,
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
                tracing::error!("Memory stream got None message: {}", self.id);
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
            if let Some(ref mut receiver) = self.write_waiter {
                return match ready!(Pin::new(receiver).poll(cx)) {
                    Ok(msg) => {
                        self.write_waiter = None;
                        match msg {
                            protocol::response::Msg::Ok(_) => Poll::Ready(Ok(buf.len())),
                            protocol::response::Msg::Error(error) => Poll::Ready(Err(
                                io::Error::new(io::ErrorKind::Other, format!("{:?}", error)),
                            )),
                        }
                    }
                    Err(err) => {
                        self.write_waiter = None;
                        Poll::Ready(Err(io::Error::new(io::ErrorKind::Other, err)))
                    }
                };
            }
            let (sender, receiver) = oneshot::channel();
            let g = GLOBAL.get().unwrap();
            let id = g.generate_id();
            let msg = protocol::Request {
                id,
                msg: Some(protocol::request::Msg::ChannelData(protocol::ChannelData {
                    data: buf.to_vec(),
                    channel_id: self.id,
                })),
            };
            g.insert_waiter(id, Waiter::sender(sender));
            g.send_request_to_dart(msg);
            self.write_waiter = Some(receiver);
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
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        loop {
            if let Some(ref mut receiver) = self.shutdown_waiter {
                let res = match ready!(Pin::new(receiver).poll(cx)) {
                    Ok(msg) => match msg {
                        protocol::response::Msg::Ok(_) => Poll::Ready(Ok(())),
                        protocol::response::Msg::Error(error) => Poll::Ready(Err(io::Error::new(
                            io::ErrorKind::Other,
                            format!("{:?}", error),
                        ))),
                    },
                    Err(err) => Poll::Ready(Err(io::Error::new(io::ErrorKind::Other, err))),
                };

                self.shutdown_waiter = None;

                return res;
            }
            let g = GLOBAL.get().unwrap();
            let id = g.generate_id();

            let (sender, receiver) = oneshot::channel();

            g.insert_waiter(id, Waiter::Sender(sender));

            g.send_request_to_dart(protocol::Request {
                id,
                msg: Some(protocol::request::Msg::ChannelClose(
                    protocol::ChannelClose {
                        channel_id: self.id,
                    },
                )),
            });
            self.shutdown_waiter = Some(receiver);
        }
    }
}

enum Waiter {
    // Func(Box<dyn FnOnce(&Global, protocol::response::Msg) + Send + Sync>),
    Sender(oneshot::Sender<protocol::response::Msg>),
}

impl Waiter {
    // fn func(f: impl FnOnce(&Global, protocol::response::Msg) + Send + Sync + 'static) -> Self {
    //     Self::Func(Box::new(f))
    // }
    fn sender(sender: oneshot::Sender<protocol::response::Msg>) -> Self {
        Self::Sender(sender)
    }
}

impl std::fmt::Debug for Waiter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Self::Func(_) => write!(f, "Waiter::Func"),
            Self::Sender(_) => write!(f, "Waiter::Sender"),
        }
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
        GLOBAL.get().unwrap().remove_listener(self.id);
    }
}

impl MemoryListener {
    pub fn port(&self) -> u32 {
        self.id
    }

    pub fn bind(port: Option<u32>) -> io::Result<Self> {
        let (sender, receiver) = mpsc::unbounded_channel();
        let g = GLOBAL.get().expect("Uninitlize");
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
    let buf = unsafe { std::slice::from_raw_parts(data, len) };

    let msg = protocol::Message::decode(buf).unwrap();
    match msg.msg {
        Some(protocol::message::Msg::Response(res)) => {
            let g = GLOBAL.get().unwrap();
            let Some(sender) = g.waiter.lock().remove(&res.id) else {
                tracing::error!("No such waiter is waiting for response: {}", res.id);
                return;
            };
            if let Some(msg) = res.msg {
                match sender {
                    // Waiter::Func(func) => func(g, msg),
                    Waiter::Sender(sender) => {
                        if let Err(err) = sender.send(msg) {
                            tracing::warn!("Failed to send response: {:?}", err);
                        }
                    }
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

                    let g = GLOBAL.get().unwrap();
                    let id = g.insert_stream(sender);

                    let stream = MemoryStream {
                        id,
                        receiver,
                        write_waiter: None,
                        shutdown_waiter: None,
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
                                code: protocol::error::Code::Unbind as _,
                                msg: format!("Port {} is not bound", new_channel.listener_id),
                            })),
                        });
                    }

                    // g.send_response_to_dart(&protocol::Response { id, msg: })
                }
                Some(protocol::request::Msg::ChannelData(data)) => {
                    let g = GLOBAL.get().unwrap();
                    if g.send_stream_data(data.channel_id, data.data) {
                        g.send_response_to_dart(protocol::Response {
                            id: req.id,
                            msg: Some(protocol::response::Msg::Ok(protocol::Ok { msg: None })),
                        })
                    } else {
                        g.send_response_to_dart(protocol::Response {
                            id: req.id,
                            msg: Some(protocol::response::Msg::Error(protocol::Error {
                                code: protocol::error::Code::ChannelNotFound as _,
                                msg: format!("Channel: {} not found", data.channel_id),
                            })),
                        })
                    }
                }
                Some(protocol::request::Msg::ChannelClose(close)) => {
                    let g = GLOBAL.get().unwrap();

                    if g.remove_stream(close.channel_id).is_some() {
                        g.send_response_to_dart(protocol::Response {
                            id: req.id,
                            msg: Some(protocol::response::Msg::Ok(protocol::Ok { msg: None })),
                        })
                    } else {
                        tracing::warn!("Try to close a non-exist stream: {}", close.channel_id);
                        g.send_response_to_dart(protocol::Response {
                            id: req.id,
                            msg: Some(protocol::response::Msg::Error(protocol::Error {
                                code: protocol::error::Code::ChannelNotFound as _,
                                msg: format!("Channel: {} not found", close.channel_id),
                            })),
                        })
                    }
                } // _ => {}
            }
        }
        None => {
            tracing::error!("Got None message from dart");
        }
    }
}

static GLOBAL: OnceLock<Global> = OnceLock::new();


fn try_recv_timeout<T>(receiver: &mut mpsc::Receiver<T>, timeout: Duration) -> bool {
    let start = std::time::Instant::now();

    loop {
        return match receiver.try_recv() {
            Ok(_) => true,
            Err(TryRecvError::Empty) => {
                let now = std::time::Instant::now();
                let offset = now - start;
                if offset > timeout {
                    return false;
                } else {
                    continue;
                }
            },
            Err(TryRecvError::Disconnected) => true,
        };
    }

}

#[no_mangle]
pub extern "C" fn crosscall_destroy() {
    let (sender, mut receiver) = mpsc::channel(128);

    let g = GLOBAL.get().unwrap();
    if let Ok(size) = g.shutdown.send(sender) {
        for _ in 0..size {
            let try_times = 3;
            let timeout = Duration::from_millis(300);

            for _ in 0..try_times {
                if try_recv_timeout(&mut receiver, timeout) {
                    break;
                }
            }
        }
    }

    if let Some(rt) = g.runtime.write().take() {
        rt.shutdown_background();
    }
}

#[no_mangle]
pub extern "C" fn crosscall_rust_initialize(port: i64, thread: u32) {
    let isolate = Isolate::new(port);

    let runtime = match thread {
        0 => tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build(),
        1 => tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build(),
        2.. => tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .worker_threads(thread as usize)
            .build(),
    };

    let g = Global {
        isolate,
        waiter: Default::default(),
        streams: Default::default(),
        listener: Default::default(),
        request_id: Default::default(),
        shutdown: broadcast::channel(128).0,
        runtime: parking_lot::RwLock::new(Some(
            runtime.expect("Failed to start tokio async runtime"),
        )),
    };

    // let guard = g.runtime.enter();

    // g.runtime.spawn(async move {

    // });
    // drop(guard);
    GLOBAL.set(g).expect("Failed to set global resource");
}

#[doc(hidden)]
#[inline]
pub fn spawn<F>(future: F) -> JoinHandle<Option<F::Output>>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let g = GLOBAL.get().unwrap();
    let rt = g.runtime.read();
    let guard = rt.as_ref().unwrap().enter();

    let mut receiver = g.shutdown.subscribe();

    let res = tokio::spawn(async move {
        tokio::select! {
            res = future => {
               Some(res)
            }
            res = receiver.recv() => {
                let sender = match res {
                    Ok(sender) => sender,
                    Err(err) => {
                        tracing::error!("Failed to receiver from shutdown notify: {:?}", err);
                        return None;
                    },
                };
                let res = sender.send(()).await;
                tracing::info!("reply to shutdown notify: {:?}", res);
                None
            }
        }
    });

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
