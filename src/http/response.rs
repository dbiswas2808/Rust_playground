use super::StatusCode;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::{Result as IoResult, Write};
use std::net::TcpStream;

pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Response {
        Response { status_code, body }
    }

    pub fn send(&self, st: &mut impl Write) -> IoResult<()> {
        write!(
            st,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            match &self.body {
                Some(b) => b,
                None => "",
            }
        )
    }
}

// impl Display for Response {
//     fn fmt(&self, f: &mut Formatter) -> FmtResult {
//         write!(
//             f,
//             "HTTP/1.1 {} {}\r\n\r\n{}",
//             self.status_code,
//             self.status_code.reason_phrase(),
//             match &self.body {
//                 Some(b) => b,
//                 None => "",
//             }
//         )
//     }
// }
