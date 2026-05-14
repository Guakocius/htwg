use std::io::{Result, Error, ErrorKind, stdin, Write, Read};
use std::net::TcpStream;
use std::process;

use regex::Regex;

use crate::Server;

const MIN_PORT_NUM: u32 = 1;
const MAX_PORT_NUM: u32 = 65535;
const LIN_EXIT_CODE: i32 = 0x0100;

#[derive(Debug, PartialEq)]
pub struct ClientList {
    pub client_list: Vec::<Client>
}

impl ClientList {
    pub fn new() -> Self {
        ClientList { client_list: Vec::<Client>::new() }
    }
    pub fn add_client(&mut self) {
        self.client_list.push(Client::new());
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Client {
    pub username: String,
    pub ip: String,
    pub server_port: String,
    pub udp_port: String,
}

impl Client {
    fn new() -> Self {
        Option::expect(Self::register().0, "Registering failed. Please try again")
    }

    fn register() -> (Option<Self>, Result<()>) {
        let user_input = stdin();
        let mut username = String::new();
        let mut ip = String::new();
        let mut udp_port = String::new();

        println!("Please register yourself. Type '|' to escape.");

        ["username", "IP address", "UDP port"]
            .into_iter()
            .zip([&mut username, &mut ip, &mut udp_port])
            .fuse()
            .for_each(|(k, v)| {
                println!("Please enter your {}:", k);
                user_input.read_line(v).expect("failed to readline");
                if v.chars().any(|c| c == '|') {
                    println!("Closing register process");
                    process::exit(LIN_EXIT_CODE);
                }
                *v = v.trim().to_string()
            });

        if !Regex::new(r"[a-zA-Z0-9_-]{3,20}")
            .unwrap()
            .is_match(&username)
        {
            println!(
                "Username must follow this RegEx convention: [a-zA-Z0-9_-]{{3,20}}"
            );
            (
                None,
                Err(Error::new(ErrorKind::InvalidInput, "-1"))
            )
        } else if !Regex::new(
            r"\b(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\b")
            .unwrap()
            .is_match(&ip) {
                println!("IP address must follow this RegEx convention:
                    \\b(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\\.){{3}}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\\b");
            (
                None,
                Err(Error::new(ErrorKind::InvalidInput, "-2")) 
            )
        } else if udp_port.trim().parse::<u32>().unwrap() < MIN_PORT_NUM && udp_port.trim().parse::<u32>().unwrap() > MAX_PORT_NUM {
                println!("UDP port number must follow this RegEx convention:
                    (6553[0-5]|655[0-2][0-9]|65[0-4][0-9]{{2}}|6[0-4][0-9]{{3}}|[1-5][0-9]{{4}}|[0-9]{{1,4}})");
         
            (
                None,
                Err(Error::new(ErrorKind::InvalidInput, "-3")) 
            )
        } else {
            (
                Some(Client {
                username: username,
                ip: ip,
                server_port: String::from("5000"),
                udp_port: udp_port
            }),
                Ok(())
            )
        }
    }

    fn send_udp(&self, port: String, stream: &mut TcpStream) -> Result<()> {
        let mut pos = 0;
        let port_bytes = port.as_bytes();
        while pos <  port_bytes.len() {
            let bytes_written = stream.write(&port_bytes[pos..]).unwrap();
            pos += bytes_written;
        }
        Ok(())

    }


    pub fn connect_to_server(&self, server: &Server) -> Result<TcpStream> {
        let ip = &server.ip;
        let port = &server.port;
        let mut msg = String::new();
        println!("Connecting to the server with {} on port {}", ip, port);
        
        if let Ok(mut stream) = TcpStream::connect(std::format!("{}:{}", ip, port)) {
            self.send_udp(self.udp_port.clone(), &mut stream).unwrap();
            println!("Sending message {}", msg);
            Ok(stream)
        } else {
            Err(Error::new(ErrorKind::ConnectionRefused, "Connection to the server has been refused"))
        }
    }
}
