/// Support for doing something awesome.
///
/// More dartdocs go here.
library;

import 'dart:async';
import 'dart:ffi';
import 'dart:isolate';

import 'package:ffi/ffi.dart';
import 'package:grpc/grpc_connection_interface.dart';
import 'package:http2/transport.dart';

import 'package:grpc/src/client/client_transport_connector.dart';
import 'package:grpc/src/client/http2_connection.dart';
import 'package:crosscall/src/protocol/protocol.pb.dart' as protocol;

class TransportConnector extends ClientTransportConnector {
  int port;
  late MemoryStream stream;

  TransportConnector(this.port);
  @override
  String get authority => port.toString();

  @override
  Future<ClientTransportConnection> connect() async {
    // socket = await Socket.connect(host, port);

    stream = await MemoryStream.connect(port);

    return ClientTransportConnection.viaStreams(stream, stream);
  }

  @override
  Future get done => stream.done;

  @override
  void shutdown() {
    stream.close();
  }
}

class ClientChannel extends ClientChannelBase {
  int port;

  ClientChannel(this.port);

  @override
  ClientConnection createConnection() {
    return Http2ClientConnection.fromClientTransportConnector(
        TransportConnector(port), ChannelOptions());
  }
}

typedef DartPostCObject = Pointer Function(
    Pointer<NativeFunction<Int8 Function(Int64, Pointer<Dart_CObject>)>>);

class Global {
  late void Function(Pointer, int) _writeToRust;
  late void Function() _destroy;

  late ReceivePort _receivePort;
  int _requestId = 0;

  Map<int, void Function(Global, protocol.Response)> _waiters = {};
  Map<int, StreamSink<List<int>>> _streams = {};

  Global._() {
    DynamicLibrary library = DynamicLibrary.open(_defaultConfig.libPath);
    _writeToRust = library.lookupFunction<Void Function(Pointer, Int32),
        void Function(Pointer, int)>("crosscall_write_to_rust");

    _destroy = library
        .lookupFunction<Void Function(), void Function()>("crosscall_destroy");

    var start = library
        .lookupFunction<Void Function(), void Function()>("crosscall_start");

    var rustInitialize = library.lookupFunction<Void Function(Int64, Uint32),
        void Function(int, int)>("crosscall_rust_initialize");

    final storeDartPostCObject =
        library.lookupFunction<DartPostCObject, DartPostCObject>(
      'store_dart_post_cobject',
    );
    storeDartPostCObject(NativeApi.postCObject);

    _receivePort = ReceivePort();
    _receivePort.listen(_message);
    rustInitialize(
        _receivePort.sendPort.nativePort, _defaultConfig.rustWorkerThread ?? 0);

    start();
  }

  int _nextId() {
    int id = _requestId;
    _requestId += 1;
    return id;
  }

  void _sendRequest(protocol.Request req,
      void Function(Global global, protocol.Response) res) {
    var msg = protocol.Message(request: req);
    var bytes = msg.writeToBuffer();

    _waiters[req.id] = res;

    _writeBytesToRust(bytes);
  }

  void _writeBytesToRust(List<int> data) {
    final Pointer<Uint8> buf = malloc.allocate(data.length);
    buf.asTypedList(data.length).setAll(0, data);

    _writeToRust(buf, data.length);
    malloc.free(buf);
  }

  void _writeMessageToRust(protocol.Message msg) {
    _writeBytesToRust(msg.writeToBuffer());
  }

  void _deinitialize() {
    _destroy();
  }

  void _message(dynamic data) {
    var msg = protocol.Message.fromBuffer(data);
    if (msg.whichMsg() == protocol.Message_Msg.request) {
      var req = msg.request;

      if (req.whichMsg() == protocol.Request_Msg.channelClose) {
        var stream = _streams[req.channelClose.channelId];
        if (stream == null) {
          var response = protocol.Message(
              response: protocol.Response(
                  id: req.id,
                  error: protocol.Error(
                      code: protocol.Error_Code.Unbind,
                      msg: "Stream not found")));
          _writeMessageToRust(response);
        } else {
          var response = protocol.Message(
              response: protocol.Response(id: req.id, ok: protocol.Ok()));
          _writeMessageToRust(response);
        }
      } else if (req.whichMsg() == protocol.Request_Msg.channelData) {
        var data = req.channelData;
        var stream = _streams[data.channelId];
        protocol.Response response;
        if (stream == null) {
          response = protocol.Response(
              id: req.id,
              error: protocol.Error(
                  code: protocol.Error_Code.Unbind, msg: "Channel not found"));
        } else {
          stream.add(data.data);
          response = protocol.Response(id: req.id, ok: protocol.Ok());
        }

        _writeMessageToRust(protocol.Message(response: response));
      }
    } else if (msg.whichMsg() == protocol.Message_Msg.response) {
      var res = msg.response;

      _waiters.remove(res.id)!(this, res);
    } else if (msg.whichMsg() == protocol.Message_Msg.notSet) {
      throw Exception("Message not set");
    }
  }

  static Global? _self;

  factory Global() {
    _self ??= Global._();
    return _self!;
  }
}

class MemoryStream implements Stream<List<int>>, StreamSink<List<int>> {
  int get channelId => _channelId;

  late int _channelId;

  late Stream<List<int>> _receiver;
  late StreamController<List<int>> _sink = StreamController();

  static Future<MemoryStream> connect(int listenerId) async {
    var id = Global()._nextId();
    var req = protocol.Request(
        id: id, newChannel: protocol.NewChannel(listenerId: listenerId));

    StreamController<MemoryStream> waiter = StreamController();

    Global()._sendRequest(req, (global, res) {
      if (res.whichMsg() == protocol.Response_Msg.ok) {
        var channelid = res.ok.channelId.channelId;

        StreamController<List<int>> stream = StreamController();

        global._streams[channelid] = stream.sink;

        MemoryStream self = MemoryStream._();
        self._channelId = channelid;
        self._receiver = stream.stream;

        waiter.add(self);
      } else {
        throw Exception("Channel not found");
      }
    });

    return waiter.stream.first;

    // var res = await waiter.stream.first;

    // if (res.whichMsg() == protocol.Response_Msg.ok) {
    //   var channelid = res.ok.channelId.channelId;
    //   MemoryStream self = MemoryStream._();
    //   self._channelId = channelid;
    //   return self;
    // } else {
    //   throw Exception("Channel not found");
    // }
  }

  MemoryStream._() {
    _sink.stream.listen(
      (data) {
        int id = Global()._nextId();

        var req = protocol.Request(
            id: id,
            channelData: protocol.ChannelData(
              data: data,
              channelId: _channelId,
            ));

        Global()._sendRequest(req, (global, res) {
          assert(res.id == req.id);
          assert(res.whichMsg() == protocol.Response_Msg.ok);
        });
      },
      onDone: () {
        int id = Global()._nextId();
        var req = protocol.Request(
            id: id,
            channelClose: protocol.ChannelClose(
              channelId: _channelId,
            ));

        Global()._sendRequest(req, (global, res) {
          assert(res.id == req.id);
          print(
              "response: ${res.toDebugString()} from request: ${req.toDebugString()}}");
        });
      },
      onError: (Object Error) {
        throw Error;
      },
    );
  }

  @override
  Future<bool> any(bool Function(List<int> element) test) {
    return _receiver.any(test);
  }

  @override
  Stream<List<int>> asBroadcastStream(
      {void Function(StreamSubscription<List<int>> subscription)? onListen,
      void Function(StreamSubscription<List<int>> subscription)? onCancel}) {
    return _receiver.asBroadcastStream(onListen: onListen, onCancel: onCancel);
  }

  @override
  Stream<E> asyncExpand<E>(Stream<E>? Function(List<int> event) convert) {
    return _receiver.asyncExpand(convert);
  }

  @override
  Stream<E> asyncMap<E>(FutureOr<E> Function(List<int> event) convert) {
    return _receiver.asyncMap(convert);
  }

  @override
  Stream<R> cast<R>() {
    return _receiver.cast();
  }

  @override
  Future<bool> contains(Object? needle) {
    return _receiver.contains(needle);
  }

  @override
  Stream<List<int>> distinct(
      [bool Function(List<int> previous, List<int> next)? equals]) {
    return _receiver.distinct(equals);
  }

  @override
  Future<E> drain<E>([E? futureValue]) {
    return _receiver.drain(futureValue);
  }

  @override
  Future<List<int>> elementAt(int index) {
    return _receiver.elementAt(index);
  }

  @override
  Future<bool> every(bool Function(List<int> element) test) {
    return _receiver.every(test);
  }

  @override
  Stream<S> expand<S>(Iterable<S> Function(List<int> element) convert) {
    return _receiver.expand(convert);
  }

  @override
  Future<List<int>> get first => _receiver.first;

  @override
  Future<List<int>> firstWhere(bool Function(List<int> element) test,
      {List<int> Function()? orElse}) {
    return _receiver.firstWhere(test);
  }

  @override
  Future<S> fold<S>(
      S initialValue, S Function(S previous, List<int> element) combine) {
    return _receiver.fold(initialValue, combine);
  }

  @override
  Future<void> forEach(void Function(List<int> element) action) {
    return _receiver.forEach(action);
  }

  @override
  Stream<List<int>> handleError(Function onError,
      {bool Function(dynamic error)? test}) {
    return _receiver.handleError(onError);
  }

  @override
  bool get isBroadcast => _receiver.isBroadcast;

  @override
  Future<bool> get isEmpty => _receiver.isEmpty;

  @override
  Future<String> join([String separator = ""]) {
    return _receiver.join(separator);
  }

  @override
  Future<List<int>> get last => _receiver.last;

  @override
  Future<List<int>> lastWhere(bool Function(List<int> element) test,
      {List<int> Function()? orElse}) {
    return _receiver.lastWhere(test, orElse: orElse);
  }

  @override
  Future<int> get length => _receiver.length;

  @override
  StreamSubscription<List<int>> listen(void Function(List<int> event)? onData,
      {Function? onError, void Function()? onDone, bool? cancelOnError}) {
    return _receiver.listen(onData,
        onError: onError, onDone: onDone, cancelOnError: cancelOnError);
  }

  @override
  Stream<S> map<S>(S Function(List<int> event) convert) {
    return _receiver.map(convert);
  }

  @override
  Future pipe(StreamConsumer<List<int>> streamConsumer) {
    return _receiver.pipe(streamConsumer);
  }

  @override
  Future<List<int>> reduce(
      List<int> Function(List<int> previous, List<int> element) combine) {
    return _receiver.reduce(combine);
  }

  @override
  Future<List<int>> get single => _receiver.single;

  @override
  Future<List<int>> singleWhere(bool Function(List<int> element) test,
      {List<int> Function()? orElse}) {
    return _receiver.singleWhere(test, orElse: orElse);
  }

  @override
  Stream<List<int>> skip(int count) {
    return _receiver.skip(count);
  }

  @override
  Stream<List<int>> skipWhile(bool Function(List<int> element) test) {
    return _receiver.skipWhile(test);
  }

  @override
  Stream<List<int>> take(int count) {
    return _receiver.take(count);
  }

  @override
  Stream<List<int>> takeWhile(bool Function(List<int> element) test) {
    return _receiver.takeWhile(test);
  }

  @override
  Stream<List<int>> timeout(Duration timeLimit,
      {void Function(EventSink<List<int>> sink)? onTimeout}) {
    return _receiver.timeout(timeLimit, onTimeout: onTimeout);
  }

  @override
  Future<List<List<int>>> toList() {
    return _receiver.toList();
  }

  @override
  Future<Set<List<int>>> toSet() {
    return _receiver.toSet();
  }

  @override
  Stream<S> transform<S>(StreamTransformer<List<int>, S> streamTransformer) {
    return _receiver.transform(streamTransformer);
  }

  @override
  Stream<List<int>> where(bool Function(List<int> event) test) {
    return _receiver.where(test);
  }

  @override
  void add(List<int> event) {
    _sink.add(event);
  }

  @override
  void addError(Object error, [StackTrace? stackTrace]) {}

  @override
  Future addStream(Stream<List<int>> stream) {
    return _sink.addStream(stream);
  }

  @override
  Future close() {
    return _sink.close();
  }

  @override
  Future get done {
    return _sink.done;
  }
}

class Config {
  String libPath;
  int? rustWorkerThread;
  Config({required this.libPath, this.rustWorkerThread});
}

Config _defaultConfig = Config(libPath: "libhub.so");

void setConfig(Config config) {
  _defaultConfig = config;
}

void destroy() {
  if (Global._self != null) {
    Global()._deinitialize();
  }
}
