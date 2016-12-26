//! C interface definition.
use std;

use libc;
use parser;
use queries::*;
use query::*;
mod strhandle;
pub use self::strhandle::StrHandle;


// Interface for Queries object ///////////////////////////////////////////////

#[no_mangle]
pub unsafe extern "C" fn qselect_parse(text: StrHandle) -> *mut Queries {
    // NB: Gives the Queries object to the outside -- caller must free.
    to_ptr(parser::parse(&text.to_str_lossy() as &str).ok())
}

#[no_mangle]
pub unsafe extern "C" fn qselect_get(queries: *const Queries,
                                     name: StrHandle)
                                     -> *const Query {
    let key = name.to_str_lossy();
    if let Some(query) = queries.as_ref().and_then(|q| q.get(&key)) {
        query as *const _                          // Share memory with outside
    } else {
        std::ptr::null()
    }
}

#[no_mangle]
pub unsafe extern "C" fn qselect_queries(queries: *const Queries)
                                         -> *const *const Query {
    unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn qselect_len(queries: *const Queries) -> libc::size_t {
    if let Some(queries) = queries.as_ref() {
        queries.len() as libc::size_t
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn qselect_free(queries: *mut Queries) { free(queries); }


// Interface for Query object /////////////////////////////////////////////////

#[no_mangle]
pub unsafe extern "C" fn qselect_name(query: *const Query) -> StrHandle {
    if let Some(query) = query.as_ref() {
        StrHandle::new(&query.signature.name)
    } else {
        StrHandle::default()
    }
}

#[no_mangle]
pub unsafe extern "C" fn qselect_text(query: *const Query) -> StrHandle {
    if let Some(query) = query.as_ref() {
        StrHandle::new(&query.text)
    } else {
        StrHandle::default()
    }
}

#[no_mangle]
pub unsafe extern "C" fn qselect_attributes(query: *const Query) -> StrHandle {
    unimplemented!()
}


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
