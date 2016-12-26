use std;
use std::borrow::Cow;
use std::ffi::CStr;

use libc;

use errors::*;


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
#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StrHandle {
    ptr: *const libc::c_char,
    len: libc::size_t,
}

// http://stackoverflow.com/a/30272872/
//
// > ...the Linux x86-64 ABI specifies that returning a struct with two scalar
// > (e.g. pointers, or long) values is done thru registers (%rax & %rdx) so is
// > very fast and efficient.

impl StrHandle {
    pub fn new(s: &str) -> StrHandle {
        StrHandle {
            ptr: s.as_ptr() as *const libc::c_char,
            len: s.len(),
        }
    }

    pub fn is_null(&self) -> bool { self.ptr.is_null() }

    pub unsafe fn from_cstr(s: *const libc::c_char) -> Result<StrHandle> {
        let n = try!(CStr::from_ptr(s)
                    .to_str()
                    .chain_err(|| "UTF8 error in foreign string."))
                    .len();
        Ok(StrHandle { ptr: s, len: n })
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

impl Default for StrHandle {
    fn default() -> StrHandle { StrHandle { ptr: std::ptr::null(), len: 0 } }
}
