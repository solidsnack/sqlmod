use parser;


const ANNO: &'static str = "--@ now";
const ANNO_RO: &'static str = "--@ now ro";
const QUERY: &'static str = "SELECT now();";
const CMT: &'static str = "--- The current time.";


#[test]
fn just_query() {
    let samples = vec![
        vec![ANNO, QUERY].join("\n"),
        vec![ANNO_RO, QUERY].join("\n"),
    ];
    for sample in samples {
        let parsed = parser::parse(&sample).unwrap();
        assert!(parsed.queries["now"].name == "now");
        assert!(parsed.queries["now"].text == QUERY);
    }
}


#[test]
fn whitespace() {
    let samples = vec![
        vec![ANNO, QUERY].join("\n"),
        vec!["", ANNO, QUERY].join("\n"),
        vec!["  ", "\t", ANNO, QUERY].join("\n"),
        vec![ANNO, QUERY].join("\n"),
        vec![ANNO, QUERY, ""].join("\n"),
        vec![ANNO, QUERY, "  "].join("\n"),
        vec![ANNO, QUERY, "  ", "\t"].join("\n"),
    ];
    for sample in samples {
        let parsed = parser::parse(&sample).unwrap();
        assert!(parsed.queries["now"].name == "now");
        assert!(parsed.queries["now"].text == QUERY);
    }
}


#[test]
fn above_comment() {
    let sample = vec![CMT, ANNO, QUERY].join("\n");
    let parsed = parser::parse(&sample).unwrap();
    assert!(parsed.queries["now"].name == "now");
    assert!(parsed.queries["now"].text == QUERY);
}


#[test]
fn trailing_comment() {
    let samples = vec![
        vec![ANNO, QUERY, CMT].join("\n"),
        vec![ANNO, QUERY, "", CMT].join("\n"),
        vec![ANNO, QUERY, "", "", CMT].join("\n"),
        vec![ANNO, QUERY, "", "\t", CMT].join("\n"),
        vec![ANNO, QUERY, "   ", "\t", CMT].join("\n"),
    ];
    for sample in samples {
        let parsed = parser::parse(&sample).unwrap();
        assert!(parsed.queries["now"].name == "now");
        assert!(parsed.queries["now"].text == QUERY);
    }
}


#[test]
fn query_comment() {
    let sample = vec![ANNO, CMT, QUERY].join("\n");
    let parsed = parser::parse(&sample).unwrap();
    assert!(parsed.queries["now"].name == "now");
    assert!(parsed.queries["now"].text == format!("{}\n{}", CMT, QUERY));
}
