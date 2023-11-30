use std::io::Read;
use std::net::{TcpListener, TcpStream};

use crate::http::Request;

pub struct Server<'a> {
    addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(addr: &'a str) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        // We need to crash the application if listening fails.
        let listener = TcpListener::bind(self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((stream, _)) => Self::read_stream(stream),
                Err(e) => println!("Failed to establish a connection: {}", e),
            };
        }
    }

    fn read_stream(mut stream: TcpStream) {
        let mut buffer = [0; 1024];

        match stream.read(&mut buffer) {
            Ok(_) => {
                println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                match Request::try_from(&buffer[..]) {
                    Ok(request) => {}
                    Err(e) => println!("Failed to parse a request: {}", e),
                }
            }
            Err(e) => println!("Failed to read from connection: {}", e),
        };
    }
}
