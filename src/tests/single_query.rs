use parser;


const ANNO: &'static str = "--@ now";
const ANNO_RO: &'static str = "--@ now ro";
const QUERY: &'static str = "SELECT now();";
const CMT: &'static str = "--- The current time.";


#[test]
fn it_works() {
    let samples = vec![
        vec![ANNO, QUERY].join("\n"),
        vec![ANNO_RO, QUERY].join("\n"),
    ];
    for sample in samples {
        println!("sample: {:?}", sample);
        let parsed = parser::parse(&sample).unwrap();
        println!("parsed: {:?}", parsed);
        assert!(parsed["now"].name() == "now");
        assert!(parsed["now"].text() == QUERY);
    }
}


#[test]
fn whitespace() {
    let anno_with_with_trailing_whitespace = vec![ANNO, "  "].join("");
    let samples = vec![
        vec![ANNO, QUERY].join("\n"),
        vec!["", ANNO, QUERY].join("\n"),
        vec!["  ", "\t", ANNO, QUERY].join("\n"),
        vec![ANNO, QUERY].join("\n"),
        vec![ANNO, QUERY, ""].join("\n"),
        vec![ANNO, QUERY, "  "].join("\n"),
        vec![ANNO, QUERY, "  ", "\t"].join("\n"),
        vec![&anno_with_with_trailing_whitespace as &str, QUERY].join("\n"),
    ];
    for sample in samples {
        let parsed = parser::parse(&sample).unwrap();
        assert!(parsed["now"].name() == "now");
        assert!(parsed["now"].text() == QUERY);
    }
}


#[test]
fn above_comment() {
    let sample = vec![CMT, ANNO, QUERY].join("\n");
    let parsed = parser::parse(&sample).unwrap();
    assert!(parsed["now"].name() == "now");
    assert!(parsed["now"].text() == QUERY);
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
        assert!(parsed["now"].name() == "now");
        assert!(parsed["now"].text() == QUERY);
    }
}


#[test]
fn query_comment() {
    let sample = vec![ANNO, CMT, QUERY].join("\n");
    let parsed = parser::parse(&sample).unwrap();
    assert!(parsed["now"].name() == "now");
    assert!(parsed["now"].text() == format!("{}\n{}", CMT, QUERY));
}
