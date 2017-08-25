extern crate libc;

use std::mem;
use std::ptr;

use libc::{c_void, size_t};

extern "C" {
    fn UJDecode(string: *const u8, length: size_t, func: *const c_void, state: *mut *mut c_void) -> *const c_void;
}

pub fn ujson4c_parse(string: &str) {
    unsafe {
        let mut state: *mut c_void = mem::uninitialized();
        UJDecode(string.as_bytes().as_ptr(), string.len() as size_t, ptr::null(), &mut state as *mut *mut c_void);
    }
}
