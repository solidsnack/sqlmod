//! Parsed results.

use lines::*;


pub struct Queries {
    pub text: String,
    pub queries: Vec<Query>,
    pub warnings: Vec<(usize, Line)>,
}


pub struct Query {
    pub position: (usize, usize),
    pub name: (usize, usize),
    pub ro: bool,
}
