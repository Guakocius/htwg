use std::io::{Read, Write};
use std::net::{AddrParseError, Shutdown, SocketAddr, TcpListener, TcpStream};
use std::process;
use std::thread;
use std::time::Duration;

use crate::client::client::ClientList;

#[derive(Debug, Clone)]
pub struct Server {
    pub ip: String,
    pub port: String,
    pub client_list: ClientList,
}

impl Server {
    pub fn new() -> Self {
        Server {
            ip: String::from("127.0.0.1"),
            port: String::from("5000"),
            client_list: ClientList::new(),
        }
    }

    pub async fn listen(&self) {
        let listener = TcpListener::bind(std::format!("{}:{}", self.ip, self.port)).unwrap();

        println!(
            "Server: Listening on port {} for incoming TCP connections",
            self.port
        );

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    println!("Server: Incoming stream from client: {:?}", stream);
                    Self::receive(&mut stream);
                }
                Err(e) => {
                    eprintln!("connection failed: {}", e);
                }
            }
        }
    }
    fn receive(socket: &mut TcpStream) {
        loop {
            let mut buf = [0; 1024];
            match socket.read(&mut buf) {
                Ok(0) => {
                    println!("Server: Connection closed from other side.\nClosing...");
                    break;
                }
                Ok(b) => {
                    let buf_str = std::str::from_utf8(&buf[..b]).expect("invalid utf-8 sequence");

                    if !buf_str.chars().any(|c| c == '|') {
                        println!(
                            "Server: Received data length: {}\nReceived data: {:?}",
                            b, buf_str
                        );
                    } else {
                        println!("Closing connection");
                        process::exit(0x0100);
                    }
                }
                Err(e) => panic!("encountered IO error: {}", e),
            }
        }
    }

    async fn send(
        self,
        addr: SocketAddr,
        data: String,
    ) -> Result<(String, String, TcpStream), AddrParseError> {
        let mut stream = TcpStream::connect(addr).unwrap();

        stream.write_all(data.as_bytes()).unwrap();
        Ok((self.ip, self.port, stream))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::client::{Client, ClientList};

    #[test]
    fn test_new() {
        let server = Server::new();
        let client_list = ClientList::new();

        assert_eq!(server.ip, String::from("127.0.0.1"));
        assert_eq!(server.port, String::from("5000"));
        assert_eq!(
            server.client_list,
            ClientList {
                client_list: client_list.client_list
            }
        );
    }
}
