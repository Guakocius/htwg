use std::{io::Result, process};
use tokio::io::{AsyncBufReadExt, BufReader, stdin};

use group_chat::client::client::Client;
use group_chat::server::server::Server;

#[tokio::main]
async fn main() -> Result<()> {
    println!("starting registering client...");

    let server = Server::new();

    let client = match Client::register(&server).await {
        Ok(Some(c)) => c,
        _ => {
            eprintln!("registering failed...");
            return Ok(());
        }
    };

    println!(
        "New client registered: {} with IP address: {} and UDP port {}\n",
        client.username, client.ip, client.udp_port
    );

    let mut stream = client
        .connect_to_server(&server)
        .await
        .expect("connection failed");

    let mut buf_reader = BufReader::new(stdin());
    let mut msg = String::new();

    println!("Please enter something. Press '|' to exit");

    loop {
        msg.clear();
        buf_reader
            .read_line(&mut msg)
            .await
            .expect("failed to readline");

        msg = msg.trim().to_string();
        msg.push('\0');

        println!("msg: {msg}");

        if !msg.contains('|') {
            Client::send(msg.clone(), &mut stream).await.unwrap();
        } else {
            println!("Closing connection");
            process::exit(0x0100);
        }
    }
}
