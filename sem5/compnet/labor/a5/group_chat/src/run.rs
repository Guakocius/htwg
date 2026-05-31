use std::{process, sync::Arc};

use tokio::{
    io::{AsyncBufReadExt, BufReader},
    join,
    sync::Mutex,
    task,
};

use crate::client::client::{Client, ClientList};
use crate::server::server::Server;

pub async fn run() -> Result<(), std::io::Error> {
    setup_system().await
}

async fn setup_system() -> Result<(), std::io::Error> {
    let server = Arc::new(Mutex::new(Server::new()));
    /*println!(
        "Server with IP address {} on port {} initialized with client list {:?}\n",
        server.ip, server.port, server.client_list
    );*/

    let server_listener = server.clone();
    let server_thread = task::spawn(async move {
        let server_instance = {
            let server = server_listener.lock().await;
            server.clone()
        };

        server_instance.listen().await
    });

    {
        let server_lock = server.lock().await;
        let mut client_list = server_lock.client_list.lock().await;
        client_list.add_client(&server_lock).await;
    }

    let client = {
        let server_lock = server.lock().await;
        let client_list = server_lock.client_list.lock().await;
        client_list.clients.last().cloned().unwrap()
    };

    /*println!(
        "New client registered: {} with IP address: {} and UDP port {}\n",
        client.username, client.ip, client.udp_port
    );*/

    let client_clone = client.clone();
    let client_server = server.clone();

    let client_thread = task::spawn(async move {
        let mut stream = {
            let server_lock = client_server.lock().await;
            client_clone.connect_to_server(&server_lock).await.unwrap()
        };

        let mut buf_reader = BufReader::new(tokio::io::stdin());
        let mut msg = String::new();
        println!("Please enter something. Press '|' to exit");

        loop {
            msg.clear();
            buf_reader
                .read_line(&mut msg)
                .await
                .expect("failed to readline");

            if !msg.chars().any(|c| c == '|') {
                Client::send(msg.clone().trim().to_string(), &mut stream)
                    .await
                    .unwrap();
            } else {
                println!("Closing connection");
                process::exit(0x0100);
            }
        }
    });

    let _ = join!(biased; server_thread, client_thread);

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
