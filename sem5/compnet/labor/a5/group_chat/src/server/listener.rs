use super::server::*;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

impl Server {
    pub async fn listen(&self) {
        let listener = TcpListener::bind(std::format!("{}:{}", self.ip, self.port))
            .await
            .unwrap();

        println!(
            "Server: Listening on port {} for incoming TCP connections",
            self.port
        );

        loop {
            match listener.accept().await {
                Ok((mut stream, addr)) => self.receive(&mut stream).await,
                Err(e) => println!("connection failed: {}", e),
            }
        }
    }

    async fn receive(&self, socket: &mut TcpStream) {
        let mut buf = [0; 1024];
        loop {
            match socket.read(&mut buf).await {
                Ok(0) => break,
                Ok(b) => {
                    let buf_str = std::str::from_utf8(&buf[..b]).expect("invalid utf-8 sequence");

                    println!("Server: Received data: {:?}", buf_str);
                    self.handle_reception(buf_str.split_terminator('|').nth(0).unwrap())
                        .await;
                }
                Err(e) => panic!("encountered IO error: {}", e),
            }
        }
    }

    async fn send(self, addr: String, data: String) -> Result<(), ()> {
        if let Ok(mut stream) = TcpStream::connect(addr).await {
            stream.write_all(data.as_bytes()).await.unwrap();
        }
        Ok(())
    }

    async fn handle_reception(&self, msg: &str) {
        let parts: Vec<&str> = msg.split('|').collect();
        match parts[0] {
            "REGISTER" => {
                let client_list = self.client_list.lock().await;
                let registered_client = client_list.clients.last().unwrap();
                let mut msg = String::from("USERLIST|");
                client_list.clone().clients.into_iter().for_each(|c| {
                    msg.push_str(format!("{},{},{},", c.username, c.ip, c.udp_port).as_str())
                });

                msg.char_indices().next_back();
                msg.push_str("\0");

                self.clone()
                    .send(
                        format!("{}:{}", registered_client.ip, registered_client.udp_port),
                        msg,
                    )
                    .await
                    .unwrap();
            }

            _ => {}
        }
    }
}
