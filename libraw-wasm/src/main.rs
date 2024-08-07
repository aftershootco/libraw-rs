extern crate alloc;

use std::fmt::write;

use alloc::{boxed::Box, vec::Vec};
use libraw_wasm::*;

fn main() {}

#[repr(C)]
#[derive(Debug)]
pub struct JsBytes {
    ptr: u32,
    len: u32,
    cap: u32,
}

impl JsBytes {
    pub fn new(mut bytes: Vec<u8>) -> *mut JsBytes {
        let ptr = bytes.as_mut_ptr() as u32;
        let len = bytes.len() as u32;
        let cap = bytes.capacity() as u32;
        core::mem::forget(bytes);
        let boxed = Box::new(JsBytes { ptr, len, cap });
        Box::into_raw(boxed)
    }
}

#[no_mangle]
pub unsafe extern "C" fn buffet(data: *mut u8, len: usize) -> *mut JsBytes {
    let buffer = unsafe { core::slice::from_raw_parts(data, len) };
    let mut proc = Processor::default();
    proc.open_buffer(buffer).unwrap();
    let res = proc.to_jpeg_no_rotation(80).unwrap();
    JsBytes::new(res)
}

#[no_mangle]
pub unsafe extern "C" fn vec_drop(raw: *mut JsBytes) {
    let veccy = Box::from_raw(raw);
    Vec::from_raw_parts(veccy.ptr as *mut u8, veccy.len as usize, veccy.cap as usize);
}
