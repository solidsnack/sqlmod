use std::borrow::Borrow;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::str::from_utf8_unchecked;
use std::ptr;

use libc::c_char;


/// Stores textual, read-only data in a way that is friendly to C as well as
/// Rust. The underlying strings can be passed directly to C, without copies,
/// since they are null-terminated; but this detail does not affect the Rust
/// interface.
pub struct StringMap {
    keys: Vec<*const c_char>, // To preserve insertion order
    data: HashMap<CString, CString>,
}

impl StringMap {
    pub fn new<'input, P, S, T>(tuples: T) -> StringMap
        where P: Borrow<(S, S)>,
              S: Borrow<str>,
              T: IntoIterator<Item = P>
    {
        let mut keys: Vec<*const c_char> = Vec::default();
        let mut data: HashMap<CString, CString> = HashMap::default();
        for pair in tuples {
            let &(ref k, ref v) = pair.borrow();
            if let Ok(k) = CString::new(k.borrow()) {
                if let Ok(v) = CString::new(v.borrow()) {
                    keys.push(k.as_ptr() as *const c_char);
                    data.insert(k, v);
                }
            }
        }
        keys.push(ptr::null());
        StringMap { keys: keys, data: data }
    }

    pub fn names(&self) -> Vec<&str> {
        self.keys[0..(self.keys.len() - 1)]
            .iter()
            .map(|&p| unsafe { CStr::from_ptr(p) })
            .map(|s| unsafe { from_utf8_unchecked(s.to_bytes()) })
            .collect()
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        if let Ok(ref k) = CString::new(key) {
            self.data
                .get(k)
                .map(|s| unsafe { from_utf8_unchecked(s.as_bytes()) })
        } else {
            None
        }
    }

    pub fn len(&self) -> usize { self.data.len() }
}


pub mod c {
    use std::ffi::CStr;
    use std::ptr;

    use libc::*;

    use stringmap::StringMap;


    pub unsafe extern "C" fn new(count: size_t,
                                 keys: *const *const c_char,
                                 values: *const *const c_char)
                                 -> *mut StringMap {
        let n = count as isize;
        let mut data = Vec::default();

        if keys.is_null() || values.is_null() {
            return ptr::null_mut();
        }

        for (k, v) in (0..n).map(|i| (*keys.offset(i), *values.offset(i))) {
            if k.is_null() || v.is_null() {
                return ptr::null_mut();
            }
            data.push((CStr::from_ptr(k).to_string_lossy(),
                       CStr::from_ptr(v).to_string_lossy()));
        }

        Box::into_raw(Box::new(StringMap::new(&data)))
    }

    pub unsafe extern "C" fn names(m: *const StringMap)
                                   -> *const *const c_char {
        m.as_ref().map(|this| this.keys.as_ptr()).unwrap_or(ptr::null())
    }

    pub unsafe extern "C" fn get(m: *const StringMap,
                                 key: *const c_char)
                                 -> *const c_char {
        if let Some(this) = m.as_ref() {
            let k = CStr::from_ptr(key).to_string_lossy();
            this.get(&k)
                .map(|s| s.as_ptr() as *const c_char)
                .unwrap_or(ptr::null())
        } else {
            ptr::null()
        }
    }

    pub unsafe extern "C" fn len(m: *const StringMap) -> size_t {
        m.as_ref().map(|this| this.len()).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use libc::c_char;

    use stringmap::c;
    use stringmap::StringMap;


    #[test]
    fn it_works() {
        let data = vec![("a", "A"), ("b", "B")];
        let m = StringMap::new(&data);
        assert!(m.len() == 2);
        assert!(m.names() == vec!["a", "b"]);
        assert!(m.get("a") == Some("A"));
        assert!(m.get("b") == Some("B"));
    }


    #[test]
    fn extern_interface() {
        let data = vec![("a", "A"), ("b", "B")];
        let keys: Vec<*const c_char> =
            data.iter().map(|&(s, _)| s.as_ptr() as *const c_char).collect();
        let vals: Vec<*const c_char> =
            data.iter().map(|&(_, s)| s.as_ptr() as *const c_char).collect();
        unsafe {
            let m = c::new(data.len(), keys.as_ptr(), vals.as_ptr());
            assert!(c::len(m) == 2);
            assert!(!c::names(m).is_null());
        }
    }
}
