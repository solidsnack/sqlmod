use std::collections::HashMap;
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
        let mut within: Option<((usize, usize), (usize, usize), bool)> = None;
        let mut end = 0;             // Byte offset: end-of-declaration-pointer



        for line in peg::lines(&text)? {
            lineno += 1;
            end = match line {
                Declaration(_, _, _) => line.end().clone(),
                Text(_, _) => line.end().clone(),
                _ => end,
            };
            match line {          // Consume declaration information if present
                Declaration(position, name, ro) => {
                    if let Some(dec) = within {
                        let start = (dec.0).0;
                        queries.push(query(text, (start, end), dec.1, dec.2));
                    }
                    within = Some((position, name, ro));
                }
                BrokenDeclaration(_, _) => {
                    warnings.push((lineno, line));
                }
                _ => {}
            }

        }

        // Handle last declaration.
        if let Some(dec) = within {
            let start = (dec.0).0;
            queries.push(query(text, (start, end), dec.1, dec.2));
        }

        Ok(Queries {
            text: text.into(),
            info: "".into(),
            warnings: HashMap::from_iter(warnings.into_iter()),
            queries: HashMap::from_iter(queries.into_iter()
                                               .map(|q| (q.name.clone(), q))),
        })
    }
}


fn read<R: Read>(source: &mut R) -> Result<String> {
    let mut s: String = String::default();
    source.read_to_string(&mut s)?;
    Ok(s)
}


fn query(text: &str,
         body: (usize, usize),
         name: (usize, usize),
         ro: bool)
         -> Query {
    let ref full = text[body.0..body.1];
    Query {
        name: text[name.0..name.1].trim().into(),
        text: full.splitn(2, '\n').last().unwrap().trim_right().into(),
        full: full.into(),
        ro: ro,
    }
}
