//! Parsed results.
use std::collections::BTreeMap;

use lines::*;


#[derive(Debug)]
pub struct Queries {
    pub text: String,
    pub info: String,
    pub queries: BTreeMap<String, Query>,
    pub warnings: BTreeMap<usize, String>,
}


#[derive(Debug)]
pub struct Query {
    pub signature: Signature,
    pub original: String,
    pub text: String,
    pub line: usize,
}

impl Query {
    pub fn render(&self) -> String {
        if self.readonly() {
            format!("--@ {} ro\n{}", self.name(), self.text)
        } else {
            format!("--@ {}\n{}", self.name(), self.text)
        }
    }

    pub fn name(&self) -> String { self.signature.name.clone() }

    pub fn readonly(&self) -> bool { self.signature.ro }

    pub fn original(&self) -> String { self.original.clone() }

    pub fn text(&self) -> String { self.text.clone() }

    pub fn line(&self) -> usize { self.line.clone() }
}
