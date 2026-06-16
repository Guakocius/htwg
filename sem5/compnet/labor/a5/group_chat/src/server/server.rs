use crate::client::client::{Client, ClientList};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct Server {
    pub ip: String,
    pub port: String,
    pub client_list: Arc<Mutex<ClientList>>,
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}

impl Server {
    pub fn new() -> Self {
        Server {
            ip: String::from("127.0.0.1"),
            port: String::from("5001"),
            client_list: Arc::new(Mutex::new(ClientList::new())),
        }
    }
    pub async fn remove_user(&mut self, username: &str) -> Option<Client> {
        let mut users_lock = self.client_list.lock().await;
        if let Some(pos) = users_lock
            .clients
            .iter()
            .position(|u| u.username == username)
        {
            let user = users_lock.clients.remove(pos);
            let msg = format!(
                "UPDATE|REMOVE|{}|{}|{}\0",
                user.username, user.ip, user.udp_port
            );

            drop(users_lock);
            self.clone().broadcast(&msg).await;
            return Some(user);
        }
        None
    }

    pub async fn add_user(&mut self, client: Client) {
        let mut users_lock = self.client_list.lock().await;

        let msg = format!(
            "UPDATE|ADD|{}|{}|{}\0",
            client.username, client.ip, client.udp_port
        );

        users_lock.clients.push(client);
        drop(users_lock);
        self.clone().broadcast(&msg).await.unwrap();
    }

    pub async fn get_userlist(&self) -> String {
        let client_list = self.client_list.lock().await;
        let mut msg = String::from("USERLIST|");

        for (i, c) in client_list.clients.iter().enumerate() {
            if i > 0 {
                msg.push(';');
            }
            msg.push_str(&format!("{},{},{}", c.username, c.ip, c.udp_port))
        }
        msg.push('\0');
        msg
    }

    pub async fn client_exists(&self, username: &str) -> bool {
        let client_list = self.client_list.lock().await;
        client_list.clients.iter().any(|c| c.username == username)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::client::ClientList;

    #[tokio::test]
    async fn test_new() {
        let server = Server::new();
        let clients = server.client_list.lock().await;

        assert_eq!(server.ip, String::from("127.0.0.1"));
        assert_eq!(server.port, String::from("5001"));
        assert_eq!(
            *clients,
            ClientList {
                clients: Vec::new()
            }
        );
    }
}
