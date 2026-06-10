use std::{io::Result, process};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, stdin};

use group_chat::client::client::Client;
use group_chat::server::server::Server;

#[tokio::main]
async fn main() -> Result<()> {
    let mut server = Server::new();

    let mut client = match Client::register(&server).await {
        Ok(Some(c)) => c,
        Ok(None) => {
            eprintln!("registering failed...");
            return Ok(());
        }
        Err(e) => {
            println!("registering failed: {}", e);
            return Err(e);
        }
    };

    loop {
        match client.connect_to_server(&server).await {
            Ok(mut stream) => {
                let msg = format!(
                    "REGISTER|{}|{}|{}\0",
                    client.username, client.ip, client.udp_port
                );

                if let Err(e) = stream.write_all(msg.as_bytes()).await {
                    eprintln!("failed sending registration: {:?}", e);
                    return Err(e);
                }

                println!("Please enter something. Press'|' to exit.");

                let mut reader = BufReader::new(stdin());
                let mut msg = String::new();

                loop {
                    msg.clear();
                    match reader.read_line(&mut msg).await {
                        Ok(_) => {
                            msg = msg.trim().to_string();

                            if msg.is_empty() {
                                continue;
                            }

                            if msg == "|" {
                                println!("Closing connection");
                                stream.write_all("LOGOUT\0".as_bytes()).await.unwrap();
                                server.remove_user(&client.username).await;
                                process::exit(0x0100);
                            }

                            msg.push('\0');
                            //Client::handle_message(&msg).await.unwrap();
                            /*let mut client = Client {
                                username: client.username.clone(),
                                ip: client.ip.clone(),
                                server_port: client.server_port.clone(),
                                udp_port: client.udp_port.clone(),
                                stream: client.stream,
                            };*/

                            match client.send(msg.clone()).await {
                                Ok(_) => {}
                                Err(e) => eprintln!("failed sending message: {:?}", e),
                            }
                        }
                        Err(e) => {
                            eprintln!("read error: {:?}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("failed connecting to server: {:?}", e);
                eprintln!(
                    "Make sure the server is running on {}:{}",
                    server.ip, server.port
                );
                return Err(e);
            }
        }
    }
}
