//
//  Generated code. Do not modify.
//  source: protocol.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'protocol.pbenum.dart';

export 'protocol.pbenum.dart';

enum Request_Msg {
  channelClose, 
  channelData, 
  newChannel, 
  notSet
}

class Request extends $pb.GeneratedMessage {
  factory Request({
    $core.int? id,
    ChannelClose? channelClose,
    ChannelData? channelData,
    NewChannel? newChannel,
  }) {
    final $result = create();
    if (id != null) {
      $result.id = id;
    }
    if (channelClose != null) {
      $result.channelClose = channelClose;
    }
    if (channelData != null) {
      $result.channelData = channelData;
    }
    if (newChannel != null) {
      $result.newChannel = newChannel;
    }
    return $result;
  }
  Request._() : super();
  factory Request.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Request.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static const $core.Map<$core.int, Request_Msg> _Request_MsgByTag = {
    2 : Request_Msg.channelClose,
    3 : Request_Msg.channelData,
    4 : Request_Msg.newChannel,
    0 : Request_Msg.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Request', package: const $pb.PackageName(_omitMessageNames ? '' : 'protocol'), createEmptyInstance: create)
    ..oo(0, [2, 3, 4])
    ..a<$core.int>(1, _omitFieldNames ? '' : 'id', $pb.PbFieldType.OU3)
    ..aOM<ChannelClose>(2, _omitFieldNames ? '' : 'channelClose', protoName: 'channelClose', subBuilder: ChannelClose.create)
    ..aOM<ChannelData>(3, _omitFieldNames ? '' : 'channelData', protoName: 'channelData', subBuilder: ChannelData.create)
    ..aOM<NewChannel>(4, _omitFieldNames ? '' : 'newChannel', protoName: 'newChannel', subBuilder: NewChannel.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Request clone() => Request()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Request copyWith(void Function(Request) updates) => super.copyWith((message) => updates(message as Request)) as Request;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Request create() => Request._();
  Request createEmptyInstance() => create();
  static $pb.PbList<Request> createRepeated() => $pb.PbList<Request>();
  @$core.pragma('dart2js:noInline')
  static Request getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Request>(create);
  static Request? _defaultInstance;

  Request_Msg whichMsg() => _Request_MsgByTag[$_whichOneof(0)]!;
  void clearMsg() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  $core.int get id => $_getIZ(0);
  @$pb.TagNumber(1)
  set id($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => clearField(1);

  @$pb.TagNumber(2)
  ChannelClose get channelClose => $_getN(1);
  @$pb.TagNumber(2)
  set channelClose(ChannelClose v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasChannelClose() => $_has(1);
  @$pb.TagNumber(2)
  void clearChannelClose() => clearField(2);
  @$pb.TagNumber(2)
  ChannelClose ensureChannelClose() => $_ensure(1);

  @$pb.TagNumber(3)
  ChannelData get channelData => $_getN(2);
  @$pb.TagNumber(3)
  set channelData(ChannelData v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasChannelData() => $_has(2);
  @$pb.TagNumber(3)
  void clearChannelData() => clearField(3);
  @$pb.TagNumber(3)
  ChannelData ensureChannelData() => $_ensure(2);

  @$pb.TagNumber(4)
  NewChannel get newChannel => $_getN(3);
  @$pb.TagNumber(4)
  set newChannel(NewChannel v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasNewChannel() => $_has(3);
  @$pb.TagNumber(4)
  void clearNewChannel() => clearField(4);
  @$pb.TagNumber(4)
  NewChannel ensureNewChannel() => $_ensure(3);
}

enum Response_Msg {
  ok, 
  error, 
  notSet
}

class Response extends $pb.GeneratedMessage {
  factory Response({
    $core.int? id,
    Ok? ok,
    Error? error,
  }) {
    final $result = create();
    if (id != null) {
      $result.id = id;
    }
    if (ok != null) {
      $result.ok = ok;
    }
    if (error != null) {
      $result.error = error;
    }
    return $result;
  }
  Response._() : super();
  factory Response.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Response.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static const $core.Map<$core.int, Response_Msg> _Response_MsgByTag = {
    2 : Response_Msg.ok,
    3 : Response_Msg.error,
    0 : Response_Msg.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Response', package: const $pb.PackageName(_omitMessageNames ? '' : 'protocol'), createEmptyInstance: create)
    ..oo(0, [2, 3])
    ..a<$core.int>(1, _omitFieldNames ? '' : 'id', $pb.PbFieldType.OU3)
    ..aOM<Ok>(2, _omitFieldNames ? '' : 'ok', subBuilder: Ok.create)
    ..aOM<Error>(3, _omitFieldNames ? '' : 'error', subBuilder: Error.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Response clone() => Response()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Response copyWith(void Function(Response) updates) => super.copyWith((message) => updates(message as Response)) as Response;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Response create() => Response._();
  Response createEmptyInstance() => create();
  static $pb.PbList<Response> createRepeated() => $pb.PbList<Response>();
  @$core.pragma('dart2js:noInline')
  static Response getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Response>(create);
  static Response? _defaultInstance;

  Response_Msg whichMsg() => _Response_MsgByTag[$_whichOneof(0)]!;
  void clearMsg() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  $core.int get id => $_getIZ(0);
  @$pb.TagNumber(1)
  set id($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => clearField(1);

  @$pb.TagNumber(2)
  Ok get ok => $_getN(1);
  @$pb.TagNumber(2)
  set ok(Ok v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasOk() => $_has(1);
  @$pb.TagNumber(2)
  void clearOk() => clearField(2);
  @$pb.TagNumber(2)
  Ok ensureOk() => $_ensure(1);

  @$pb.TagNumber(3)
  Error get error => $_getN(2);
  @$pb.TagNumber(3)
  set error(Error v) { setField(3, v); }
  @$pb.TagNumber(3)
  $core.bool hasError() => $_has(2);
  @$pb.TagNumber(3)
  void clearError() => clearField(3);
  @$pb.TagNumber(3)
  Error ensureError() => $_ensure(2);
}

class QueryListener extends $pb.GeneratedMessage {
  factory QueryListener() => create();
  QueryListener._() : super();
  factory QueryListener.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory QueryListener.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'QueryListener', package: const $pb.PackageName(_omitMessageNames ? '' : 'protocol'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  QueryListener clone() => QueryListener()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  QueryListener copyWith(void Function(QueryListener) updates) => super.copyWith((message) => updates(message as QueryListener)) as QueryListener;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static QueryListener create() => QueryListener._();
  QueryListener createEmptyInstance() => create();
  static $pb.PbList<QueryListener> createRepeated() => $pb.PbList<QueryListener>();
  @$core.pragma('dart2js:noInline')
  static QueryListener getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<QueryListener>(create);
  static QueryListener? _defaultInstance;
}

class ListenerList extends $pb.GeneratedMessage {
  factory ListenerList({
    $core.Iterable<$core.int>? listenerList,
  }) {
    final $result = create();
    if (listenerList != null) {
      $result.listenerList.addAll(listenerList);
    }
    return $result;
  }
  ListenerList._() : super();
  factory ListenerList.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ListenerList.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ListenerList', package: const $pb.PackageName(_omitMessageNames ? '' : 'protocol'), createEmptyInstance: create)
    ..p<$core.int>(1, _omitFieldNames ? '' : 'listenerList', $pb.PbFieldType.KU3, protoName: 'listenerList')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ListenerList clone() => ListenerList()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ListenerList copyWith(void Function(ListenerList) updates) => super.copyWith((message) => updates(message as ListenerList)) as ListenerList;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ListenerList create() => ListenerList._();
  ListenerList createEmptyInstance() => create();
  static $pb.PbList<ListenerList> createRepeated() => $pb.PbList<ListenerList>();
  @$core.pragma('dart2js:noInline')
  static ListenerList getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ListenerList>(create);
  static ListenerList? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get listenerList => $_getList(0);
}

class NewChannel extends $pb.GeneratedMessage {
  factory NewChannel({
    $core.int? listenerId,
  }) {
    final $result = create();
    if (listenerId != null) {
      $result.listenerId = listenerId;
    }
    return $result;
  }
  NewChannel._() : super();
  factory NewChannel.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory NewChannel.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'NewChannel', package: const $pb.PackageName(_omitMessageNames ? '' : 'protocol'), createEmptyInstance: create)
    ..a<$core.int>(1, _omitFieldNames ? '' : 'listenerId', $pb.PbFieldType.OU3, protoName: 'listenerId')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  NewChannel clone() => NewChannel()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  NewChannel copyWith(void Function(NewChannel) updates) => super.copyWith((message) => updates(message as NewChannel)) as NewChannel;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static NewChannel create() => NewChannel._();
  NewChannel createEmptyInstance() => create();
  static $pb.PbList<NewChannel> createRepeated() => $pb.PbList<NewChannel>();
  @$core.pragma('dart2js:noInline')
  static NewChannel getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<NewChannel>(create);
  static NewChannel? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get listenerId => $_getIZ(0);
  @$pb.TagNumber(1)
  set listenerId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasListenerId() => $_has(0);
  @$pb.TagNumber(1)
  void clearListenerId() => clearField(1);
}

class ChannelId extends $pb.GeneratedMessage {
  factory ChannelId({
    $core.int? channelId,
  }) {
    final $result = create();
    if (channelId != null) {
      $result.channelId = channelId;
    }
    return $result;
  }
  ChannelId._() : super();
  factory ChannelId.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelId.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ChannelId', package: const $pb.PackageName(_omitMessageNames ? '' : 'protocol'), createEmptyInstance: create)
    ..a<$core.int>(2, _omitFieldNames ? '' : 'channelId', $pb.PbFieldType.OU3, protoName: 'channelId')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChannelId clone() => ChannelId()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChannelId copyWith(void Function(ChannelId) updates) => super.copyWith((message) => updates(message as ChannelId)) as ChannelId;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ChannelId create() => ChannelId._();
  ChannelId createEmptyInstance() => create();
  static $pb.PbList<ChannelId> createRepeated() => $pb.PbList<ChannelId>();
  @$core.pragma('dart2js:noInline')
  static ChannelId getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelId>(create);
  static ChannelId? _defaultInstance;

  @$pb.TagNumber(2)
  $core.int get channelId => $_getIZ(0);
  @$pb.TagNumber(2)
  set channelId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(2)
  $core.bool hasChannelId() => $_has(0);
  @$pb.TagNumber(2)
  void clearChannelId() => clearField(2);
}

class ChannelClose extends $pb.GeneratedMessage {
  factory ChannelClose({
    $core.int? channelId,
  }) {
    final $result = create();
    if (channelId != null) {
      $result.channelId = channelId;
    }
    return $result;
  }
  ChannelClose._() : super();
  factory ChannelClose.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelClose.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ChannelClose', package: const $pb.PackageName(_omitMessageNames ? '' : 'protocol'), createEmptyInstance: create)
    ..a<$core.int>(1, _omitFieldNames ? '' : 'channelId', $pb.PbFieldType.OU3, protoName: 'channelId')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChannelClose clone() => ChannelClose()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChannelClose copyWith(void Function(ChannelClose) updates) => super.copyWith((message) => updates(message as ChannelClose)) as ChannelClose;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ChannelClose create() => ChannelClose._();
  ChannelClose createEmptyInstance() => create();
  static $pb.PbList<ChannelClose> createRepeated() => $pb.PbList<ChannelClose>();
  @$core.pragma('dart2js:noInline')
  static ChannelClose getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelClose>(create);
  static ChannelClose? _defaultInstance;

  @$pb.TagNumber(1)
  $core.int get channelId => $_getIZ(0);
  @$pb.TagNumber(1)
  set channelId($core.int v) { $_setUnsignedInt32(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasChannelId() => $_has(0);
  @$pb.TagNumber(1)
  void clearChannelId() => clearField(1);
}

class ChannelData extends $pb.GeneratedMessage {
  factory ChannelData({
    $core.List<$core.int>? data,
    $core.int? channelId,
  }) {
    final $result = create();
    if (data != null) {
      $result.data = data;
    }
    if (channelId != null) {
      $result.channelId = channelId;
    }
    return $result;
  }
  ChannelData._() : super();
  factory ChannelData.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory ChannelData.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'ChannelData', package: const $pb.PackageName(_omitMessageNames ? '' : 'protocol'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'data', $pb.PbFieldType.OY)
    ..a<$core.int>(2, _omitFieldNames ? '' : 'channelId', $pb.PbFieldType.OU3, protoName: 'channelId')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  ChannelData clone() => ChannelData()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  ChannelData copyWith(void Function(ChannelData) updates) => super.copyWith((message) => updates(message as ChannelData)) as ChannelData;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ChannelData create() => ChannelData._();
  ChannelData createEmptyInstance() => create();
  static $pb.PbList<ChannelData> createRepeated() => $pb.PbList<ChannelData>();
  @$core.pragma('dart2js:noInline')
  static ChannelData getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChannelData>(create);
  static ChannelData? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get data => $_getN(0);
  @$pb.TagNumber(1)
  set data($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasData() => $_has(0);
  @$pb.TagNumber(1)
  void clearData() => clearField(1);

  @$pb.TagNumber(2)
  $core.int get channelId => $_getIZ(1);
  @$pb.TagNumber(2)
  set channelId($core.int v) { $_setUnsignedInt32(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasChannelId() => $_has(1);
  @$pb.TagNumber(2)
  void clearChannelId() => clearField(2);
}

enum Ok_Msg {
  channelId, 
  notSet
}

class Ok extends $pb.GeneratedMessage {
  factory Ok({
    ChannelId? channelId,
  }) {
    final $result = create();
    if (channelId != null) {
      $result.channelId = channelId;
    }
    return $result;
  }
  Ok._() : super();
  factory Ok.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Ok.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static const $core.Map<$core.int, Ok_Msg> _Ok_MsgByTag = {
    1 : Ok_Msg.channelId,
    0 : Ok_Msg.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Ok', package: const $pb.PackageName(_omitMessageNames ? '' : 'protocol'), createEmptyInstance: create)
    ..oo(0, [1])
    ..aOM<ChannelId>(1, _omitFieldNames ? '' : 'channelId', protoName: 'channelId', subBuilder: ChannelId.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Ok clone() => Ok()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Ok copyWith(void Function(Ok) updates) => super.copyWith((message) => updates(message as Ok)) as Ok;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Ok create() => Ok._();
  Ok createEmptyInstance() => create();
  static $pb.PbList<Ok> createRepeated() => $pb.PbList<Ok>();
  @$core.pragma('dart2js:noInline')
  static Ok getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Ok>(create);
  static Ok? _defaultInstance;

  Ok_Msg whichMsg() => _Ok_MsgByTag[$_whichOneof(0)]!;
  void clearMsg() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  ChannelId get channelId => $_getN(0);
  @$pb.TagNumber(1)
  set channelId(ChannelId v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasChannelId() => $_has(0);
  @$pb.TagNumber(1)
  void clearChannelId() => clearField(1);
  @$pb.TagNumber(1)
  ChannelId ensureChannelId() => $_ensure(0);
}

class Error extends $pb.GeneratedMessage {
  factory Error({
    Error_Code? code,
    $core.String? msg,
  }) {
    final $result = create();
    if (code != null) {
      $result.code = code;
    }
    if (msg != null) {
      $result.msg = msg;
    }
    return $result;
  }
  Error._() : super();
  factory Error.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Error.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Error', package: const $pb.PackageName(_omitMessageNames ? '' : 'protocol'), createEmptyInstance: create)
    ..e<Error_Code>(1, _omitFieldNames ? '' : 'code', $pb.PbFieldType.OE, defaultOrMaker: Error_Code.Unbind, valueOf: Error_Code.valueOf, enumValues: Error_Code.values)
    ..aOS(2, _omitFieldNames ? '' : 'msg')
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Error clone() => Error()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Error copyWith(void Function(Error) updates) => super.copyWith((message) => updates(message as Error)) as Error;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Error create() => Error._();
  Error createEmptyInstance() => create();
  static $pb.PbList<Error> createRepeated() => $pb.PbList<Error>();
  @$core.pragma('dart2js:noInline')
  static Error getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Error>(create);
  static Error? _defaultInstance;

  @$pb.TagNumber(1)
  Error_Code get code => $_getN(0);
  @$pb.TagNumber(1)
  set code(Error_Code v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasCode() => $_has(0);
  @$pb.TagNumber(1)
  void clearCode() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get msg => $_getSZ(1);
  @$pb.TagNumber(2)
  set msg($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasMsg() => $_has(1);
  @$pb.TagNumber(2)
  void clearMsg() => clearField(2);
}

enum Message_Msg {
  request, 
  response, 
  notSet
}

class Message extends $pb.GeneratedMessage {
  factory Message({
    Request? request,
    Response? response,
  }) {
    final $result = create();
    if (request != null) {
      $result.request = request;
    }
    if (response != null) {
      $result.response = response;
    }
    return $result;
  }
  Message._() : super();
  factory Message.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Message.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static const $core.Map<$core.int, Message_Msg> _Message_MsgByTag = {
    1 : Message_Msg.request,
    2 : Message_Msg.response,
    0 : Message_Msg.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'Message', package: const $pb.PackageName(_omitMessageNames ? '' : 'protocol'), createEmptyInstance: create)
    ..oo(0, [1, 2])
    ..aOM<Request>(1, _omitFieldNames ? '' : 'request', subBuilder: Request.create)
    ..aOM<Response>(2, _omitFieldNames ? '' : 'response', subBuilder: Response.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Message clone() => Message()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Message copyWith(void Function(Message) updates) => super.copyWith((message) => updates(message as Message)) as Message;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Message create() => Message._();
  Message createEmptyInstance() => create();
  static $pb.PbList<Message> createRepeated() => $pb.PbList<Message>();
  @$core.pragma('dart2js:noInline')
  static Message getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Message>(create);
  static Message? _defaultInstance;

  Message_Msg whichMsg() => _Message_MsgByTag[$_whichOneof(0)]!;
  void clearMsg() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  Request get request => $_getN(0);
  @$pb.TagNumber(1)
  set request(Request v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasRequest() => $_has(0);
  @$pb.TagNumber(1)
  void clearRequest() => clearField(1);
  @$pb.TagNumber(1)
  Request ensureRequest() => $_ensure(0);

  @$pb.TagNumber(2)
  Response get response => $_getN(1);
  @$pb.TagNumber(2)
  set response(Response v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasResponse() => $_has(1);
  @$pb.TagNumber(2)
  void clearResponse() => clearField(2);
  @$pb.TagNumber(2)
  Response ensureResponse() => $_ensure(1);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
