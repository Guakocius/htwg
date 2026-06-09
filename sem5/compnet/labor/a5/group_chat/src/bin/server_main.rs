use std::{io::Result, sync::Arc};

use tokio::sync::Mutex;

use group_chat::server::server::Server;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting server...");

    let mut server = Server::new();

    println!("Starting server on {}:{}", server.ip, server.port);
    println!("Waiting for incoming TCP connections...");

    server.listen().await;

    Ok(())
}
