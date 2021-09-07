
use crate::http::{ParseError, Response, StatusCode, request::Request};
use std::{convert::{TryFrom}, io::{Read}, net::{TcpListener, TcpStream}};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response; 
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)        
    }
} 

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new (addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(self, mut handler: impl Handler) -> () {
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((stream,_)) => process_request(stream, &mut handler),
                Err(e) => println!("Connection error: {}", e)
            }
        }
    }
}

fn process_request(mut stream: TcpStream, handler: &mut impl Handler) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(_) => {
            let response = match Request::try_from(&buffer[..])
            {
                Ok(r) => handler.handle_request(&r),
                Err(e) => handler.handle_bad_request(&e)
            };
            if let Err(_) = response.send(&mut stream) {
                println!("Failed to send response!");
            }
        },
        Err(e) => println!("Stream read error: {}", e)
    }
}
