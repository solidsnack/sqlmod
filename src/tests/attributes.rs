use parser;


const SAMPLE: &'static str = include_str!("attributes.sql");


// Template for attribute test applied to each query.
macro_rules! test_attrs {
    ( $query:ident; [$( $attr:expr ),*] ) => {
        #[test]
        fn $query() {
            assert!(compare_attrs(stringify!($query), vec![$( $attr ),*]));
        }
    };
}


test_attrs!(now; []);

test_attrs!(now_ro; ["ro"]);

test_attrs!(now_etc; ["ro", "&c.", "->", "(1)"]);

test_attrs!(utc_parens; ["(t: datetime)", "ro"]);

test_attrs!(utc_parens2; ["(t timestamp with time zone)"]);

test_attrs!(utc_parens3; ["(t timestamp with time zone)", "(1)"]);


fn compare_attrs(query: &str, expected: Vec<&str>) -> bool {
    let queries = parser::parse(SAMPLE).unwrap();
    let ref q = queries[query];

    println!("For {:?}: {:?} & {:?}", q.name(), q.attributes(), expected);

    q.attributes() == expected
}
