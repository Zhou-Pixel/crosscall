#!/bin/bash


cwd=$PWD


crossc=$cwd/crossc
rust=$cwd/rust
dart=$cwd/dart

cd $crossc
cargo publish

cd $rust
cargo publish

cd $dart
flutter pub publish
