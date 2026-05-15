mod client;
mod server;

use std::net::{TcpListener, TcpStream};
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

    println!("\nUpdated client list {:?}\n", server.client_list);
    let server_clone = server.clone();
    let server_thread = thread::spawn(move || server_clone.listen());
    let server_clone = server.clone();

    let client_thread_conn =
        thread::spawn(move || client.connect_to_server(&server_clone).unwrap());

    let server_thread_resp = server_thread.join();
    let client_thread_conn_resp = client_thread_conn.join();

    //thread::spawn(move || &server.listen().unwrap());
    //thread::spawn(move || client.connect_to_server(&server).unwrap());
    //Server::receive(&mut stream);
}
