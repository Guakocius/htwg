use super::server::*;

use crate::client::client::Client;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    task,
};

impl Server {
    pub async fn listen(&self) {
        let listener = TcpListener::bind(std::format!("{}:{}", self.ip, self.port))
            .await
            .unwrap();

        println!(
            "Server: Listening on {}:{} for incoming TCP connections",
            self.ip, self.port
        );

        loop {
            match listener.accept().await {
                Ok((mut stream, _addr)) => self.recv(&mut stream).await,
                Err(e) => println!("connection failed: {}", e),
            }
        }
    }

    //async fn handle_tcp(&self) -> Result<()> {}

    async fn recv(&mut self, socket: &mut TcpStream, client: Client) {
        let mut buf = [0; 1024];
        loop {
            match socket.read(&mut buf).await {
                Ok(0) => {
                    self.remove_user(&client.username);
                    break;
                }
                Ok(b) => {
                    let buf_str = std::str::from_utf8(&buf[..b]).expect("invalid utf-8 sequence");

                    println!("Server: Received data: {:?}", buf_str);
                    self.handle_reception(buf_str.trim_matches('\0'), client)
                        .await;
                    break;
                }
                Err(e) => panic!("encountered IO error: {}", e),
            }
        }
    }

    async fn handle_reception(&mut self, msg: &str, client: Client) {
        let parts: Vec<&str> = msg.split('|').collect();
        match parts[0] {
            "REGISTER" => {
                let mut msg = String::from("USERLIST|");
                {
                    let client_list = self.client_list.lock().await;
                    client_list.clients.iter().for_each(|c| {
                        msg.push_str(&format!("{},{},{};", c.username, c.ip, c.udp_port))
                    });
                }
                if msg.ends_with(';') {
                    msg.pop();
                }
                msg.push('\0');
                println!("sending userlist back to client");

                Self::send(format!("{}:{}", client.ip, client.server_port), &msg).await;

                let msg = format!("UPDATE|ADD|{}|{}|{}", parts[1], parts[2], parts[3]);

                if let Err(e) = self.clone().broadcast(&msg).await {
                    eprintln!("ERROR: unable to send update to clients: {:?}", e);
                }
            }

            "LOGOUT" => match self.clone().remove_user(&client.username).await {
                None => {
                    Self::send(
                        format!("{}:{}", client.ip, client.server_port),
                        "ERROR|Connection is not registered",
                    );
                }
                Some(_) => {
                    Self::send(
                        format!("{}:{}", client.ip, client.server_port),
                        "LOGOUT_SUCCESS",
                    );
                }
            },

            "BROADCAST" => self.broadcast()

            _ => println!("unknown message type: {}", parts[0]),
        }

        self.remove_user(&client.username);
    }
}
