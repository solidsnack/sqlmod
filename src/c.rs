//! C interface definition.

use std::ffi::CStr;
use std::ops::Deref;

use libc::*;

use parser;
use queries::*;
use stringmap;
use stringmap::StringMap;


#[no_mangle]
pub unsafe extern "C" fn queries_parse(text: *const c_char)
                                       -> Option<Queries> {
    parser::parse(CStr::from_ptr(text).to_string_lossy().deref()).ok()
}


// #[no_mangle]
// extern "C" fn query(queries: Option<Queries>,
//                     name: *const c_char)
//                     -> Option<Query> {
//     unimplemented!()
// }


#[no_mangle]
pub unsafe extern "C" fn stringmap_new(count: size_t,
                                       keys: *const *const c_char,
                                       values: *const *const c_char)
                                       -> *mut StringMap {
    stringmap::c::new(count, keys, values)
}


#[no_mangle]
pub unsafe extern "C" fn stringmap_names(m: *const StringMap)
                                         -> *const *const c_char {
    stringmap::c::names(m)
}


#[no_mangle]
pub unsafe extern "C" fn stringmap_len(m: *const StringMap) -> size_t {
    stringmap::c::len(m)
}

#[no_mangle]
pub unsafe extern "C" fn stringmap_get(m: *const StringMap,
                                       key: *const c_char)
                                       -> *const c_char {
    stringmap::c::get(m, key)
}


#[no_mangle]
pub unsafe extern "C" fn stringmap_free(m: *mut StringMap) { drop(m); }


unsafe fn drop<T>(ptr: *mut T) {
    // Obtain ownership, then go out of scope to drop.
    if !ptr.is_null() {
        Box::from_raw(ptr);
    }
}
