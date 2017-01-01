use std;
use std::borrow::Cow;
use std::ffi::CStr;

use libc;

use errors::*;
use parser;
use queries::*;
use query::*;


// C Types ////////////////////////////////////////////////////////////////////

/// A C-friendly representation of a counted string. This is useful to:
///
/// * Share a heap-allocated `String` with C.
///
/// * Allow C to share a counted string with Rust.
///
/// This struct is small enough to be efficiently passed on the stack, so it's
/// easy to create APIs where neither caller nor callee need to free it.
///
/// This is intended to be used in cases where the caller is going to copy
/// pretty much immediately upon receiving the string, as for example happens
/// when turning a Rust `String` into a Python `str` or Ruby String` -- both
/// languages have their own counted string representation.
///
/// Note that use-after-free is a very real possibility with this data
/// structure -- it doesn't marshal string data from Rust but rather shares it.
/// For heap-allocated strings that are part of a larger structure this is a
/// reasonable thing to do; but for `&str` declarations local to a function it
/// is rather pointless.
///
/// http://stackoverflow.com/a/30272872/
///
/// > ...the Linux x86-64 ABI specifies that returning a struct with two scalar
/// > (e.g. pointers, or long) values is done thru registers (%rax & %rdx) so
/// > is very fast and efficient
#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub struct qselect_str_t {
    ptr: *const libc::c_char,
    len: libc::size_t,
}

#[allow(non_camel_case_types)]
type qselect_queries_t = Queries;

#[allow(non_camel_case_types)]
type qselect_query_t = Query;


// Interface for Queries object ///////////////////////////////////////////////

#[no_mangle]
pub unsafe extern "C" fn qselect_queries_parse(text: qselect_str_t)
                                               -> *mut qselect_queries_t {
    // NB: Gives the Queries object to the outside -- caller must free.
    to_ptr(parser::parse(&text.to_str_lossy() as &str).ok())
}

#[no_mangle]
pub unsafe extern "C" fn qselect_queries_get_query_by_name
    (queries: *const qselect_queries_t,
     name: qselect_str_t)
     -> *const qselect_query_t {
    let key = name.to_str_lossy();
    if let Some(query) = queries.as_ref().and_then(|q| q.get(&key)) {
        query as *const _                          // Share memory with outside
    } else {
        std::ptr::null()
    }
}

#[no_mangle]
pub unsafe extern "C" fn qselect_queries_get_query_by_index
    (queries: *const qselect_queries_t,
     index: libc::size_t)
     -> *const qselect_query_t {
    let index = index as usize;
    if let Some(queries) = queries.as_ref() {
        for (i, query) in queries.iter().enumerate() {
            if index == i {
                return query as *const _;
            }
        }
    }
    return std::ptr::null();
}

#[no_mangle]
pub unsafe extern "C" fn
    qselect_queries_num_queries(queries: *const qselect_queries_t)
                                -> libc::size_t {
    if let Some(queries) = queries.as_ref() {
        queries.len() as libc::size_t
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn
    qselect_queries_free(queries: *mut qselect_queries_t) {
    free(queries);
}


// Interface for Query object /////////////////////////////////////////////////

#[no_mangle]
pub unsafe extern "C" fn qselect_query_get_name(query: *const qselect_query_t)
                                                -> qselect_str_t {
    query.as_ref()
         .map(|query| qselect_str_t::new(&query.signature.name))
         .unwrap_or_default()
}

#[no_mangle]
pub unsafe extern "C" fn qselect_query_get_text(query: *const qselect_query_t)
                                      -> qselect_str_t {
    query.as_ref()
         .map(|query| qselect_str_t::new(&query.text))
         .unwrap_or_default()
}

#[no_mangle]
pub unsafe extern "C" fn
    qselect_query_num_attributes(query: *const qselect_query_t)
                                 -> libc::size_t {
    if let Some(query) = query.as_ref() {
        query.signature.attributes.len() as libc::size_t
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn qselect_query_get_attribute_by_index
    (query: *const qselect_query_t,
     index: libc::size_t)
     -> qselect_str_t {
    let index = index as usize;
    query.as_ref()
         .and_then(|query| query.signature.attributes.get(index))
         .map(|s| qselect_str_t::new(&s))
         .unwrap_or_default()
}


// Utilties for memory management /////////////////////////////////////////////

unsafe fn to_ptr<T>(maybe: Option<T>) -> *mut T {
    if let Some(thing) = maybe {
        Box::into_raw(Box::new(thing))
    } else {
        std::ptr::null_mut()
    }
}

unsafe fn free<T>(ptr: *mut T) {
    if !ptr.is_null() {
        Box::from_raw(ptr);   // Obtain ownership, then go out of scope to drop
    }
}


// String utilities ///////////////////////////////////////////////////////////

#[no_mangle]
pub unsafe extern "C" fn qselect_str(ptr: *const libc::c_char,
                                     len: libc::size_t)
                                     -> qselect_str_t {
    qselect_str_t { ptr: ptr, len: len }
}

impl qselect_str_t {
    pub fn new(s: &str) -> qselect_str_t {
        qselect_str_t {
            ptr: s.as_ptr() as *const libc::c_char,
            len: s.len(),
        }
    }

    pub fn is_null(&self) -> bool { self.ptr.is_null() }

    pub unsafe fn from_cstr(s: *const libc::c_char) -> Result<qselect_str_t> {
        let n = try!(CStr::from_ptr(s)
                    .to_str()
                    .chain_err(|| "UTF8 error in foreign string."))
                    .len();
        Ok(qselect_str_t { ptr: s, len: n })
    }

    pub unsafe fn to_slice(&self) -> &[u8] {
        std::slice::from_raw_parts(self.ptr as *const u8, self.len)
    }

    pub unsafe fn to_str(&self) -> Result<&str> {
        std::str::from_utf8(self.to_slice())
            .chain_err(|| "UTF8 error in foreign string.")
    }

    pub unsafe fn to_str_lossy(&self) -> Cow<str> {
        std::string::String::from_utf8_lossy(self.to_slice())
    }
}

impl Default for qselect_str_t {
    fn default() -> qselect_str_t {
        qselect_str_t { ptr: std::ptr::null(), len: 0 }
    }
}
