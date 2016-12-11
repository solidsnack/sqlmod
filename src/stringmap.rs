use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::ptr;
use std::str;

use libc::*;


/// Stores textual, read-only data in a way that is friendly C as well as Rust.
/// The underlying strings can be passed directly to C, without copies, since
/// they are null-terminated; but this detail does not affect the Rust
/// interface.
pub struct StringMap {
    data: HashMap<CString, CString>,
    keys: Vec<CString>,
    ptrs: Vec<*const c_char>,
}

impl StringMap {
    pub fn names(&self) -> Vec<&str> {
        self.keys
            .iter()
            .map(|s| unsafe { str::from_utf8_unchecked(s.as_bytes()) })
            .collect()
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        if let Ok(ref k) = CString::new(key) {
            self.data
                .get(k)
                .map(|s| unsafe { str::from_utf8_unchecked(s.as_bytes()) })
        } else {
            None
        }
    }

    pub fn len(&self) -> usize { self.keys.len() }
}


pub unsafe extern "C" fn names(m: Option<StringMap>) -> *const *const c_char {
    m.map(|this| this.ptrs.as_ptr()).unwrap_or(ptr::null())
}

pub unsafe extern "C" fn get(m: Option<StringMap>,
                             key: *const c_char)
                             -> *const c_char {
    if let Some(this) = m {
        let k = CStr::from_ptr(key).to_string_lossy();
        this.get(&k)
            .map(|s| s.as_ptr() as *const c_char)
            .unwrap_or(ptr::null())
    } else {
        ptr::null()
    }
}

pub unsafe extern "C" fn len(m: Option<StringMap>) -> size_t {
    m.map(|this| this.len()).unwrap_or(0)
}
