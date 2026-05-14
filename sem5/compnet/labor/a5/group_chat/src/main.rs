mod client;
mod server;

use std::net::{TcpListener, TcpStream};

use client::client::Client;
use server::server::Server;

fn main() {
    let mut server = Server::new();
    println!(
        "Server with IP address {} on port {} initialized with client list {:?}\n",
        server.ip, server.port, server.client_list
    );

    server.client_list.add_client();
    let client = server.client_list.client_list.iter().last().unwrap();
    server.port = client.clone().server_port;

    println!(
        "\nNew client registered: {} with IP address: {} and UDP port {}\n",
        client.username, client.ip, client.udp_port
    );

    server.listen().unwrap();
    client.connect_to_server(&server).unwrap();
    //Server::receive(&mut stream);
}
