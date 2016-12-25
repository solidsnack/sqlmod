#![cfg(test)]

use parser;

mod attributes;
mod long_example;
mod single_query;


#[test]
fn it_works() {
    assert!(parser::parse("").is_ok());
    assert!(parser::parse("--@ one\nSELECT 1;").is_ok());
}
