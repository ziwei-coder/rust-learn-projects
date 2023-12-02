use std::io::{Result as IoResult, Write};

use crate::http::StatusCode;

pub struct Response<'a> {
    status_code: StatusCode,
    body: Option<&'a str>,
}

impl<'a> Response<'a> {
    pub fn new(status_code: StatusCode, body: Option<&'a str>) -> Self {
        Self { status_code, body }
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = self.body.unwrap_or("");

        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n {}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}
