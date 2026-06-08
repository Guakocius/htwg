use std::io::{Error, ErrorKind, Result};
use tokio::{io::AsyncWriteExt, net::TcpStream};

#[derive(Debug)]
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
    pub async fn add_client(&mut self, client: Client) {
        self.clients.push(client);
    }
}

#[derive(Debug)]
pub struct Client {
    pub username: String,
    pub ip: String,
    pub server_port: String,
    pub udp_port: String,
    pub stream: Option<TcpStream>,
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

    pub async fn send(&mut self, msg: String) -> Result<()> {
        if let Some(ref mut stream) = self.stream {
            let mut pos = 0;
            let msg_bytes = msg.as_bytes();

            while pos < msg_bytes.len() {
                let bytes_written = stream.write(&msg_bytes[pos..]).await?;
                if bytes_written == 0 {
                    return Err(Error::new(
                        ErrorKind::WriteZero,
                        "failed to write to socket",
                    ));
                }
                pos += bytes_written;
            }
            Ok(())
        } else {
            Err(Error::new(ErrorKind::NotConnected, "no active stream"))
        }
    }

    pub async fn send_to_stream(stream: &mut TcpStream, msg: String) -> Result<()> {
        let mut pos = 0;
        let msg_bytes = msg.as_bytes();

        while pos < msg_bytes.len() {
            let bytes_written = stream.write(&msg_bytes[pos..]).await?;

            if bytes_written == 0 {
                return Err(Error::new(
                    ErrorKind::WriteZero,
                    "failed to write to socket",
                ));
            }
            pos += bytes_written;
        }
        Ok(())
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
        assert_eq!(client.stream, None);
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
