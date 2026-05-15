use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};

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

    pub fn listen(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(std::format!("{}:{}", self.ip, self.port)).unwrap();

        println!(
            "Listening on port {} for incoming TCP connections",
            self.port
        );

        loop {
            println!("inside loop");
            for stream in listener.incoming() {
                println!("{:?}", stream);
                match stream {
                    Ok(mut stream) => {
                        Self::receive(&mut stream);
                    }
                    Err(e) => {
                        eprintln!("connection failed: {}", e);
                    }
                }
            }
            match listener.accept() {
                Ok((mut _socket, addr)) => {
                    println!("Incoming connection accepted: {:?}", addr);
                    Self::receive(&mut _socket);
                }
                Err(e) => eprintln!("couldn't get client: {:?}", e),
            }
            ()
        }
    }
    pub fn receive(socket: &mut TcpStream) {
        //let mut buf = [0; 68]; // NOTE: Change depending on size of client register packet
        let mut buf = [0; 1024];
        let data = socket.read(&mut buf[..]).expect("no data received");
        if data == 0 {
            println!("Connection closed from other side.\nClosing...");
            socket
                .shutdown(Shutdown::Both)
                .expect("shutdown call failed");
        } else {
        }
    }
}
