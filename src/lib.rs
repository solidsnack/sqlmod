#![recursion_limit = "1024"]               // `error_chain!` can recurse deeply

extern crate libc;
#[macro_use]
extern crate error_chain;


pub mod c;
pub mod errors;
pub mod lines;
pub mod parser;
#[cfg_attr(rustfmt, rustfmt_skip)]
pub mod peg;
pub mod queries;
mod tests;
