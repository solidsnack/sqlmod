pub use queries::*;


#[derive(Debug)]
pub enum Line<'input> {
    Declaration(Section<'input>, Signature),
    BrokenDeclaration(Section<'input>),
    Comment(Section<'input>),
    Empty(Section<'input>),
    Text(Section<'input>),
}

pub use self::Line::*;

impl<'input> Line<'input> {
    pub fn section(&'input self) -> &'input Section<'input> {
        match *self {
            Declaration(ref m, _) => m,
            BrokenDeclaration(ref m) => m,
            Comment(ref m) => m,
            Empty(ref m) => m,
            Text(ref m) => m,
        }
    }

    pub fn start(&self) -> usize { self.section().start() }

    pub fn end(&self) -> usize { self.section().end() }

    pub fn text(&self) -> &str { self.section().s() }

    pub fn blank(&self) -> bool { self.section().s().trim().len() == 0 }
}


#[derive(Debug)]
pub struct Section<'input>(pub usize, pub usize, pub &'input str);

impl<'input> Section<'input> {
    pub fn start(&self) -> usize { self.0.clone() }

    pub fn end(&self) -> usize { self.1.clone() }

    pub fn s(&self) -> &'input str { self.2 }
}
