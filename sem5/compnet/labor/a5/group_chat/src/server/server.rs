use crate::client::client::{Client, ClientList};

use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct Server {
    pub ip: String,
    pub port: String,
    pub client_list: Arc<Mutex<ClientList>>,
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
        let user = users_lock
            .clients
            .iter()
            .position(|u| u.username == username)
            .unwrap();

        let user = users_lock.clients.remove(user);
        self.clone().broadcast(&format!(
            "UPDATE|REMOVE|{}|{}|{}",
            user.username, user.ip, user.udp_port
        ));
        Some(user)
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
