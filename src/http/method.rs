
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

use super::ParseError;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Method {
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH 
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let method_str = match self {
            Self::GET => "GET",
            Self::DELETE => "DELETE",
            Self::POST => "POST",
            Self::PUT => "PUT",
            Self::HEAD => "HEAD",
            Self::CONNECT => "CONNECT",
            Self::OPTIONS => "OPTIONS",
            Self::TRACE => "TRACE",
            Self::PATCH => "PATCH" 
        };

        write!(f, "{}", method_str)
    }
}

impl FromStr for Method {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "DELETE" => Ok(Self::DELETE),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "HEAD" => Ok(Self::HEAD),
            "CONNECT" => Ok(Self::CONNECT),
            "OPTIONS" => Ok(Self::OPTIONS),
            "TRACE" => Ok(Self::TRACE),
            "PATCH" => Ok(Self::PATCH),
            _ => Err(ParseError::InvalidMethod)
        }
    }
}
