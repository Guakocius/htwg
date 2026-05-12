use std::net::{TcpListener, TcpStream};

use crate::client::client;

struct Server {
    ip: String,
    port: i32,
}

impl Server {
    fn new() -> Server {
        Server {
            ip: String::from("127.0.0.1"),
            port: 22,
        }
    }

    fn listen(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(std::format!("{}:{}", self.ip, self.port))?;

        for stream in listener.incoming() {
            self.handle_client(stream?);
        }
        Ok(())
    }

    fn handle_client(&self, stream: TcpStream) {}
}
