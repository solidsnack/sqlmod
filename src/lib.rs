#![recursion_limit = "1024"]               // `error_chain!` can recurse deeply

#[macro_use]
extern crate error_chain;
extern crate libc;
extern crate linked_hash_map;

pub mod c;
pub use c::*;
pub mod errors;
pub mod lines;
pub mod parser;
#[cfg_attr(rustfmt, rustfmt_skip)]
pub mod peg;
pub mod queries;
pub mod query;
mod tests;
