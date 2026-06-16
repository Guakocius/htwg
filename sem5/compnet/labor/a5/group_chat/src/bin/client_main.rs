use std::{
    io::{BufRead, Result, Write, stdin, stdout},
    net::UdpSocket,
    process,
    sync::Arc,
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::Mutex,
    time::sleep,
};

use group_chat::{
    client::{client::Client, udp::send_handshake},
    input::{ClientCommand, connected_menu},
    server::server::Server,
};

#[tokio::main]
async fn main() -> Result<()> {
    let server = Server::new();

    let (client, stream) = Client::register(&server).await?;

    println!("Successfully authenticated to server");

    let shared_stream = Arc::new(Mutex::new(stream));
    let reader_stream = Arc::clone(&shared_stream);

    // Asynchronous Reader Task
    tokio::spawn(async move {
        let mut buf = [0; 1024];
        loop {
            let bytes_read = {
                let mut stream = reader_stream.lock().await;
                match stream.read(&mut buf).await {
                    Ok(0) => {
                        println!("\n[System]: Server closed connection.");
                        process::exit(0x0100);
                    }
                    Ok(b) => b,
                    Err(_) => break,
                }
            };
            let incoming = String::from_utf8_lossy(&buf[..bytes_read]);
            for chunk in incoming.split('\0').filter(|s| !s.is_empty()) {
                // If it's a userlist payload, format it clearly so it doesn't drown in the menu text
                if chunk.starts_with("USERLIST|") {
                    println!("\n=== ACTIVE ONLINE USERS ===");
                    let users = chunk.trim_start_matches("USERLIST|").replace(';', "\n* ");
                    if users.is_empty() {
                        println!("No other users registered.");
                    } else {
                        println!("* {}", users);
                    }
                    println!("===========================\n");
                } else {
                    println!("\n[Server Message]: {}", chunk);
                }
            }
        }
    });

    let udp_addr = format!("{}:{}", client.ip, client.udp_port);
    let socket = UdpSocket::bind(&udp_addr).expect("Unable to bind local UDP socket");

    loop {
        match connected_menu().await {
            Ok(ClientCommand::ListUsers) => {
                let payload = "USERLIST\0";
                let mut stream = shared_stream.lock().await;
                let _ = stream.write_all(payload.as_bytes()).await;

                // Give the background async reader task a split second to fetch and print
                // the payload before the next iteration reprints the main menu prompt.
                drop(stream);
                sleep(Duration::from_millis(150)).await;
            }
            Ok(ClientCommand::BroadcastMessage) => {
                print!("Enter text to broadcast (Enter '|' to return to options): ");
                Write::flush(&mut stdout())?;
                let mut msg = String::new();
                stdin().lock().read_line(&mut msg)?;
                let trimmed_msg = msg.trim();

                if trimmed_msg == "|" {
                    println!("Returning to options menu...");
                    continue;
                }

                if !trimmed_msg.is_empty() {
                    let payload = format!("BROADCAST|{}\0", trimmed_msg);
                    let mut stream = shared_stream.lock().await;
                    let _ = stream.write_all(payload.as_bytes()).await;
                }
            }
            Ok(ClientCommand::InitiateChat) => {
                print!("Enter Target Peer Destination IP: ");
                Write::flush(&mut stdout())?;
                let mut target_ip = String::new();
                stdin().lock().read_line(&mut target_ip)?;

                print!("Enter Target Peer UDP Port: ");
                Write::flush(&mut stdout())?;
                let mut target_port = String::new();
                stdin().lock().read_line(&mut target_port)?;

                let dst_addr = format!("{}:{}", target_ip.trim(), target_port.trim());
                send_handshake(&socket, &dst_addr, &client.username, "5002").await?;
            }
            Ok(ClientCommand::Logout) => {
                let mut stream = shared_stream.lock().await;
                let _ = stream.write_all("LOGOUT\0".as_bytes()).await;
                println!("Session terminated.");
                break;
            }
            Err(e) => {
                eprintln!("Menu read failure encounter: {:?}", e);
                break;
            }
        }
    }
    Ok(())
}
