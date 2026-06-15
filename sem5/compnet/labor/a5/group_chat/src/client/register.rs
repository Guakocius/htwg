use super::client::*;
use crate::server::server::Server;

use std::io::{stdin, stdout, Write, BufRead, Error, ErrorKind, Result};
use regex::Regex;

use tokio::{io::AsyncWriteExt, net::TcpStream};

const MIN_PORT_NUM: u32 = 1;
const MAX_PORT_NUM: u32 = 65535;
const SERVER_PORT: &str = "5001";

impl Client {

        pub async fn register(server: &Server) -> Result<(Self, TcpStream)> {
            let mut username = String::new();
            let mut ip = String::new();
            let mut udp_port = String::new();
        
            print!("Enter username: ");
            Write::flush(&mut stdout())?;
            stdin().lock().read_line(&mut username)?;
            let username = username.trim().to_string();

            print!("Enter IP address [default is 127.0.0.1]: ");
            Write::flush(&mut stdout())?;
            stdin().lock().read_line(&mut ip)?;
            let mut ip = ip.trim().to_string();
            if ip.is_empty() {
                ip = "127.0.0.1".to_string();
            }

            print!("Enter UDP port: ");
            Write::flush(&mut stdout())?;
            stdin().lock().read_line(&mut udp_port)?;
            let udp_port = udp_port.trim().to_string();

            let server_addr = format!("{}:{}", server.ip, server.port);
            let mut stream = TcpStream::connect(&server_addr).await?;

            let payload = format!("REGISTER|{}|{}|{}\0", username, ip, udp_port);
            stream.write_all(payload.as_bytes()).await?;

            let client = Client {
                username,
                ip,
                server_port: server.port.clone(),
                udp_port,
                stream: None,
            };

            Ok((client, stream))

            } 

    fn validate_registration(step: &str, target: &str) -> bool {
        match step {
            "username" => Regex::new(r"[a-zA-Z0-9_-]{3,20}").unwrap().is_match(target),
            "IP address" => Regex::new(
            r"\b(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\b").unwrap().is_match(target),
            "UDP port" => {

                let port = target.trim().parse::<u32>().unwrap();
                port.gt(&MIN_PORT_NUM) && port.lt(&MAX_PORT_NUM)
            } 
            _ => {
                eprintln!("registration failed.");
                false
            }
        }
    }
}
