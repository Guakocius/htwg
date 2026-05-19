use std::net::{TcpListener, TcpStream};
use std::process;

use tokio::{join, task};

use crate::client::client::{Client, ClientList};
use crate::server::server::Server;

pub async fn run() -> Result<(), std::io::Error> {
    setup_system().await
}

async fn setup_system() -> Result<(), std::io::Error> {
    let mut server: Server = Server::new();
    println!(
        "Server with IP address {} on port {} initialized with client list {:?}\n",
        server.ip, server.port, server.client_list
    );

    server.client_list.add_client(&server.clone()).await;
    let client = server.client_list.client_list.last().cloned().unwrap();
    server.port = client.server_port.clone();

    println!(
        "\nNew client registered: {} with IP address: {} and UDP port {}\n",
        client.username, client.ip, client.udp_port
    );

    let server_clone = server.clone();
    let server_thread = task::spawn(async move { server_clone.listen().await });
    let server_clone = server.clone();

    let client_clone = client.clone();

    let client_thread = task::spawn(async move {
        let mut stream = client_clone.connect_to_server(&server_clone).await.unwrap();
        let mut msg = String::new();
        println!("Please enter something. Press '|' to exit");
        loop {
            msg.clear();
            std::io::stdin()
                .read_line(&mut msg)
                .expect("failed to readline");

            Client::send(msg.clone().trim().to_string(), &mut stream)
                .await
                .unwrap();
        }
    });

    let (result, _) = join!(
            biased;
            server_thread,
            client_thread
    );

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    /*#[test]
    fn run_succeeds() {
        let run_result = run();
        assert!(run_result.is_ok());
    }*/
}
