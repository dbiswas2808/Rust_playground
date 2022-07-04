use super::method::{Method, MethodError};
use super::{QueryString, QueryStringValue};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::io::Write;
use std::net::TcpStream;
use std::str;
use std::str::Utf8Error;

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: super::method::Method,
}

impl<'buf> Request<'buf> {
    fn from_byte_array(buf: &[u8]) -> Result<Self, String> {
        unimplemented!()
    }

    pub fn path(&self) -> &str {
        &self.path
    }
    pub fn method(&self) -> &Method {
        &self.method
    }
    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1

    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {
        let request: &'buf str = str::from_utf8(buf)?;
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;
        let mut query_str = None;

        if let Some(i) = path.find('?') {
            query_str = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            path: path,
            query_string: query_str,
            method: method,
        })
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    None
}

// trait Encrypt {
//     fn encrypt(&self) -> Self;
// }

// impl Encrypt for String {
//     fn encrypt(&self) -> Self {
//         unimplemented!();
//     }
// }

// impl Encrypt for &[u8] {
//     fn encrypt(&self) -> Self {
//         unimplemented!()
//     }
// }

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> ParseError {
        ParseError::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> ParseError {
        ParseError::InvalidMethod
    }
}
