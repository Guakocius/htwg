use std::{
    io::{BufRead, Result, Write, stdin, stdout},
    net::UdpSocket,
    process,
    sync::Arc,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::Mutex,
};

use group_chat::{
    client::{
        client::Client,
        udp::{listen_udp, send_handshake},
    },
    server::server::Server,
};

#[tokio::main]
async fn main() -> Result<()> {
    let server = Server::new();

    let (client, stream) = Client::register(&server).await?;

    println!("Successfully authenticated to server");

    let shared_stream = Arc::new(Mutex::new(stream));
    let reader_stream = Arc::clone(&shared_stream);
    tokio::spawn(async move {
        let mut buf = [0; 1024];
        loop {
            let bytes_read = {
                let mut stream = reader_stream.lock().await;
                match stream.read(&mut buf).await {
                    Ok(0) => {
                        println!("Server closed connection.");
                        process::exit(0x0100);
                    }
                    Ok(b) => b,
                    Err(_) => break,
                }
            };
            let incoming = String::from_utf8_lossy(&buf[..bytes_read]);
            for chunk in incoming.split('\0').filter(|s| !s.is_empty()) {
                println!("[Server Message: ]{}", chunk);
            }
        }
    });

    let udp_addr = format!("{}:{}", client.ip, client.udp_port);
    let socket = UdpSocket::bind(&udp_addr).expect("unable to bind UDP");
    let socket_clone = socket.try_clone().expect("clone failed");

    std::thread::spawn(move || {
        listen_udp(socket_clone).unwrap();
    });

    loop {
        println!("Main Menu");
        println!("1. Send Broadcast Message");
        println!("2. Initiate UDP Peer Handshake");
        println!("3. Logout and Exit");
        print!("Please select an action (1-3)");

        Write::flush(&mut stdout())?;

        let mut choice = String::new();
        stdin().lock().read_line(&mut choice)?;

        match choice.trim() {
            "1" => {
                print!("Enter text to broadcast: ");
                Write::flush(&mut stdout())?;
                let mut msg = String::new();
                stdin().lock().read_line(&mut msg)?;
                let msg = msg.trim();

                let payload = format!("BROADCAST|{}\0", msg);
                let mut stream = shared_stream.lock().await;
                stream.write_all(payload.as_bytes()).await.unwrap();
            }
            "2" => {
                print!("Enter Target Peer Destination IP: ");
                Write::flush(&mut stdout())?;
                let mut target_ip = String::new();
                stdin().lock().read_line(&mut target_ip)?;

                print!("Enter Target Peer UDP Port: ");
                Write::flush(&mut stdout())?;
                let mut target_port = String::new();
                stdin().lock().read_line(&mut target_port)?;

                let dst_addr = format!("{}:{}", target_ip.trim(), target_port.trim());
                send_handshake(&socket, &dst_addr, &client.username, "5002").await;
            }
            "3" => {
                let mut stream = shared_stream.lock().await;
                stream.write_all("LOGOUT\0".as_bytes()).await.unwrap();
                println!("Session terminated.");
                break;
            }
            _ => println!("Invalid choice"),
        }
    }
    Ok(())
}
