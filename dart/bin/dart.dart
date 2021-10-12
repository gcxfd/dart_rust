import 'dart:ffi';
import 'dart:typed_data';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import './blake3.dart';
import 'dart:convert';

void main(List<String> arguments) async {
  final dylibPath = "../rust.so";
  final dylib = DynamicLibrary.open(dylibPath);
  final so = DartRust(dylib);

  final data = Uint8List.fromList("1".codeUnits);

  print(base64.encode(await so.blake3Hash(data:data)));
}
