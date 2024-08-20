#!/bin/bash

protoc transport/protocol.proto --prost_out=./rust/src/ --proto_path=transport
protoc transport/protocol.proto --dart_out=./dart/lib/src/protocol --proto_path=transport