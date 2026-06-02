use super::*;

use super::client::*;

use crate::utils::enums::SendKind;

use std::io::{Error, ErrorKind, Result};
use tokio::{io::AsyncReadExt, net::{TcpStream, tcp::OwnedReadHalf}};

use crate::server::server::Server;

impl Client {
    pub async fn connect_to_server(&self, server: &Server) -> Result<TcpStream> {
        let addr = format!("{}:{}", server.ip, server.port);

        let stream = TcpStream::connect(addr).await?;
        println!("Client: connected to server successfully");

        Ok(stream)
    }

    pub async fn recv(stream: &mut OwnedReadHalf) -> Result<()> {
        let mut buf = [0; 1024];

        match stream.read(&mut buf).await {
            Ok(0) => {
                println!("Server closed connection");
                return Ok(())
            }
            Ok(b) => {
                let mut buf_str = std::str::from_utf8(&buf[..b]).expect("invalid utf-8 sequence");
                let parts = buf_str.split('|');
                Ok(())
            }
        }
    }

    async fn handle_message(send_kind: SendKind, msg: &str) -> Result<()> {
        match send_kind {
            SendKind::Server => {
                let parts = msg.split('|');
                let cmd = parts[0];

                match cmd {
                    "USERLIST" => {
                        
                    }
                    "UPDATE" => {

                    }
                    "BROADCAST" => {}
                    "LOGOUT_SUCCESS" => {}
                    "ERROR" => {}
                    _ => println!("unknown message type: {}", cmd);

                }
            }
        }
    }
}
