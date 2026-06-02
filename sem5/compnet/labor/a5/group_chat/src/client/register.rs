use super::client::*;
use crate::utils::enums::SendKind;

use std::io::{Error, ErrorKind, Result};

use regex::Regex;

use tokio::{
    net::TcpStream,
    io::{self, AsyncReadExt, AsyncBufRead, AsyncBufReadExt, BufReader},
    time::{Duration, timeout},
    task
};

use crate::server::server::Server;

const MIN_PORT_NUM: u32 = 1;
const MAX_PORT_NUM: u32 = 65535;
const SERVER_IP: &str = "127.0.0.1";
const SERVER_PORT: &str = "5001";

impl Client {

        pub async fn register(server: &Server) -> Result<Option<Self>> {
        
        let user_input = io::stdin(); 
        let reader = BufReader::new(user_input);
        let client = Self::register_from(server, reader).await?.unwrap();

        let mut stream = Self::connect_to_server(&client, server).await?;
        let msg = std::format!("REGISTER|{}|{}|{}\0", client.username, client.ip, client.udp_port);

        Self::send(SendKind::Server, msg, &mut stream).await.unwrap();

        let (mut read, mut write) = stream.into_split();

        let listen_thread = task::spawn(async move { Self::recv(&mut read).await });
            
        Ok(Some(client))
    }

    async fn register_from<R>(server: &Server, mut reader: R) -> Result<Option<Self>>
        where R: AsyncBufRead + Unpin  {
        
        let mut username = String::new();
        let mut ip = String::new();
        let mut udp_port = String::new();

        println!("Client: Please register yourself. Type '|' to escape."); 
        
        let titles = ["username", "IP address", "UDP port"];

        let addr = format!("{}:{}", SERVER_IP, SERVER_PORT);

        for (title, target) in titles
            .into_iter()
            .zip([&mut username, &mut ip, &mut udp_port]) {

                println!("Client: Please enter your {}:", title);

                timeout(Duration::from_secs(30), reader.read_line(target)).await
                    .map_err(|_| Error::new(ErrorKind::TimedOut, "registration timed out"))??;

                
                if target.contains('|') {
                    println!("Closing connection");
                    return Ok(None)
                }
                *target = target.trim().to_string();

                

                if target.is_empty() {
                    Server::send(addr, &format!("ERROR|REGISTER expects {}", title));
                    return Ok(None)
                }
                if !Self::validate_registration(title, target) {
                    Server::send(addr, &format!("ERROR|Invalid {}", title));
                    return Ok(None)
                }
            }
            {
                let client_list = server.client_list.lock().await;
                if client_list.clients.iter().any(|c| c.username == username) {
                    Server::send(addr, &format!("ERROR|Nickname already registered"));
                    return Ok(None)
                }
            }

                let client = Client {
                    username,
                    ip,
                    server_port: String::from(SERVER_PORT),
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
