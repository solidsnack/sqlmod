#[derive(Debug)]
pub enum Line {
    Declaration((usize, usize), (usize, usize), bool),
    BrokenDeclaration(usize, usize),
    Comment(usize, usize),
    Empty(usize, usize),
    Text(usize, usize),
}

pub use self::Line::*;

impl Line {
    pub fn start(&self) -> &usize {
        match *self {
            Declaration((ref start, _), _, _) => start,
            BrokenDeclaration(ref start, _) => start,
            Comment(ref start, _) => start,
            Empty(ref start, _) => start,
            Text(ref start, _) => start,
        }
    }

    pub fn end(&self) -> &usize {
        match *self {
            Declaration((_, ref end), _, _) => end,
            BrokenDeclaration(_, ref end) => end,
            Comment(_, ref end) => end,
            Empty(_, ref end) => end,
            Text(_, ref end) => end,
        }
    }
}
