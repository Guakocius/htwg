use std::{
    io::{Error, ErrorKind, Result},
    net::{SocketAddr, UdpSocket},
};

use tokio::task;

pub async fn handle_udp(ip: &str, port: &str) -> Result<UdpSocket> {
    let addr = format!("{}:{}", ip, port);
    match UdpSocket::bind(&addr) {
        Ok(socket) => {
            println!("UDP socket bound to {}", addr);
            Ok(socket)
        }
        Err(e) => {
            eprintln!("UDP failed to bind socket to {}", addr);
            Err(e)
        }
    }
}

pub async fn send_handshake(
    socket: &UdpSocket,
    target: &str,
    username: &str,
    port: &str,
) -> Result<()> {
    let msg = format!("HANDSHAKE|{}|{}\0", username, port);

    match socket.send_to(msg.as_bytes(), target) {
        Ok(b) => {
            println!("UDP handshake sent to {} ({} bytes)", target, b);
            Ok(())
        }
        Err(e) => {
            eprintln!("UDP failed sending handshake to {}: {:?}", target, e);
            Err(e)
        }
    }
}

pub async fn send_handshake_return(socket: &UdpSocket, target: &str, port: &str) -> Result<()> {
    let msg = format!("HANDSHAKE_RETURN|{}\0", port);

    match socket.send_to(msg.as_bytes(), target) {
        Ok(b) => {
            println!("UDP handshake return sent to {} ({} bytes)", target, b);
            Ok(())
        }
        Err(e) => {
            eprintln!("UDP failed sending handshake return to {}: {:?}", target, e);
            Err(e)
        }
    }
}

pub async fn recv_on_udp(socket: UdpSocket) -> Result<()> {
    let mut buf = [0; 1024];

    loop {
        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                if let Ok(msg) = std::str::from_utf8(&buf[..amt]) {
                    let msg = msg.trim_matches('\0');
                    println!("UDP received from {}: {}", src, msg);

                    if let Err(e) = handle_udp_message(msg, src).await {
                        eprintln!("UDP error handling message from {}: {:?}", src, e);
                    }
                }
            }
            Err(e) => {
                eprintln!("UDP receive error: {:?}", e);
                return Err(e);
            }
        }
    }
}

async fn handle_udp_message(msg: &str, src: SocketAddr) -> Result<()> {
    let parts: Vec<&str> = msg.split('|').collect();

    match parts.get(0).map(|s| *s) {
        Some("HANDSHAKE") => {
            if parts.len() >= 3 {
                let username = parts[1];
                let port = parts[2];
                println!(
                    "UDP handshake from {} (TCP port: {}) at {}",
                    username, port, src
                );
            } else {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "Invalid HANDSHAKE format",
                ));
            }
            Ok(())
        }

        Some("HANDSHAKE_RETURN") => {
            if parts.len() >= 2 {
                let port = parts[1];
                println!("UDP handshake return (TCP port: {}) from {}", port, src);
            } else {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "Invalid HANDSHAKE_RETURN format",
                ));
            }
            Ok(())
        }

        Some(u) => {
            eprintln!("Unknown UDP message type: {} from {}", u, src);
            Ok(())
        }

        None => Err(Error::new(ErrorKind::InvalidData, "Empty message received")),
    }
}

pub fn spawn_udp_listener(socket: UdpSocket) -> task::JoinHandle<Result<()>> {
    task::spawn(async move { recv_on_udp(socket).await })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_handle_udp_bind() {
        assert!(handle_udp("127.0.0.1", "0").await.is_ok());
    }

    #[test]
    fn test_handshake_msg_format() {
        let username = "test";
        let port = "5002";
        let msg = format!("HANDSHAKE|{}|{}\0", username, port);
        assert_eq!(msg, "HANDSHAKE|test|5002\0");
    }

    #[test]
    fn test_handshake_return_msg_format() {
        let port = "5002";
        let msg = format!("HANDSHAKE_RETURN|{}\0", port);

        assert_eq!(msg, "HANDSHAKE_RETURN|5002\0");
    }
}
