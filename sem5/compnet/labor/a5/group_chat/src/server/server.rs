use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use crate::client::client::{Client, ClientList};

pub struct Server {
    pub ip: String,
    pub port: i32,
    pub client_list: ClientList,
}

impl Server {
    pub fn new() -> Server {
        Server {
            ip: String::from("127.0.0.1"),
            port: 22,
            client_list: ClientList::new(),
        }
    }

    fn listen(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(std::format!("{}:{}", self.ip, self.port))?;

        println!(
            "Listening on port {} for incoming TCP connections",
            self.port
        );

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(|| Self::handle_client(stream));
                }
                Err(e) => {
                    eprintln!("connection failed: {}", e);
                }
            }
        }

        Ok(())
    }

    fn handle_client(mut stream: TcpStream) {
        println!("Incoming conneciton accepted: {:?}", stream.peer_addr());
    }
}
