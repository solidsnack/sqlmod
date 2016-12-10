use std::collections::HashSet;

use parser;


#[test]
fn it_works() {
    let sample = include_str!("pg_examples.sql");
    let parsed = parser::parse(sample);
    let keys: HashSet<&str> = vec!["as_interval",
                                   "as_timestamp",
                                   "maybe_create_user",
                                   "now",
                                   "load_archive"]
                                  .into_iter()
                                  .collect();

    assert!(parsed.is_ok());

    if let Ok(ref q) = parsed {
        let found: HashSet<&str> =
            q.queries.keys().map(|s| s.as_str()).collect();
        for q in q.queries.values() {
            println!("{:#?}", q);   // Visible with `cargo test -- --nocapture`
        }
        assert!(found == keys);
    }
}
