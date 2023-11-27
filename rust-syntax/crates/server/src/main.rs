fn main() {
    let server = Server::new("127.0.0.1:8080");
    server.run();
}

struct Server<'a> {
    addr: &'a str,
}

impl<'a> Server<'a> {
    fn new(addr: &'a str) -> Self {
        Self { addr }
    }

    fn run(&self) {}
}
