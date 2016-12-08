#![cfg(test)]

use parser;


const NOW: &'static str = "SELECT now();";
const CMT: &'static str = "--- The current time.";


#[test]
fn it_works() {
    let sql = include_str!("one.sql");
    let parsed = parser::parse(sql).unwrap();
    assert!(parsed.queries["now"].name == "now");
    assert!(parsed.queries["now"].text == NOW);
}


#[test]
fn whitespace() {
    let sql = include_str!("one-with-leading-whitespace.sql");
    let parsed = parser::parse(sql).unwrap();
    assert!(parsed.queries["now"].name == "now");
    assert!(parsed.queries["now"].text == NOW);

    let sql = include_str!("one-with-trailing-whitespace.sql");
    let parsed = parser::parse(sql).unwrap();
    assert!(parsed.queries["now"].name == "now");
    assert!(parsed.queries["now"].text == NOW)
}


#[test]
fn simple_leading_comment() {
    let sql = include_str!("one-with-leading-comments.sql");
    let parsed = parser::parse(sql).unwrap();
    assert!(parsed.queries["now"].name == "now");
    assert!(parsed.queries["now"].text == NOW);
}


#[test]
fn simple_trailing_comment() {
    let sql = include_str!("one-with-trailing-comment.sql");
    let parsed = parser::parse(sql).unwrap();
    assert!(parsed.queries["now"].name == "now");
    assert!(parsed.queries["now"].text == format!("{}\n{}", NOW, CMT));
}



#[test]
fn simple_query_comment() {
    let sql = include_str!("one-with-middle-comment.sql");
    let parsed = parser::parse(sql).unwrap();
    assert!(parsed.queries["now"].name == "now");
    assert!(parsed.queries["now"].text == format!("{}\n{}", CMT, NOW));
}
