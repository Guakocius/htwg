use std::{
    io::{Error, ErrorKind, Result},
    net::UdpSocket,
};

pub async fn send_handshake(
    socket: &UdpSocket,
    target: &str,
    username: &str,
    port: &str,
) -> Result<()> {
    let msg = format!("HANDSHAKE|{}|{}\0", username, port);
    socket.send_to(msg.as_bytes(), target)?;
    Ok(())
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

pub fn listen_udp(socket: UdpSocket) -> Result<()> {
    let mut buf = [0; 1024];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                let msg = String::from_utf8_lossy(&buf[..amt]);
                let msg = msg.trim_matches('\0');
                let parts: Vec<&str> = msg.split('|').collect();
                if parts.is_empty() {
                    continue;
                }
                if parts[0] == "HANDSHAKE" && parts.len() >= 3 {
                    println!(
                        "[UDP Handshake] Request received from user {} on port {}",
                        parts[1], parts[2]
                    );
                    let reply = format!("HANDSHAKE_RETURN|{}\0", parts[2]);
                    socket.send_to(reply.as_bytes(), src).unwrap();
                } else if parts[0] == "HANDSHAKE_RETURN" {
                    println!("[UDP Handshake] Direct negotiation successfully");
                }
            }
            Err(e) => return Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
