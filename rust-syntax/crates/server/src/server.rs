use std::io::Read;
use std::net::{TcpListener, TcpStream};

use crate::http::request::ParseError;
use crate::http::{Request, Response, StatusCode};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse a request: {}", e);
        Response::new(StatusCode::Ok, None)
    }
}

pub struct Server<'a> {
    addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(addr: &'a str) -> Self {
        Self { addr }
    }

    pub fn run(self, handler: &mut impl Handler) {
        println!("Listening on {}", self.addr);

        // We need to crash the application if listening fails.
        let listener = TcpListener::bind(self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((stream, _)) => Self::handle_stream(stream, handler),
                Err(e) => println!("Failed to establish a connection: {}", e),
            };
        }
    }

    fn handle_stream(mut stream: TcpStream, handler: &mut impl Handler) {
        let mut buffer = [0; 1024];

        match stream.read(&mut buffer) {
            Err(e) => println!("Failed to read from connection: {}", e),
            Ok(_) => {
                println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                let response = match Request::try_from(&buffer[..]) {
                    Ok(request) => {
                        dbg!(&request);
                        handler.handle_request(&request)
                    }
                    Err(e) => handler.handle_bad_request(&e),
                };

                if let Err(e) = response.send(&mut stream) {
                    println!("Failed to send response: {}", e);
                }
            }
        };
    }
}
