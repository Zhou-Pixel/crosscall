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

class Error_Code extends $pb.ProtobufEnum {
  static const Error_Code Unbind = Error_Code._(0, _omitEnumNames ? '' : 'Unbind');
  static const Error_Code ChannelNotFound = Error_Code._(1, _omitEnumNames ? '' : 'ChannelNotFound');

  static const $core.List<Error_Code> values = <Error_Code> [
    Unbind,
    ChannelNotFound,
  ];

  static final $core.Map<$core.int, Error_Code> _byValue = $pb.ProtobufEnum.initByValue(values);
  static Error_Code? valueOf($core.int value) => _byValue[value];

  const Error_Code._($core.int v, $core.String n) : super(v, n);
}


const _omitEnumNames = $core.bool.fromEnvironment('protobuf.omit_enum_names');
