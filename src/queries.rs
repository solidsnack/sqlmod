use std;
use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::ops::Index;


#[derive(Debug)]
pub struct Queries {
    pub info: Option<String>,
    pub queries: Vec<Query>,
    pub indexes: BTreeMap<String, usize>,
}

impl Queries {
    pub fn new<'a, Collection, Q>(items: Collection) -> Queries
        where Collection: IntoIterator<Item = Q>,
              Q: Into<Query>
    {
        let mut queries = Vec::default();
        let mut indexes = BTreeMap::default();

        for item in items {
            let q = item.into();
            indexes.insert(q.name(), queries.len());
            queries.push(q);
        }

        Queries { info: None, queries: queries, indexes: indexes }
    }

    pub fn keys(&self) -> std::vec::IntoIter<String> {
        let vec: Vec<_> = self.queries.iter().map(|q| q.name()).collect();
        vec.into_iter()
    }

    pub fn len(&self) -> usize { self.queries.len() }

    pub fn iter(&self) -> std::slice::Iter<Query> { self.queries.iter() }

    pub fn format(&self) -> String {
        let texts: Vec<_> = self.queries.iter().map(|q| q.format()).collect();
        texts.join("\n\n\n")
    }

    pub fn info(&self) -> Option<String> { self.info.clone() }

    pub fn get<K: ?Sized>(&self, key: &K) -> Option<&Query>
        where K: Borrow<str>
    {
        self.indexes.get(key.borrow()).map(|i| &self.queries[*i])
    }
}

impl<'a, K: ?Sized> Index<&'a K> for Queries
    where K: Borrow<str>
{
    type Output = Query;

    fn index(&self, index: &K) -> &Query {
        &self.queries[self.indexes[index.borrow()]]
    }
}

impl IntoIterator for Queries {
    type Item = Query;
    type IntoIter = std::vec::IntoIter<Query>;

    fn into_iter(self) -> Self::IntoIter { self.queries.into_iter() }
}


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Query {
    pub signature: Signature,
    pub text: String,
}

impl Query {
    pub fn format(&self) -> String {
        let mut signature = vec![self.name()];
        if self.readonly() {
            signature.push("ro".into());
        }
        if let Some(ref coarity) = self.signature.coarity {
            signature.push(coarity.format());
        }
        format!("--@ {}\n{}", signature.join(" "), self.text)
    }

    pub fn name(&self) -> String { self.signature.name.clone() }

    pub fn readonly(&self) -> bool { self.signature.ro }

    pub fn text(&self) -> String { self.text.clone() }
}

impl<'a> From<(&'a str, &'a str)> for Query {
    fn from(pair: (&str, &str)) -> Query { (&pair).into() }
}

impl<'a> From<&'a (&'a str, &'a str)> for Query {
    fn from(pair: &(&str, &str)) -> Query {
        Query {
            signature: Signature {
                name: pair.0.into(),
                ro: false,
                coarity: None,
            },
            text: pair.1.into(),
        }
    }
}


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Signature {
    pub name: String,
    pub ro: bool,
    pub coarity: Option<CoArity>,
}


#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CoArity {
    Zero,
    Maybe,
    One,
    Any,
    Many,
}

pub use self::CoArity::*;

impl CoArity {
    pub fn parse(s: &str) -> Option<CoArity> {
        match s {
            "()" => Some(Zero),
            "(?)" => Some(Maybe),
            "(1)" => Some(One),
            "(*)" => Some(Any),
            "(+)" => Some(Many),
            _ => None,
        }
    }

    pub fn format(&self) -> String {
        match *self {
            Zero => "()",
            Maybe => "(?)",
            One => "(1)",
            Any => "(*)",
            Many => "(+)",
        }
        .into()
    }
}

impl Default for CoArity {
    fn default() -> Self { Any }
}


#[cfg(test)]
mod tests {
    use queries::*;

    #[test]
    fn it_works() {
        let data = [("q1", "SELECT 1"), ("q2", "SELECT 2")];
        let queries = Queries::new(&data);

        assert!(queries.len() == 2);
    }

    #[test]
    fn indexing() {
        let data = [("q1", "SELECT 1"), ("q2", "SELECT 2")];
        let queries = Queries::new(&data);

        assert!(queries["q1"].text() == "SELECT 1");
        assert!(queries["q2"].text() == "SELECT 2");
        assert!(queries.get("q3") == None);
    }

    #[test]
    fn getting() {
        let data = [("q1", "SELECT 1"), ("q2", "SELECT 2")];
        let queries = Queries::new(&data);

        let q1 = Query {
            signature: Signature {
                name: data[0].0.into(),
                ro: false,
                coarity: None,
            },
            text: data[0].1.into(),
        };

        assert!(queries.get("q1") == Some(&q1));
        assert!(queries.get("q3") == None);
    }

    #[test]
    fn formatting() {
        let data = [("q1", "SELECT 1"), ("q2", "SELECT 2")];
        let queries = Queries::new(&data);

        let q1 = format!("--@ {}\n{}", data[0].0, data[0].1);
        let q2 = format!("--@ {}\n{}", data[1].0, data[1].1);

        assert!(queries["q1"].format() == q1);
        assert!(queries["q2"].format() == q2);
    }

}
