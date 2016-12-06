//! C interface definition.

use libc;


#[no_mangle]
extern "C" fn parse(text: *const libc::c_char) -> Option<Queries> {
    unimplemented!()
}



