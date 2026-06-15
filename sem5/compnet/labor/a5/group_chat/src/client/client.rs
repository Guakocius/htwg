use std::sync::Arc;
use tokio::{net::TcpStream, sync::Mutex};

#[derive(Debug, Clone)]
pub struct ClientList {
    pub clients: Vec<Client>,
}

impl PartialEq for ClientList {
    fn eq(&self, other: &Self) -> bool {
        if self.clients.len() != other.clients.len() {
            return false;
        }
        self.clients
            .iter()
            .zip(other.clients.iter())
            .all(|(c1, c2)| c1 == c2)
    }
}

impl Default for ClientList {
    fn default() -> Self {
        ClientList::new()
    }
}

impl ClientList {
    pub fn new() -> Self {
        ClientList {
            clients: Vec::<Client>::new(),
        }
    }
    pub async fn add_client(&mut self, client: Client) {
        self.clients.push(client);
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    pub username: String,
    pub ip: String,
    pub server_port: String,
    pub udp_port: String,
    pub stream: Option<Arc<Mutex<TcpStream>>>,
}

impl PartialEq for Client {
    fn eq(&self, other: &Self) -> bool {
        self.username == other.username
            && self.ip == other.ip
            && self.server_port == other.server_port
            && self.udp_port == other.udp_port
    }
}

impl Client {
    pub fn new(username: String, ip: String, server_port: String, udp_port: String) -> Self {
        Client {
            username,
            ip,
            server_port,
            udp_port,
            stream: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clientlist_new() {
        let client_list = ClientList::new();
        assert_eq!(client_list.clients.len(), 0);
    }

    #[test]
    fn test_client_new() {
        let client = Client::new(
            String::from("test"),
            String::from("127.0.0.1"),
            String::from("5001"),
            String::from("5002"),
        );

        assert_eq!(client.username, "test");
        assert_eq!(client.ip, "127.0.0.1");
        assert_eq!(client.server_port, "5001");
        assert_eq!(client.udp_port, "5002");
        assert!(client.stream.is_none());
    }

    #[test]
    fn test_client_equality() {
        let client1 = Client::new(
            String::from("user1"),
            String::from("192.168.1.1"),
            String::from("5001"),
            String::from("5002"),
        );

        let client2 = Client::new(
            String::from("user1"),
            String::from("192.168.1.1"),
            String::from("5001"),
            String::from("5002"),
        );

        assert_eq!(client1, client2);
    }
}
