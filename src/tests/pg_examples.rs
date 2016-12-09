use std::collections::HashSet;

use parser;


#[test]
fn just_query() {
    let sample = include_str!("pg_examples.sql");
    let parsed = parser::parse(sample);
    let keys: HashSet<&str> =
        vec!["as_interval", "as_timestamp", "maybe_create_user", "now"]
            .into_iter()
            .collect();

    assert!(parsed.is_ok());

    if let Ok(ref q) = parsed {
        let found: HashSet<&str> =
            q.queries.keys().map(|s| s.as_str()).collect();
        assert!(found == keys);
    }
}
