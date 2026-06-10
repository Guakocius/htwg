use super::client::*;

use std::io::{Error, ErrorKind, Result};
use regex::Regex;

use tokio::{
    io::{stdin, AsyncBufRead, AsyncBufReadExt, BufReader, AsyncWriteExt},
    time::{Duration, timeout},
};

use crate::server::server::Server;

const MIN_PORT_NUM: u32 = 1;
const MAX_PORT_NUM: u32 = 65535;
const SERVER_PORT: &str = "5001";

impl Client {

        pub async fn register(server: &Server) -> Result<Option<Self>> {
        
        let user_input = stdin(); 
        let reader = BufReader::new(user_input);

        match Self::register_from(server, reader).await? {
            Some(mut client) => match Self::connect_to_server(&client, server).await {
                Ok(mut stream) => {
                    let msg = format!("REGISTER|{}|{}|{}\0", client.username, client.ip, client.udp_port);

                    match stream.write_all(msg.as_bytes()).await {
                        Ok(_) => {
                            client.stream = Some(stream);
                            Ok(Some(client))
                        }
                        Err(e) => {
                            eprintln!("failed to send registration: {}", e);
                            Err(Error::other(format!("failed to send registration: {:?}", e)))
                        }
                    }
                }
                Err(e) => {
                    eprintln!("failed to connect to server: {}", e);
                    Err(e)
                }
            }
        None => Ok(None)
        }
    }

    async fn register_from<R>(server: &Server, mut reader: R) -> Result<Option<Self>>
        where R: AsyncBufRead + Unpin  {
        
        let mut username = String::new();
        let mut ip = String::new();
        let mut udp_port = String::new();
 
        let titles = ["username", "IP address", "UDP port"];

        for (title, target) in titles
            .into_iter()
            .zip([&mut username, &mut ip, &mut udp_port]) {

                println!("Client: Please enter your {}:", title);

                match timeout(Duration::from_secs(30), reader.read_line(target)).await {
                    Ok(Ok(_)) => {
                        if target.contains('|') {
                            println!("Registration cancelled");
                            return Ok(None);
                        }
                        *target = target.trim().to_string();

                        if target.is_empty() {
                            println!("Error: {} cannot be empty", title);
                            return Ok(None)
                        }

                        if !Self::validate_registration(title, target) {
                            eprintln!("Error: invalid {}", title);
                            return Ok(None)
                        }
                    }
                    Ok(Err(e)) => {
                        return Err(Error::other(format!("Read error: {:?}", e)));
                    }
                    Err(_) => {
                        return Err(Error::new(ErrorKind::TimedOut, "Registration timed out"));
                    }
                }
            }
                
        if server.client_exists(&username).await {
            eprintln!("Error: Nickname already registered");
            return Ok(None);
        }

        let client = Client {
            username,
            ip,
            server_port: String::from(SERVER_PORT),
            udp_port,
            stream: None
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
                eprintln!("registration failed.");
                false
            }
        }
    }
}
