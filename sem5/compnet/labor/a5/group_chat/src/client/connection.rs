use super::*;

use super::client::*;

use std::{
    io::{Error, ErrorKind, Result},
    net::{SocketAddr, TcpStream},
};

use crate::server::server::Server;

impl Client {
    pub async fn connect_to_server(&self, server: &Server) -> Result<TcpStream> {
        let ip = &server.ip;
        let port = &server.port;
        let mut msg = String::new();

        let mut addrs = Vec::<SocketAddr>::new();

        for i in 0..=10 {
            let port = port.parse::<u16>().unwrap() + i;
            addrs.push(SocketAddr::from(([127, 0, 0, 1], port)));
        }

        if let Ok(mut stream) = TcpStream::connect(&addrs[..]) {
            Self::send(self.udp_port.clone(), &mut stream)
                .await
                .unwrap();
            Ok(stream)
        } else {
            Err(Error::new(
                ErrorKind::ConnectionRefused,
                "Error: Connection to the server has been refused",
            ))
        }
    }
}
