#!/usr/bin/env bash

DIR=$(cd "$(dirname "$0")"; pwd)
set -ex
cd $DIR

flutter_rust_bridge_codegen --rust-input rust/src/blake3.rs --dart-output dart/bin/blake3.dart


mkdir -p so

cd rust

cargo build --release

mv target/release/libdart_rust.dylib ../rust.so

