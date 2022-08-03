use std::{ffi::CString, os::raw::c_char};

use openfga_dsl_parser::*;

static mut DOC: String = String::new();

#[no_mangle]
pub fn dsl_to_json(offset: *mut u8, len: usize) -> *mut c_char {
    let input_buf = unsafe {
        let buf: &mut [u8] = core::slice::from_raw_parts_mut(offset, len);
        buf
    };
    let mut parser = Parser::new(std::str::from_utf8(input_buf).unwrap());
    let doc = parser.parse_document().unwrap();

    let json = json::JsonTransformer::new(&doc).serialize();
    unsafe {
        DOC = json.clone();
    }
    CString::new(json).unwrap().into_raw()
}

#[no_mangle]
pub fn doc_len() -> usize {
    unsafe { DOC.len() }
}

#[no_mangle]
pub fn set_at(ptr: *mut i32, byte: i32, overwrite: u32) -> u8 {
    unsafe {
        let buf: &mut [i32] = core::slice::from_raw_parts_mut(ptr, 1);
        if buf[0] > 0 {
            if overwrite == 0 {
                return 0;
            }
        }
        buf[0] = byte;
    }
    return 1;
}
