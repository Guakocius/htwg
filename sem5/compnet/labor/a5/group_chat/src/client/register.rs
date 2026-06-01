use super::client::*;

use std::io::{Error, ErrorKind, Result};

use regex::Regex;

use tokio::{
    net::TcpStream,
    io::{self, AsyncReadExt, AsyncBufRead, AsyncBufReadExt, BufReader},
    time::{Duration, timeout},
};

use crate::server::server::Server;

const MIN_PORT_NUM: u32 = 1;
const MAX_PORT_NUM: u32 = 65535;

impl Client {

        pub async fn register(server: &Server) -> Result<Option<Self>> {
        
        let user_input = io::stdin(); 
        let reader = BufReader::new(user_input);
        let client = Self::register_from(reader).await?.unwrap();

        let mut stream = Self::connect_to_server(&client, server).await?;
        let msg = std::format!("REGISTER|{}|{}|{}\0", client.username, client.ip, client.udp_port);

        Self::send(msg, &mut stream).await.unwrap();

        let mut buf = [0; 1024];

            match stream.read(&mut buf).await {
                Ok(0) => {
                    println!("Client: Server closed connection");
                    return Ok(None);
                }
                Ok(b) => {
                    let buf_str = std::str::from_utf8(&buf[..b]).expect("invalid utf-8 sequence");

                    println!("Client: Received server response: {:?}", buf_str);
                }
                Err(e) => panic!("encountered IO error: {}", e),
            }

        Ok(Some(client))
    }

    async fn register_from<R>(mut reader: R) -> Result<Option<Self>>
        where R: AsyncBufRead + Unpin  {
        
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

                
                if target.contains('|') {
                    return Ok(None)
                }
                *target = target.trim().to_string();

                if !Self::validate_registration(title, target) {
                    return Err(Error::new(ErrorKind::InvalidInput, "-1"));
                }
            }

                let client = Client {
                    username,
                    ip,
                    server_port: String::from("5001"),
                    udp_port
                };

                Ok(Some(client))
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
                println!("registration failed.");
                false
            }
        }
    }
}
