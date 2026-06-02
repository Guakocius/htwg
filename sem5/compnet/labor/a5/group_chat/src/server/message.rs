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
            }
        }
        Ok(())
    }

    pub async fn broadcast(self, msg: &str) -> Result<(), ()> {
        let users_lock = self.client_list.lock().await;

        for user in &users_lock.clients {
            if let Err(e) = Self::send(format!("{}:{}", user.ip, user.server_port), msg).await {
                eprintln!("error in sending broadcast to {}: {:?}", user.username, e);
            }
        }
        Ok(())
    }
}
