use crate::client::client::ClientList;

#[derive(Debug, Clone)]
pub struct Server {
    pub ip: String,
    pub port: String,
    pub client_list: ClientList,
}

impl Server {
    pub fn new() -> Self {
        Server {
            ip: String::from("127.0.0.1"),
            port: String::from("5000"),
            client_list: ClientList::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::client::{Client, ClientList};

    #[test]
    fn test_new() {
        let server = Server::new();
        let client_list = ClientList::new();

        assert_eq!(server.ip, String::from("127.0.0.1"));
        assert_eq!(server.port, String::from("5000"));
        assert_eq!(
            server.client_list,
            ClientList {
                client_list: client_list.client_list
            }
        );
    }
}
