use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::vec::Vec;

#[derive(Debug)]
pub struct QueryString<'rs> {
    data: HashMap<&'rs str, Value<'rs>>
}

#[derive(Debug)]
pub enum Value<'rs> {
    One(&'rs str),
    Multiple(Vec<&'rs str>),
    None
}

impl<'rs> QueryString<'rs> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

// ex a=1&b=2&c&e====&d=7&d=abc
// { a: 1, b:2, c:None, e:===, d:[7, abc]}
impl<'rs> From<&'rs str> for QueryString<'rs> {
    fn from(s: &'rs str) -> Self {
        let mut data = HashMap::new();

        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";

            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }
            let mut to_insert = Value::None;

            if val != "" {
                to_insert = Value::One(val);
            }

            data.entry(key)

            .and_modify(|existing| match existing {
                // Must dereference pointer in order to overwrite the value, all enum variants take up the same space
                Value::None => *existing = Value::One(val),
                Value::One(prev) => *existing = Value::Multiple(vec![prev, val]),
                Value::Multiple(vec) => vec.push(val)

            })

            .or_insert(to_insert);
        }

        QueryString { data }
    }
}

impl<'rs> Display for QueryString<'rs> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mut string = Str!("{");
        for (key, value) in (&self.data).into_iter() {
            string = match value {
                Value::One(val) => format!("{}\n  {}=\"{}\"", string, key, val),
                Value::Multiple(vec) => format!("{}\n  {}={:?}", string, key, vec),
                Value::None => format!("{}\n \"{}\"", string, key)
            }
        }

        write!(f, "{}\n}}", string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_qs() {
        let qs = QueryString::from("a=1&f&b=2&c&e====&d=7&d=abc&f=5");

        // { a: 1, b:2, c:None, e:===, d:[7, abc], f:5}

        match &qs.data["a"] {
            Value::One(v) => assert_eq!(*v, "1"),
            Value::Multiple(v) => panic!("a has multiple values {:?}", v),
            Value::None => panic!("a has no value")
        }
        match &qs.data["b"] {
            Value::One(v) => assert_eq!(*v, "2"),
            Value::Multiple(v) => panic!("b has multiple values {:?}", v),
            Value::None => panic!("b has no value")
        }
        match &qs.data["c"] {
            Value::One(v) => panic!("c has a value {}", v),
            Value::Multiple(v) => panic!("c has multiple values {:?}", v),
            Value::None => {} // everything is cool, dont need to worry.
        }
        match &qs.data["d"] {
            Value::One(v) => panic!("d only has one value {}", v),
            Value::Multiple(v) => assert!(v.len() == 2 && v.contains(&"7") && v.contains(&"abc")),
            Value::None => panic!("d has no value")
        }
        match &qs.data["e"] {
            Value::One(v) => assert_eq!(*v, "==="),
            Value::Multiple(v) => panic!("e has multiple values {:?}", v),
            Value::None => panic!("e has no value")
        }
        match &qs.data["f"] {
            Value::One(v) => assert_eq!(*v, "5"),
            Value::Multiple(v) => panic!("f has multiple values {:?}", v),
            Value::None => panic!("f has no value")
        }
    }
}