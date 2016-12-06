use std::io::*;

use lines::*;
use queries::*;
use peg;


pub fn parse<T: BufRead>(text: &T) -> Result<Queries> {
    let s = text.chars().into();
    let n = 0;

    let mut queries = Vec::default();
    let mut warnings = Vec::default();

    let mut z = 0;
    let mut within = None;

    for line in peg::lines(&s)? {
        n += 1;
        match line {
            Declaration(position, name, ro) => {
                if let Some(item) = within {
                     queries.push(Query {
                         position: (item.0, z),
                         name: item.1,
                         ro: item.2,
                     });
                }
                within = Some((position, name, ro));
            }
            BrokenDeclaration(_, _) => {
                warnings.push((n, line));
            }
            _ => {}
        }
        z = line.end();
    }

    Queries { text: s, queries: queries, warings: warnings }
}
