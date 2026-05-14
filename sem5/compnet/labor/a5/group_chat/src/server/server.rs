use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

use crate::client::client::ClientList;

pub struct Server {
    pub ip: String,
    pub port: String,
    pub client_list: ClientList,
}

impl Server {
    pub fn new() -> Server {
        Server {
            ip: String::from("127.0.0.1"),
            port: String::from("5000"),
            client_list: ClientList::new(),
        }
    }

    pub fn listen(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(std::format!("{}:{}", self.ip, self.port))?;

        println!(
            "Listening on port {} for incoming TCP connections",
            self.port
        );

        for stream in listener.incoming() {
            println!("{:?}", stream);
            match stream {
                Ok(mut stream) => {
                    thread::spawn(move || Self::receive(&mut stream));
                }
                Err(e) => {
                    eprintln!("connection failed: {}", e);
                }
            }
        }
        match listener.accept() {
            Ok((mut _socket, addr)) => {
                println!("Incoming connection accepted: {:?}", addr);
                thread::spawn(move || Self::receive(&mut _socket));
            }
            Err(e) => eprintln!("couldn't get client: {:?}", e),
        }
        Ok(())
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
            println!("Received message: {} from {}", data, todo!())
        }
    }
}
