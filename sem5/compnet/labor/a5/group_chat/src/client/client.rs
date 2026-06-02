use std::io::Result;
use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::server::server::Server;
use crate::utils::enums::SendKind;

#[derive(Debug, Clone)]
pub struct ClientList {
    pub clients: Vec<Client>,
}

impl PartialEq for ClientList {
    fn eq(&self, other: &Self) -> bool {
        self.clients == other.clients
    }
}

impl ClientList {
    pub fn new() -> Self {
        ClientList {
            clients: Vec::<Client>::new(),
        }
    }
    pub async fn add_client(&mut self, server: &Server) {
        self.clients.push(Client::new(server).await);
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    pub username: String,
    pub ip: String,
    pub server_port: String,
    pub udp_port: String,
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
    async fn new(server: &Server) -> Self {
        Option::expect(
            Self::register(server).await.unwrap_or(Some(Client {
                username: String::from("default"),
                ip: String::from("127.0.0.1"),
                server_port: String::from("5001"),
                udp_port: String::from("123"),
            })),
            "Registering failed. Please try again",
        )
    }

    pub async fn send(send_kind: SendKind, msg: String, stream: &mut TcpStream) -> Result<()> {
        let mut pos = 0;
        let msg_bytes = msg.as_bytes();

        match send_kind {
            SendKind::Udp => {}
            SendKind::Tcp => {
                if let Ok()
            }
            SendKind::Server => {}
            _ => {}
        }

        while pos < msg_bytes.len() {
            let bytes_written = stream.write(&msg_bytes[pos..]).await?;
            pos += bytes_written;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::server::Server;

    #[tokio::test]
    async fn constructor_default_works() {
        let server = Server::new();
        let default_client = Client::new(&server).await;

        assert_eq!(
            default_client,
            Client {
                username: String::from("default"),
                ip: String::from("127.0.0.1"),
                server_port: String::from("5001"),
                udp_port: String::from("123")
            }
        );
    }
}
