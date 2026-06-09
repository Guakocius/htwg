use super::server::*;
use crate::client::client::Client;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    task,
};

impl Server {
    pub async fn listen(&mut self) {
        let listener = TcpListener::bind(format!("{}:{}", self.ip, self.port))
            .await
            .unwrap();

        println!(
            "Server: Listening on {}:{} for incoming TCP connections",
            self.ip, self.port
        );

        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    println!("new connection from {}", addr);
                    let server_clone = self.clone();

                    task::spawn(async move {
                        if let Err(e) = server_clone.handle_client(stream).await {
                            eprintln!("error on client {}: {}", addr, e);
                        }
                    });
                }
                Err(e) => println!("connection failed: {}", e),
            }
        }
    }

    async fn handle_client(self, mut stream: TcpStream) -> Result<(), String> {
        let mut buf = [0; 1024];
        let mut client: Option<Client> = None;
        let mut server = self;

        loop {
            match stream.read(&mut buf).await {
                Ok(0) => {
                    if let Some(ref c) = client {
                        println!("Server: Client {} has disconnected", c.username);

                        server.remove_user(&c.username).await.unwrap();
                    }
                    break;
                }
                Ok(b) => {
                    let buf_str = std::str::from_utf8(&buf[..b])
                        .map_err(|_| "invalid utf-8 sequence".to_string())?;

                    println!("Server: Received data: {:?}", buf_str);

                    let msg = buf_str.trim_matches('\0').trim();

                    if msg.is_empty() {
                        continue;
                    }

                    match server.handle_reception(msg, &mut stream, &mut client).await {
                        Ok(_) => { /*continue*/ }
                        Err(e) => {
                            eprintln!("error handling messsage: {}", e);
                            let msg = format!("ERROR|{}\0", e);

                            stream.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
                Err(e) => {
                    eprintln!("IO error: {}", e);
                    if let Some(ref c) = client {
                        server.remove_user(&c.username).await.unwrap();
                    }
                    return Err(format!("IO error: {}", e));
                }
            }
        }
        Ok(())
    }

    async fn handle_reception(
        &mut self,
        msg: &str,
        stream: &mut TcpStream,
        client: &mut Option<Client>,
    ) -> Result<(), String> {
        let parts: Vec<&str> = msg.split('|').collect();

        if parts.is_empty() || msg.is_empty() {
            return Err("INVALID_FORMAT".to_string());
        }

        match parts[0] {
            "REGISTER" => {
                if parts.len() != 4 {
                    return Err("INVALID_FORMAT".to_string());
                }

                if client.is_some() {
                    return Err("ALREADY_REGISTERED".to_string());
                }

                let username = parts[1];
                let ip = parts[2];
                let udp_port = parts[3];

                if udp_port.parse::<u32>().is_err() {
                    return Err("INVALID_PORT".to_string());
                }

                if self.client_exists(username).await {
                    return Err("NAME_ALREADY_USED".to_string());
                }

                let new_client = Client {
                    username: username.to_string(),
                    ip: ip.to_string(),
                    server_port: self.port.clone(),
                    udp_port: udp_port.to_string(),
                    stream: None,
                };

                let client_info = Client {
                    username: new_client.username.clone(),
                    ip: new_client.ip.clone(),
                    server_port: new_client.server_port.clone(),
                    udp_port: new_client.udp_port.clone(),
                    stream: None,
                };

                *client = Some(client_info);
                self.add_user(new_client).await;

                let userlist = self.get_userlist().await;
                stream
                    .write_all(userlist.as_bytes())
                    .await
                    .map_err(|e| format!("failed to send userlist: {}", e))?;

                println!("Server: User {} registered successfully", username);
                Ok(())
            }

            "LOGOUT" => {
                if let Some(c) = client.as_ref() {
                    self.remove_user(&c.username).await.unwrap();

                    stream
                        .write_all("LOGOUT_SUCCESS\0".as_bytes())
                        .await
                        .map_err(|e| format!("failed to successfully logout: {}", e))?;

                    println!("Server: User {} successfully logged out", c.username);

                    *client = None;
                    Ok(())
                } else {
                    Err("LOGOUT_FAILED".to_string())
                }
            }

            "BROADCAST" => {
                if parts.len() < 2 {
                    return Err("INVALID_BROADCAST_FORMAT".to_string());
                }
                if let Some(c) = client.as_ref() {
                    let msg = parts[1..].join("|");
                    let msg = format!("BROADCAST|{}|{}\0", c.username, msg);

                    if let Err(e) = self.clone().broadcast(&msg).await {
                        eprintln!("broadcast error: {:?}", e);
                    }

                    stream
                        .write_all("SUCCESS|MESSAGE_SENT\0".as_bytes())
                        .await
                        .map_err(|e| format!("failed to send success: {}", e))?;

                    Ok(())
                } else {
                    Err("LOGOUT_FAILED".to_string())
                }
            }
            _ => {
                eprintln!("unknown message type: {}", parts[0]);
                Err("INVALID_FORMAT".to_string())
            }
        }
    }
}
