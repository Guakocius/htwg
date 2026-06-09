use std::{io::Result, process};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, stdin};

use group_chat::client::client::Client;
use group_chat::server::server::Server;

#[tokio::main]
async fn main() -> Result<()> {
    println!("starting registering client...");

    let server = Server::new();

    let client = match Client::register(&server).await {
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

    println!(
        "New client registered: {} with IP address: {} and UDP port {} connected to server port {}\n",
        client.username, client.ip, client.udp_port, client.server_port
    );

    match client.connect_to_server(&server).await {
        Ok(mut stream) => {
            println!("connected to server on {}:{}", server.ip, server.port);

            let msg = format!(
                "REGISTER|{}|{}|{}\0",
                client.username, client.ip, client.udp_port
            );

            if let Err(e) = stream.write_all(msg.as_bytes()).await {
                eprintln!("failed sending registration: {:?}", e);
                return Err(e);
            }

            println!("REGISTER message sent.");
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

                        if msg.contains('|') {
                            println!("Closing connection");
                            stream.write_all("LOGOUT\0".as_bytes()).await.unwrap();
                            process::exit(0x0100);
                        }

                        msg.push('\0');
                        match stream.write_all(msg.as_bytes()).await {
                            Ok(_) => println!("msg: {}", msg),
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
    Ok(())
}
