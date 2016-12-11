//! C interface definition.

use std::ffi::CStr;
use std::ops::Deref;

use libc::*;

use parser;
use queries::*;


#[no_mangle]
unsafe extern "C" fn parse(text: *const c_char) -> Option<Queries> {
    parser::parse(CStr::from_ptr(text).to_string_lossy().deref()).ok()
}


#[no_mangle]
extern "C" fn query(queries: Option<Queries>,
                    name: *const c_char)
                    -> Option<Query> {
    unimplemented!()
}


#[no_mangle]
extern "C" fn render(query: Option<Query>) -> *const c_char {
    let s: &str = "x";
    s.as_ptr() as *const i8
}


#[no_mangle]
extern "C" fn name(query: Option<Query>) -> *const c_char { unimplemented!() }


#[no_mangle]
extern "C" fn text(query: Option<Query>) -> *const c_char { unimplemented!() }


#[no_mangle]
extern "C" fn readonly(query: Option<Query>) -> bool { unimplemented!() }
