#![allow(unused_variables)]

use anyhow::Result;
use flutter_rust_bridge::ZeroCopyBuffer;
use std::convert::TryInto;

pub fn blake3_hash(data: Vec<u8>) -> Result<Vec<u8>> {
  Ok(blake3::hash(&data).as_bytes().to_vec())
}

/*
use crate::const_u8::const_u8;
use crate::dart::Dart_NewFinalizableHandle_DL_Trampolined;
use crate::dart_sdk::Dart_Handle;
use blake3::Hasher;
use core::slice;
use safer_ffi::prelude::*;
use static_init::dynamic;
use std::mem::size_of;

#[derive_ReprC]
#[ReprC::opaque]
pub struct Blake3Hasher {
  pub h: Hasher,
}

#[ffi_export]

impl Blake3Hasher {
  pub fn new() -> Self {
    Self { h: Hasher::new() }
  }
}

#[ffi_export]
pub fn blake3_hasher_new(object: Dart_Handle) -> repr_c::Box<Blake3Hasher> {
  let peer = repr_c::Box::new(Blake3Hasher::new());
  unsafe {
    Dart_NewFinalizableHandle_DL_Trampolined(
      object,
      0 as *mut libc::c_void,
      8,
      Some(blake3_hasher_gced),
    )
  };
  peer
}

#[ffi_export]
pub fn blake3_hasher_update(hasher: &mut Blake3Hasher, data: *const u8, len: usize) {
  let v = unsafe { slice::from_raw_parts(data, len) };
  hasher.h.update(v);
}

#[ffi_export]
pub fn blake3_hasher_end(hasher: repr_c::Box<Blake3Hasher>) -> *const u8 {
  return const_u8(*hasher.h.finalize().as_bytes());
}

#[dynamic]
static SIZE: usize = size_of::<Blake3Hasher>();

pub extern "C" fn blake3_hasher_gced(
  isolate_callback_data: *mut libc::c_void,
  peer: *mut libc::c_void,
) {
  //unsafe { slice::from_raw_parts(peer, *SIZE) };
}

*/
