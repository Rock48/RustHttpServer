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