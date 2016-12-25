use std::collections::HashSet;

use parser;


#[test]
fn it_works() {
    let sample = include_str!("long_example.sql");
    let parsed = parser::parse(sample);
    let keys: HashSet<&str> = vec!["as_interval",
                                   "as_timestamp",
                                   "maybe_create_user",
                                   "now",
                                   "load_archive"]
                                  .into_iter()
                                  .collect();

    assert!(parsed.is_ok());

    if let Ok(ref queries) = parsed {
        let alloc: Vec<_> = queries.keys().collect();
        let found: HashSet<&str> = alloc.iter().map(|s| s.as_str()).collect();
        for q in queries.iter() {
            // Visible with `cargo test -- --nocapture`.
            println!("{:#?}\n{}", q.signature, q.format());
        }
        assert!(found == keys);
    }
}
