#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Query {
    pub signature: Signature,
    pub text: String,
}

impl Query {
    pub fn format(&self) -> String {
        format!("--@ {}\n{}", self.signature.format(), self.text)
    }

    pub fn name(&self) -> String { self.signature.name.clone() }

    pub fn attributes(&self) -> Vec<String> {
        self.signature.attributes.clone()
    }

    pub fn text(&self) -> String { self.text.clone() }
}

impl<'a> From<&'a Query> for Query {
    fn from(query: &Query) -> Query { query.clone() }
}

impl<'a> From<(&'a str, &'a str)> for Query {
    fn from(pair: (&str, &str)) -> Query { (&pair).into() }
}

impl<'a> From<&'a (&'a str, &'a str)> for Query {
    fn from(pair: &(&str, &str)) -> Query {
        Query {
            signature: Signature { name: pair.0.into(), attributes: vec![] },
            text: pair.1.into(),
        }
    }
}


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Signature {
    pub name: String,
    pub attributes: Vec<String>,
}

impl Signature {
    pub fn format(&self) -> String {
        if self.attributes.len() > 0 {
            vec![self.name.clone(), self.attributes.join(" ")].join(" ")
        } else {
            self.name.clone()
        }
    }
}
