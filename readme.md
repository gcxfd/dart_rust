
https://github.com/fzyzcjy/flutter_rust_bridge

安装
安装依赖cbindgen：（cargo install cbindgen )
安装依赖项ffigen： dart pub global activate ffigen，并安装 LLVM。
安装此代码生成器二进制文件cargo install flutter_rust_bridge_codegen。
添加flutter_rust_bridge = "1.0"（1.0应该是最新版本）到 Rust 的Cargo.toml.
加入flutter_rust_bridge: ^1.0（同上，应该是最新的版本）扑/达特pubspec.yaml下的部分dependencies。


