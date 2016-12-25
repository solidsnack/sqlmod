#![cfg(test)]

pub mod attributes;
pub mod long_example;
pub mod single_query;

use parser;


#[test]
fn it_works() {
    assert!(parser::parse("").is_ok());
    assert!(parser::parse("--@ one\nSELECT 1;").is_ok());
}
