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
    Multiple(Vec<&'rs str>)
}

impl<'rs> QueryString<'rs> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
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
            data.entry(key)

            .and_modify(|existing| match existing {
                // Must dereference pointer in order to overwrite the value, all enum variants take up the same space
                Value::One(prev) => *existing = Value::Multiple(vec![prev, val]),
                Value::Multiple(vec) => vec.push(val)

            })

            .or_insert(Value::One(val));
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
                Value::Multiple(vec) => format!("{}\n  {}={:?}", string, key, vec)
            }
        }

        write!(f, "{}\n}}", string)
    }
}