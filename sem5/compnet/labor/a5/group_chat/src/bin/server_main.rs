use std::{io::Result, sync::Arc};

use tokio::sync::Mutex;

use group_chat::server::server::Server;

#[tokio::main]
async fn main() -> Result<()> {
    println!("starting server...");

    let server = Server::new();

    server.listen().await;

    println!(
        "Server with IP address {} on port {} initialized with client list {:?} and listening\n",
        server.ip, server.port, server.client_list
    );

    Ok(())
}
