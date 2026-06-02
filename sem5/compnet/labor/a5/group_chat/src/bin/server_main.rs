use std::{io::Result, sync::Arc};

use tokio::sync::Mutex;

use group_chat::server::server::Server;

#[tokio::main]
async fn main() -> Result<()> {
    println!("starting server...");

    let server = Server::new();

    server.listen().await;

    Ok(())
}
