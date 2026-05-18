use std::{io::{Result, Error, ErrorKind, stdin, Write, Read}, net::TcpStream, sync::LazyLock, process};

use tokio::{io::{self, AsyncBufRead, AsyncBufReadExt, BufReader}, time::{timeout, Duration}};

use regex::Regex;

use crate::server::server::Server;

const MIN_PORT_NUM: u32 = 1;
const MAX_PORT_NUM: u32 = 65535;
const LIN_EXIT_CODE: i32 = 0x0100;

#[derive(Debug, Clone)]
pub struct ClientList {
    pub client_list: Vec::<Client>
}

impl PartialEq for ClientList {
    fn eq(&self, other: &Self) -> bool {  
        self.client_list == other.client_list
    }
}

impl ClientList {
    pub fn new() -> Self {
        ClientList { client_list: Vec::<Client>::new() }
    }
    pub async fn add_client(&mut self) {
        self.client_list.push(Client::new().await);
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    pub username:  String,
    pub ip:  String,
    pub server_port:  String,
    pub udp_port:  String,
}

impl PartialEq for Client {
    fn eq(&self, other: &Self) -> bool {
        
            self.username == other.username &&
            self.ip == other.ip &&
            self.server_port == other.server_port &&
            self.udp_port == other.udp_port
        
    }
}

impl Client {
    async fn new() -> Self {
        Option::expect(Self::register().await.unwrap_or(Some(Client { 
            username: String::from("default"), 
                ip: String::from("127.0.0.1"), 
                server_port: String::from("50000"), udp_port: String::from("123") })), 
            "Registering failed. Please try again")
    }

    async fn register() -> Result<Option<Self>> {
        let user_input = io::stdin(); 
        let mut reader = BufReader::new(user_input);

        let mut username = String::new();
        let mut ip = String::new();
        let mut udp_port = String::new();

        println!("Client: Please register yourself. Type '|' to escape."); 
        
        let titles = ["username", "IP address", "UDP port"];

        for (title, target) in titles
            .into_iter()
            .zip([&mut username, &mut ip, &mut udp_port]) {

                println!("Client: Please enter your {}:", title);

                timeout(Duration::from_secs(30), reader.read_line(target)).await
                    .map_err(|_| Error::new(ErrorKind::TimedOut, "registration timed out"))??;

                if !Self::validate_registration(title, target) {
                    println!("Error: Invalid {}", title);
                    //Err(Error::new(ErrorKind::InvalidInput, "-1"))
                }

                if target.contains('|') {
                    println!("Closing register process");
                    //process::exit(LIN_EXIT_CODE);
                    return Ok(None)
                }
                *target = target.trim().to_string();
            }

                Ok(Some(Client {
                    username,
                    ip,
                    server_port: String::from("50000"),
                    udp_port
                }))
            } 

    fn validate_registration(step: &str, target: &String) -> bool {
        match step {
            "username" => Regex::new(r"[a-zA-Z0-9_-]{3,20}").unwrap().is_match(target),
            "IP address" => Regex::new(
            r"\b(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\b").unwrap().is_match(target),
            "UDP port" => {

                let port = target.trim().parse::<u32>().unwrap();
                port.gt(&MIN_PORT_NUM) && port.lt(&MAX_PORT_NUM)
            } 
            _ => {
                println!("registration failed.");
                false
            }
        }
    }

    pub fn send(&self, msg: String, stream: &mut TcpStream) -> Result<()> {
        let mut pos = 0;
        let msg_bytes = msg.as_bytes();
        while pos <  msg_bytes.len() {
            let bytes_written = stream.write(&msg_bytes[pos..]).unwrap();
            pos += bytes_written;
        }
        Ok(())

    }


    pub fn connect_to_server(&self, server: &Server) -> Result<TcpStream> {
        let ip = &server.ip;
        let port = &server.port;
        let mut msg = String::new();
        println!("Client: Connecting to the server with {} on port {}", ip, port);
        
        if let Ok(mut stream) = TcpStream::connect(std::format!("{}:{}", ip, port)) {
            println!("Client: Connected sucessfully with IP {} and port {}", ip, port);
            self.send(self.udp_port.clone(), &mut stream).unwrap();
            //println!("Client: Sending message {}", msg);
            Ok(stream)
        } else {
            Err(Error::new(ErrorKind::ConnectionRefused, "Error: Connection to the server has been refused"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_default_works() {
        let default_client = Client::new();

        assert_eq!(default_client, 
            Client { username: String::from("default"), ip: String::from("127.0.0.1"), 
                server_port: String::from("50000"), udp_port: String::from("123") });
    }
}
