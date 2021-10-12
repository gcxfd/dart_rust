#!/usr/bin/env bash

DIR=$(cd "$(dirname "$0")"; pwd)
set -ex
cd $DIR

flutter_rust_bridge_codegen --rust-input rust/src/blake3.rs --dart-output dart/src/blake3.dart

