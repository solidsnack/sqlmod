use std::io::Read;

use errors::*;
use lines::Line::*;
use queries::*;
use query::*;
use peg;


pub fn parse<P: Parse>(text: P) -> Result<Queries> { Parse::parse(text) }


pub trait Parse {
    fn parse(text: Self) -> Result<Queries>;
}


impl<'a, R: Read> Parse for &'a mut R {
    fn parse(text: &mut R) -> Result<Queries> {
        try!(read(text).map(|ref s| Parse::parse(s)))
    }
}


impl<'a> Parse for &'a String {
    fn parse(text: &String) -> Result<Queries> { Parse::parse(text as &str) }
}


impl<'a, P: Parse> Parse for (&'a str, P) {
    fn parse(source: (&'a str, P)) -> Result<Queries> {
        let noinfo = try!(Parse::parse(source.1));
        let withinfo = Queries::new(Some(source.0.into()), noinfo.iter());
        Ok(withinfo)
    }
}


impl<'a> Parse for &'a str {
    fn parse(text: &'a str) -> Result<Queries> {
        let mut queries = Vec::default();
        let mut warnings: Vec<(usize, String)> = Vec::default();
        let mut lineno = 0;
        let mut within: Option<(usize, usize, Signature)> = None;
        let mut start = 0;         // Byte offset: begin-of-declaration-pointer
        let mut end = 0;             // Byte offset: end-of-declaration-pointer

        for line in try!(peg::lines(&text)) {
            lineno += 1;
            match line {          // Consume declaration information if present
                Declaration(_, ref signature) => {
                    if let Some((_, _, sig)) = within {
                        if start > 0 {
                            queries.push(query(text, sig, (start, end)));
                        }
                    }
                    start = 0;
                    within = Some((lineno, line.start(), signature.clone()));
                }
                BrokenDeclaration(_) => {
                    warnings.push((lineno, line.text().into()));
                }
                _ => {}
            }
            if start == 0 {
                start = match line {
                    Text(_) if !line.blank() => line.start(),
                    Comment(_) => line.start(),
                    _ => start,
                };
            }
            end = match line {
                Declaration(_, _) => line.end(),
                Text(_) if !line.blank() => line.end(),
                _ => end,
            };
        }

        // Handle last declaration.
        if let Some((_, _, sig)) = within {
            if start > 0 {
                queries.push(query(text, sig, (start, end)));
            }
        }

        let queries: Queries = queries.iter().collect();

        Ok(queries)
    }
}


fn read<R: Read>(source: &mut R) -> Result<String> {
    let mut s: String = String::default();
    try!(source.read_to_string(&mut s));
    Ok(s)
}


fn query(text: &str, signature: Signature, body: (usize, usize)) -> Query {
    Query {
        signature: signature,
        text: text[body.0..body.1].trim_right().into(),
    }
}
