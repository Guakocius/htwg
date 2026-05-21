use std::net::{TcpStream, UdpSocket};

use crate::server::server::Server;

struct Udp {
    addrs: Vec<String>,
    ports: Vec<String>,
}

impl Udp {
    async fn new() -> Self {}
    fn read_stream(stream: &mut TcpStream) -> (Vec<String>, Vec<String>) {
        let mut buf = [0; 1024];

        match stream.read(&mut but) {
            Ok(0) => break,
            Ok(b) => {
                let buf_str = std::str::from_utf8(&buf[..b]).expect("invalid utf-8 sequence");
            }
        }
    }
    async fn send_port_to_client() {}
}
