use std::collections::BTreeMap;
use std::iter::FromIterator;
use std::io::Read;

use errors::*;
use lines::*;
use queries::*;
use peg;


pub fn parse<P: Parse>(text: P) -> Result<Queries> { Parse::parse(text) }


pub trait Parse {
    fn parse(text: Self) -> Result<Queries>;
}


impl<'a, R: Read> Parse for &'a mut R {
    fn parse(text: &mut R) -> Result<Queries> {
        read(text).map(|ref s| Parse::parse(s))?
    }
}


impl<'a> Parse for &'a String {
    fn parse(text: &String) -> Result<Queries> { Parse::parse(text as &str) }
}


impl<'a, P: Parse> Parse for (&'a str, P) {
    fn parse(source: (&'a str, P)) -> Result<Queries> {
        let Queries { text, queries, warnings, .. } = Parse::parse(source.1)?;
        Ok(Queries {
            info: source.0.into(),
            text: text,
            warnings: warnings,
            queries: queries,
        })
    }
}


impl<'a> Parse for &'a str {
    fn parse(text: &'a str) -> Result<Queries> {
        let mut queries = Vec::default();
        let mut warnings = Vec::default();
        let mut lineno = 0;
        let mut within: Option<(usize, usize, Signature)> = None;
        let mut start = 0;         // Byte offset: begin-of-declaration-pointer
        let mut end = 0;             // Byte offset: end-of-declaration-pointer

        for line in peg::lines(&text)? {
            lineno += 1;
            match line {          // Consume declaration information if present
                Declaration(_, ref signature) => {
                    if let Some((l, i, sig)) = within {
                        if start > 0 {
                            queries.push(query(text, l, i, sig, (start, end)));
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
        if let Some((l, i, sig)) = within {
            if start > 0 {
                queries.push(query(text, l, i, sig, (start, end)));
            }
        }

        Ok(Queries {
            text: text.into(),
            info: "".into(),
            warnings: BTreeMap::from_iter(warnings.into_iter()),
            queries: BTreeMap::from_iter(queries.into_iter()
                                                .map(|q| {
                                                    (q.signature.name.clone(),
                                                     q)
                                                })),
        })
    }
}


fn read<R: Read>(source: &mut R) -> Result<String> {
    let mut s: String = String::default();
    source.read_to_string(&mut s)?;
    Ok(s)
}


fn query(text: &str,
         line: usize,
         begin: usize,
         signature: Signature,
         body: (usize, usize))
         -> Query {
    Query {
        signature: signature,
        line: line,
        text: text[body.0..body.1].trim_right().into(),
        original: text[begin..body.1].into(),
    }
}
