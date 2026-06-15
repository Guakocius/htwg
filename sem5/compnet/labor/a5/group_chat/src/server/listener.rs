use super::server::*;
use crate::client::client::Client;
use std::sync::Arc;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::Mutex,
    task,
};

impl Server {
    pub async fn listen(&mut self) {
        let listener = TcpListener::bind(format!("{}:{}", self.ip, self.port))
            .await
            .unwrap();

        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    let server_clone = self.clone();

                    task::spawn(async move {
                        if let Err(e) = server_clone.handle_client(stream).await {
                            eprintln!("error on client {}: {}", addr, e);
                        }
                    });
                }
                Err(e) => eprintln!("connection failed: {}", e),
            }
        }
    }

    async fn handle_client(self, stream: TcpStream) -> Result<(), String> {
        let mut buf = [0; 1024];
        let shared_stream = Arc::new(Mutex::new(stream));
        let mut client_name: Option<String> = None;
        let mut server = self;

        loop {
            let bytes_read = {
                let mut locked_stream = shared_stream.lock().await;
                match locked_stream.read(&mut buf).await {
                    Ok(0) => break,
                    Ok(b) => b,
                    Err(e) => return Err(format!("Socket read error: {:?}", e)),
                }
            };

            let data = String::from_utf8_lossy(&buf[..bytes_read]);
            let packets: Vec<&str> = data.split('\0').filter(|s| !s.is_empty()).collect();

            for packet in packets {
                let parts: Vec<&str> = packet.split('|').map(|s| s.trim()).collect();
                if parts.is_empty() {
                    continue;
                }

                match parts[0] {
                    "REGISTER" => {
                        if parts.len() < 4 {
                            let mut s = shared_stream.lock().await;
                            let _ = s.write_all("ERROR|INVALID_FORMAT\0".as_bytes()).await;
                            continue;
                        }
                        let username = parts[1].to_string();
                        let ip = parts[2].to_string();
                        let udp_port = parts[3].to_string();

                        if server.client_exists(&username).await {
                            let mut s = shared_stream.lock().await;
                            s.write_all("ERROR|NAME_ALREADY_USED\0".as_bytes())
                                .await
                                .unwrap();
                            continue;
                        }

                        let new_client = Client {
                            username: username.clone(),
                            ip,
                            server_port: server.port.clone(),
                            udp_port,
                            stream: Some(Arc::clone(&shared_stream)),
                        };

                        client_name = Some(username.clone());
                        server.add_user(new_client).await;

                        // Respond immediately with userlist state confirmation
                        let userlist = server.get_userlist().await;
                        let mut s = shared_stream.lock().await;
                        s.write_all(userlist.as_bytes()).await.unwrap();
                    }
                    "LOGOUT" => {
                        if let Some(ref name) = client_name {
                            server.remove_user(name).await;
                            let mut s = shared_stream.lock().await;
                            s.write_all("LOGOUT_SUCCESS\0".as_bytes()).await.unwrap();
                        }
                        return Ok(());
                    }
                    "BROADCAST" => {
                        if parts.len() < 2 {
                            let mut s = shared_stream.lock().await;
                            s.write_all("ERROR|INVALID_BROADCAST_FORMAT\0".as_bytes())
                                .await
                                .unwrap();
                            continue;
                        }
                        if let Some(ref name) = client_name {
                            let payload = parts[1..].join("|");
                            let msg = format!("BROADCAST|{}|{}\0", name, payload);
                            server.clone().broadcast(&msg).await.unwrap();
                        }
                    }
                    _ => {
                        let mut s = shared_stream.lock().await;
                        s.write_all("ERROR|INVALID_FORMAT\0".as_bytes())
                            .await
                            .unwrap();
                    }
                }
            }
        }
        if let Some(ref name) = client_name {
            server.remove_user(name).await;
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
            _ => Err("INVALID_FORMAT".to_string()),
        }
    }
}
