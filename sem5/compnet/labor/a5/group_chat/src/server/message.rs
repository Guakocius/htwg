use super::server::*;

use tokio::io::AsyncWriteExt;

impl Server {
    pub async fn send(msg: &str) -> Result<(), String> {
        Ok(())
    }

    pub async fn broadcast(self, msg: &str) -> Result<(), String> {
        let users_lock = self.client_list.lock().await;
        let mut errors = Vec::new();

        for user in &users_lock.clients {
            if let Some(ref shared_stream) = user.stream {
                let mut stream = shared_stream.lock().await;
                if let Err(e) = stream.write_all(msg.as_bytes()).await {
                    errors.push(format!("Failed broadcasting to {}: {:?}", user.username, e));
                }
            } else {
                errors.push(format!(
                    "No active connection handle found for client {}",
                    user.username
                ));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(format!("broadcast errors: {:?}", errors))
        }
    }
}
