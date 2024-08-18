//
//  Generated code. Do not modify.
//  source: proto/protocol.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use requestDescriptor instead')
const Request$json = {
  '1': 'Request',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 13, '10': 'id'},
    {'1': 'newChannel', '3': 4, '4': 1, '5': 11, '6': '.protocol.NewChannel', '9': 0, '10': 'newChannel'},
    {'1': 'channelClose', '3': 2, '4': 1, '5': 11, '6': '.protocol.ChannelClose', '9': 0, '10': 'channelClose'},
    {'1': 'channelData', '3': 3, '4': 1, '5': 11, '6': '.protocol.ChannelData', '9': 0, '10': 'channelData'},
  ],
  '8': [
    {'1': 'msg'},
  ],
};

/// Descriptor for `Request`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List requestDescriptor = $convert.base64Decode(
    'CgdSZXF1ZXN0Eg4KAmlkGAEgASgNUgJpZBI2CgpuZXdDaGFubmVsGAQgASgLMhQucHJvdG9jb2'
    'wuTmV3Q2hhbm5lbEgAUgpuZXdDaGFubmVsEjwKDGNoYW5uZWxDbG9zZRgCIAEoCzIWLnByb3Rv'
    'Y29sLkNoYW5uZWxDbG9zZUgAUgxjaGFubmVsQ2xvc2USOQoLY2hhbm5lbERhdGEYAyABKAsyFS'
    '5wcm90b2NvbC5DaGFubmVsRGF0YUgAUgtjaGFubmVsRGF0YUIFCgNtc2c=');

@$core.Deprecated('Use responseDescriptor instead')
const Response$json = {
  '1': 'Response',
  '2': [
    {'1': 'id', '3': 1, '4': 1, '5': 13, '10': 'id'},
    {'1': 'ok', '3': 2, '4': 1, '5': 11, '6': '.protocol.Ok', '9': 0, '10': 'ok'},
    {'1': 'error', '3': 3, '4': 1, '5': 11, '6': '.protocol.Error', '9': 0, '10': 'error'},
  ],
  '8': [
    {'1': 'msg'},
  ],
};

/// Descriptor for `Response`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List responseDescriptor = $convert.base64Decode(
    'CghSZXNwb25zZRIOCgJpZBgBIAEoDVICaWQSHgoCb2sYAiABKAsyDC5wcm90b2NvbC5Pa0gAUg'
    'JvaxInCgVlcnJvchgDIAEoCzIPLnByb3RvY29sLkVycm9ySABSBWVycm9yQgUKA21zZw==');

@$core.Deprecated('Use queryListenerDescriptor instead')
const QueryListener$json = {
  '1': 'QueryListener',
};

/// Descriptor for `QueryListener`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List queryListenerDescriptor = $convert.base64Decode(
    'Cg1RdWVyeUxpc3RlbmVy');

@$core.Deprecated('Use listenerListDescriptor instead')
const ListenerList$json = {
  '1': 'ListenerList',
  '2': [
    {'1': 'listenerList', '3': 1, '4': 3, '5': 13, '10': 'listenerList'},
  ],
};

/// Descriptor for `ListenerList`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List listenerListDescriptor = $convert.base64Decode(
    'CgxMaXN0ZW5lckxpc3QSIgoMbGlzdGVuZXJMaXN0GAEgAygNUgxsaXN0ZW5lckxpc3Q=');

@$core.Deprecated('Use newChannelDescriptor instead')
const NewChannel$json = {
  '1': 'NewChannel',
  '2': [
    {'1': 'listenerId', '3': 1, '4': 1, '5': 13, '10': 'listenerId'},
  ],
};

/// Descriptor for `NewChannel`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newChannelDescriptor = $convert.base64Decode(
    'CgpOZXdDaGFubmVsEh4KCmxpc3RlbmVySWQYASABKA1SCmxpc3RlbmVySWQ=');

@$core.Deprecated('Use channelIdDescriptor instead')
const ChannelId$json = {
  '1': 'ChannelId',
  '2': [
    {'1': 'channelId', '3': 2, '4': 1, '5': 13, '10': 'channelId'},
  ],
};

/// Descriptor for `ChannelId`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List channelIdDescriptor = $convert.base64Decode(
    'CglDaGFubmVsSWQSHAoJY2hhbm5lbElkGAIgASgNUgljaGFubmVsSWQ=');

@$core.Deprecated('Use channelCloseDescriptor instead')
const ChannelClose$json = {
  '1': 'ChannelClose',
  '2': [
    {'1': 'channelId', '3': 1, '4': 1, '5': 13, '10': 'channelId'},
  ],
};

/// Descriptor for `ChannelClose`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List channelCloseDescriptor = $convert.base64Decode(
    'CgxDaGFubmVsQ2xvc2USHAoJY2hhbm5lbElkGAEgASgNUgljaGFubmVsSWQ=');

@$core.Deprecated('Use channelDataDescriptor instead')
const ChannelData$json = {
  '1': 'ChannelData',
  '2': [
    {'1': 'data', '3': 1, '4': 1, '5': 12, '10': 'data'},
    {'1': 'channelId', '3': 2, '4': 1, '5': 13, '10': 'channelId'},
  ],
};

/// Descriptor for `ChannelData`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List channelDataDescriptor = $convert.base64Decode(
    'CgtDaGFubmVsRGF0YRISCgRkYXRhGAEgASgMUgRkYXRhEhwKCWNoYW5uZWxJZBgCIAEoDVIJY2'
    'hhbm5lbElk');

@$core.Deprecated('Use okDescriptor instead')
const Ok$json = {
  '1': 'Ok',
  '2': [
    {'1': 'channelId', '3': 1, '4': 1, '5': 11, '6': '.protocol.ChannelId', '9': 0, '10': 'channelId'},
  ],
  '8': [
    {'1': 'msg'},
  ],
};

/// Descriptor for `Ok`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List okDescriptor = $convert.base64Decode(
    'CgJPaxIzCgljaGFubmVsSWQYASABKAsyEy5wcm90b2NvbC5DaGFubmVsSWRIAFIJY2hhbm5lbE'
    'lkQgUKA21zZw==');

@$core.Deprecated('Use errorDescriptor instead')
const Error$json = {
  '1': 'Error',
  '2': [
    {'1': 'code', '3': 1, '4': 1, '5': 14, '6': '.protocol.Error.Code', '10': 'code'},
    {'1': 'msg', '3': 2, '4': 1, '5': 9, '10': 'msg'},
  ],
  '4': [Error_Code$json],
};

@$core.Deprecated('Use errorDescriptor instead')
const Error_Code$json = {
  '1': 'Code',
  '2': [
    {'1': 'Unbind', '2': 0},
  ],
};

/// Descriptor for `Error`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List errorDescriptor = $convert.base64Decode(
    'CgVFcnJvchIoCgRjb2RlGAEgASgOMhQucHJvdG9jb2wuRXJyb3IuQ29kZVIEY29kZRIQCgNtc2'
    'cYAiABKAlSA21zZyISCgRDb2RlEgoKBlVuYmluZBAA');

@$core.Deprecated('Use messageDescriptor instead')
const Message$json = {
  '1': 'Message',
  '2': [
    {'1': 'request', '3': 1, '4': 1, '5': 11, '6': '.protocol.Request', '9': 0, '10': 'request'},
    {'1': 'response', '3': 2, '4': 1, '5': 11, '6': '.protocol.Response', '9': 0, '10': 'response'},
  ],
  '8': [
    {'1': 'msg'},
  ],
};

/// Descriptor for `Message`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List messageDescriptor = $convert.base64Decode(
    'CgdNZXNzYWdlEi0KB3JlcXVlc3QYASABKAsyES5wcm90b2NvbC5SZXF1ZXN0SABSB3JlcXVlc3'
    'QSMAoIcmVzcG9uc2UYAiABKAsyEi5wcm90b2NvbC5SZXNwb25zZUgAUghyZXNwb25zZUIFCgNt'
    'c2c=');

