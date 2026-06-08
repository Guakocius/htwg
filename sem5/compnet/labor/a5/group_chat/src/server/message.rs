use super::server::*;

use tokio::{io::AsyncWriteExt, net::TcpStream};

impl Server {
    pub async fn send(stream: &mut TcpStream, msg: &str) -> Result<(), String> {
        match stream.write_all(msg.as_bytes()).await {
            Ok(_) => {
                println!("Server: Message sent successfully");
                Ok(())
            }
            Err(e) => {
                eprintln!("error sending message: {}", e);
                Err(format!("Failed to send message: {}", e))
            }
        }
    }

    pub async fn broadcast(self, msg: &str) -> Result<(), String> {
        let users_lock = self.client_list.lock().await;
        let mut errors = Vec::new();

        for user in &users_lock.clients {
            let addr = format!("{}:{}", user.ip, user.server_port);

            match TcpStream::connect(&addr).await {
                Ok(mut stream) => {
                    if let Err(e) = stream.write_all(msg.as_bytes()).await {
                        errors.push(format!("Failed to send to {}: {}", user.username, e));
                        eprintln!("error sending broadcast to {}: {}", user.username, e);
                    }
                }
                Err(e) => {
                    errors.push(format!("Failed to connect to {}: {}", user.username, e));
                    eprintln!("error connecting to {}: {}", user.username, e);
                }
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(format!("broadcast errors: {:?}", errors))
        }
    }
}
