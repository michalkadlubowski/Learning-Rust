use std::io::Error;

use tokio::io::{AsyncWrite, AsyncWriteExt};

use super::StatusCode;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    pub async fn send(&self, stream: &mut (impl AsyncWrite + std::marker::Unpin))  -> Result<(), Error> {
        let body = match &self.body {
            Some(s) => s,
            None => ""
        };
        let res = format!("HTTP/1.1 {} {}\r\n\r\n{}",
        self.status_code,
        self.status_code.reason_phrase(),
        body);
        
        stream.write_all(res.as_bytes()).await
    }
}