use std::convert::TryFrom;
use std::fmt::{Display, Formatter, Result as FmtResult, Debug};
use std::str;
use super::{QueryString, Method, ParseError};
/*
EXAMPLE HTTP REQUEST:

GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/

#[derive(Debug)]
pub struct Request<'rs> {
    path: &'rs str,
    method: Method,
    query: Option<QueryString<'rs>>,
    body: Option<&'rs str>,
}

impl<'rs> Request<'rs> {
    pub fn path(&self) -> &str { self.path }
    pub fn method(&self) -> &Method { &self.method }
    pub fn query(&self) -> Option<&QueryString> { self.query.as_ref() }
    pub fn body(&self) -> Option<&str> { self.body }
}

/**
 * Gets the next word in the string, returning a slice of the word as well as a slice of the remaining string
 */
fn get_next_word(input: &str) -> Option<(&str,&str)> {
    for (i, c) in input.chars().enumerate() {
        if c == ' ' || c == '\r' { 
            // &input[i+1..] is NOT adding one char, it is adding one BYTE
            // this could generate invalid UTF8
            // however, a space and a newline is one byte, so we know it's ok
            return Some((&input[..i], &input[i+1..]));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::http::query_string::Value;

    use super::*;

    #[test]
    fn next_word() {
        let mut my_str = "this is my\rstring ";
        let (word,mut my_str) = get_next_word(&mut my_str).expect("no next word!");
        assert_eq!(word, "this");
        assert_eq!(my_str, "is my\rstring ");
        let (word,mut my_str) = get_next_word(&mut my_str).expect("no next word!");
        assert_eq!(word, "is");
        assert_eq!(my_str, "my\rstring ");
        let (word,mut my_str) = get_next_word(&mut my_str).expect("no next word!");
        assert_eq!(word, "my");
        assert_eq!(my_str, "string ");
        let (word,mut my_str) = get_next_word(&mut my_str).expect("no next word!");
        assert_eq!(word, "string");
        assert_eq!(my_str, "");

        match get_next_word(&mut my_str) {
            None => {}, // do nothing, all good.
            Some(t) => panic!("Found extra word, tuple result: {:?}", t)
        }
    }

    #[test]
    fn invalid_protocol() {
        if let Err(e) = Request::try_from("GET /user?id=10 HTTP/1.2\r\n".as_bytes()) {
            assert_eq!(e, ParseError::InvalidProtocol);
        } else {
            panic!("Invalid protocol came back valid");
        }
    }

    #[test]
    fn valid_request() {
        let req = Request::try_from("GET /user?id=10 HTTP/1.1\r\nTODO: HEADERS\r\nNice Body".as_bytes()).expect("Request failed to parse");

        assert_eq!(req.method, Method::GET);
        assert_eq!(req.path, "/user");

        let qs = req.query.expect("Query string missing");

        assert_eq!(qs.len(), 1);
        
        let val = qs.get("id").expect("id in qs missing");

        match &val {
            Value::One(v) => assert_eq!(*v, "10"),
            Value::Multiple(v) => panic!("user has multiple values {:?}", v),
            Value::None => panic!("user has no value")
        }

        
        assert_eq!(req.body, None); // TODO!!! 
    }
}

impl<'rs> TryFrom<&'rs [u8]> for Request<'rs> {
    type Error = ParseError;

    fn try_from(bytes: &'rs [u8]) -> Result<Self, Self::Error> {
        let request: &str = str::from_utf8(bytes)?;

        // let (method, request) = match get_next_word(request) {
        //     Some(result) => result,
        //     None => return Err(ParseError::InvalidRequest)
        // }

        // you can use ? though
        
        // GET /user?id=10 HTTP/1.1\r\n
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;
        let mut query = None;
        if let Some(i) = path.find('?') {
            // we know '?' is 1 byte so [i+1] is ok
            query = Some(QueryString::from(&path[i+1..]));
            path = &path[..i];
        }

        Ok( Self { 
            path: path, 
            method, 
            query, 
            body: None 
        })
    }
}

impl<'rs> Display for Request<'rs> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let body = self.body.unwrap_or("NONE");
        let query = match &self.query {
            None => Str!("NONE"),
            Some(qs) => qs.to_string()
        };
        write!(f, "PATH: \"{}\"\nMETHOD: {}\nQUERY:\n{}\nBODY\n=====\n{}\n", self.path, self.method, query, body)
    }
}