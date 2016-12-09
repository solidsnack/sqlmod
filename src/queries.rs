//! Parsed results.
use std::collections::HashMap;

use lines::*;


#[derive(Debug)]
pub struct Queries {
    pub text: String,
    pub info: String,
    pub queries: HashMap<String, Query>,
    pub warnings: HashMap<usize, Line>,
}


#[derive(Debug)]
pub struct Query {
    pub full: String,
    pub text: String,
    pub name: String,
    pub ro: bool,
}
