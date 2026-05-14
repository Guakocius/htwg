mod client;
mod server;

use client::client::Client;

fn main() {
    let c = Client::new();
    println!(
        "New client registered: {} with IP address: {} and UDP port {}",
        c.username, c.ip, c.port
    );
}
