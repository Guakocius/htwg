mod client;
mod server;

use client::client::Client;

use server::server::Server;

fn main() {
    let mut server = Server::new();
    println!(
        "Server with IP address {} on port {} initialized with client list {:?}",
        server.ip, server.port, server.client_list
    );

    server.client_list.add_client();

    println!(
        "New client registered: {} with IP address: {} and UDP port {}",
        server
            .client_list
            .client_list
            .iter()
            .last()
            .unwrap()
            .username,
        server.client_list.client_list.iter().last().unwrap().ip,
        server.client_list.client_list.iter().last().unwrap().port
    );
    println!(
        "Server with IP address {} on port {} initialized with client list {:?}",
        server.ip, server.port, server.client_list
    );
}
