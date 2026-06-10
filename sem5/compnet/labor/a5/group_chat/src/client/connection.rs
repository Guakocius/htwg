use super::client::*;

use std::io::{Error, ErrorKind, Result};
use tokio::{
    io::AsyncReadExt,
    net::{TcpStream, tcp::OwnedReadHalf},
};

use crate::server::server::Server;

impl Client {
    pub async fn connect_to_server(&self, server: &Server) -> Result<TcpStream> {
        let addr = format!("{}:{}", server.ip, server.port);

        match TcpStream::connect(&addr).await {
            Ok(stream) => Ok(stream),
            Err(e) => {
                eprintln!("failed to connect to server at {}: {:?}", addr, e);
                Err(Error::new(
                    ErrorKind::ConnectionRefused,
                    format!("failed to connect to server: {}", e),
                ))
            }
        }
    }

    pub async fn recv(stream: &mut OwnedReadHalf) -> Result<String> {
        let mut buf = [0; 1024];

        match stream.read(&mut buf).await {
            Ok(0) => {
                println!("Server closed connection");
                Ok(String::new())
            }
            Ok(b) => {
                let buf_str = std::str::from_utf8(&buf[..b])
                    .map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid UTF-8"))?;

                let msg = buf_str.trim_matches('\0').to_string();
                Ok(msg)
            }
            Err(e) => {
                eprintln!("IO error: {:?}", e);
                Err(e)
            }
        }
    }

    pub async fn handle_message(msg: &str) -> Result<()> {
        let parts: Vec<&str> = msg.split('|').collect();
        println!("msg: {:?}", msg);

        if parts.is_empty() {
            return Err(Error::new(ErrorKind::InvalidData, "Empty message"));
        }

        match msg {
            "USERLIST" => {
                if parts.len() > 1 {
                    let users = &parts[1..];
                    for user_info in users {
                        if !user_info.is_empty() {
                            let user_parts: Vec<&str> = user_info.split(',').collect();
                            if user_parts.len() == 3 {
                                println!(
                                    "\t{} ({}, UDP Port: {})",
                                    user_parts[0], user_parts[1], user_parts[2]
                                );
                            }
                        }
                    }
                } else {
                    println!("no users connected");
                }
                Ok(())
            }
            "UPDATE" => {
                if parts.len() >= 5 {
                    let action = parts[1];
                    let username = parts[2];
                    let ip = parts[3];
                    let port = parts[4];

                    match action {
                        "ADD" => println!("new user connected: {} ({}:{})\n", username, ip, port),

                        "REMOVE" => println!("user disconnected: {} ({}:{})\n", username, ip, port),
                        _ => println!("unknown update action: {}", action),
                    }
                    Ok(())
                } else {
                    Err(Error::new(ErrorKind::InvalidData, "Invalid UPDATE format"))
                }
            }

            "BROADCAST" => {
                if parts.len() >= 3 {
                    let username = parts[1];
                    let msg = parts[2..].join("|");
                    println!("\n{}: {}\n", username, msg);
                    Ok(())
                } else {
                    Err(Error::new(
                        ErrorKind::InvalidData,
                        "Invalid BROADCAST format",
                    ))
                }
            }

            "LOGOUT_SUCCESS" => {
                println!("logged out successfully");
                Ok(())
            }

            "SUCCESS" => {
                println!("operation successful");
                if parts.len() > 1 {
                    println!("Details: {}", parts[1..].join("|"));
                }
                Ok(())
            }

            "ERROR" => {
                if parts.len() > 1 {
                    eprintln!("server error: {}", parts[1..].join("|"));
                } else {
                    eprintln!("server error");
                }
                Ok(())
            }

            "HANDSHAKE" => {
                if parts.len() >= 3 {
                    let username = parts[1];
                    let port = parts[2];
                    println!("handshake request from {} on TCP port {}\n", username, port);
                    Ok(())
                } else {
                    Err(Error::new(
                        ErrorKind::InvalidData,
                        "Invalid HANDSHAKE format",
                    ))
                }
            }

            "HANDSHAKE_RETURN" => {
                if parts.len() >= 2 {
                    let port = parts[1];
                    println!("handshake reply on TCP port: {}", port);
                    Ok(())
                } else {
                    Err(Error::new(
                        ErrorKind::InvalidData,
                        "Invalid HANDSHAKE_RETURN format",
                    ))
                }
            }

            "MSG" => {
                if parts.len() >= 2 {
                    let msg = parts[1..].join("|");
                    println!("direct message: {}\n", msg);
                    Ok(())
                } else {
                    Err(Error::new(ErrorKind::InvalidData, "Invalid MSG format"))
                }
            }

            _ => {
                eprintln!("unknown message type: {}", parts[0]);
                Ok(())
            }
        }
    }
}
