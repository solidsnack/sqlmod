use std;
use std::borrow::Borrow;
use std::iter::FromIterator;
use std::ops::Index;

use linked_hash_map::LinkedHashMap;

use query::*;


#[derive(Debug)]
pub struct Queries {
    info: Option<String>,
    data: LinkedHashMap<String, Query>,
}

impl Queries {
    pub fn new<'a, Collection, Q>(info: Option<String>,
                                  items: Collection)
                                  -> Queries
        where Collection: IntoIterator<Item = Q>,
              Q: Into<Query>
    {
        let mut data = LinkedHashMap::default();

        for item in items {
            let q = item.into();
            data.insert(q.name(), q);
        }

        Queries { info: info, data: data }
    }

    pub fn keys(&self) -> std::vec::IntoIter<&String> {
        // Cast to hide library iterator type.
        let vec: Vec<_> = self.data.keys().collect();
        vec.into_iter()
    }

    pub fn len(&self) -> usize { self.data.len() }

    pub fn iter(&self) -> std::vec::IntoIter<&Query> {
        // Cast to hide library iterator type.
        let vec: Vec<_> = self.data.values().collect();
        vec.into_iter()
    }

    pub fn format(&self) -> String {
        let texts: Vec<_> = self.iter().map(|q| q.format()).collect();
        texts.join("\n\n\n")
    }

    pub fn info(&self) -> Option<String> { self.info.clone() }

    pub fn get<K: ?Sized>(&self, key: &K) -> Option<&Query>
        where K: Borrow<str>
    {
        self.data.get(key.borrow())
    }
}

impl<'a, K: ?Sized> Index<&'a K> for Queries
    where K: Borrow<str>
{
    type Output = Query;

    fn index(&self, index: &K) -> &Query { &self.data[index.borrow()] }
}

impl<Q> FromIterator<Q> for Queries
    where Q: Into<Query>
{
    fn from_iter<I>(items: I) -> Self
        where I: IntoIterator<Item = Q>
    {
        Queries::new(None, items)
    }
}



#[cfg(test)]
mod tests {
    use queries::*;
    use query::*;

    #[test]
    fn it_works() {
        let data = [("q1", "SELECT 1"), ("q2", "SELECT 2")];
        let queries: Queries = data.iter().collect();

        assert!(queries.len() == 2);
    }

    #[test]
    fn indexing() {
        let data = [("q1", "SELECT 1"), ("q2", "SELECT 2")];
        let queries: Queries = data.iter().collect();

        assert!(queries["q1"].text() == "SELECT 1");
        assert!(queries["q2"].text() == "SELECT 2");
        assert!(queries.get("q3") == None);
    }

    #[test]
    fn getting() {
        let data = [("q1", "SELECT 1"), ("q2", "SELECT 2")];
        let queries: Queries = data.iter().collect();

        let q1 = Query {
            signature: Signature {
                name: data[0].0.into(),
                attributes: vec![],
            },
            text: data[0].1.into(),
        };

        assert!(queries.get("q1") == Some(&q1));
        assert!(queries.get("q3") == None);
    }

    #[test]
    fn formatting() {
        let data = [("q1", "SELECT 1"), ("q2", "SELECT 2")];
        let queries: Queries = data.iter().collect();

        let q1 = format!("--@ {}\n{}", data[0].0, data[0].1);
        let q2 = format!("--@ {}\n{}", data[1].0, data[1].1);

        println!("q1 fixture:  {:?}", q1);
        println!("q1 format(): {:?}", queries["q1"].format());
        println!("q2 fixture:  {:?}", q2);
        println!("q2 format(): {:?}", queries["q2"].format());

        assert!(queries["q1"].format() == q1);
        assert!(queries["q2"].format() == q2);
    }

}
