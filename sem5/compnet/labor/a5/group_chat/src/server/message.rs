use super::server::*;

use tokio::{io::AsyncWriteExt, net::TcpStream};

impl Server {
    pub async fn send(addr: String, msg: &str) -> Result<(), ()> {
        if let Ok(mut stream) = TcpStream::connect(addr).await {
            stream.write_all(msg.as_bytes()).await.unwrap();
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
