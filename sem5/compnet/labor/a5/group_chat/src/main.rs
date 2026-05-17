mod client;
mod server;

use std::net::{TcpListener, TcpStream};
use std::process;
use std::thread;

use client::client::{Client, ClientList};
use server::server::Server;

fn main() {
    let mut server: Server = Server::new();
    println!(
        "Server with IP address {} on port {} initialized with client list {:?}\n",
        server.ip, server.port, server.client_list
    );

    server.client_list.add_client();
    let client = server.client_list.client_list.last().cloned().unwrap();
    server.port = client.server_port.clone();

    println!(
        "\nNew client registered: {} with IP address: {} and UDP port {}\n",
        client.username, client.ip, client.udp_port
    );

    let server_clone = server.clone();
    let server_thread = thread::spawn(move || server_clone.listen());
    let server_clone = server.clone();

    let client_clone = client.clone();

    let client_thread = thread::spawn(move || {
        let mut stream = client_clone.connect_to_server(&server_clone).unwrap();
        let mut msg = String::new();
        println!("Please enter something. Press '|' to exit");
        loop {
            msg.clear();
            std::io::stdin()
                .read_line(&mut msg)
                .expect("failed to readline");

            client
                .send(msg.clone().trim().to_string(), &mut stream)
                .unwrap();
        }
    });

    server_thread.join().unwrap();
    client_thread.join().unwrap();
}
