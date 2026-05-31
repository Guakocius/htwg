use super::*;

use super::client::*;

use std::{
    io::{Error, ErrorKind, Read, Result},
    net::{SocketAddr, TcpListener},
};
use tokio::net::TcpStream;

use crate::server::server::Server;

impl Client {
    pub async fn connect_to_server(&self, server: &Server) -> Result<TcpStream> {
        let addr = format!("{}:{}", server.ip, server.port);

        let stream = TcpStream::connect(addr).await?;
        println!("Client: connected to server successfully");

        Ok(stream)
    }
}
