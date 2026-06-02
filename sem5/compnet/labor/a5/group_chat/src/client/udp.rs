use std::{
    io::Result,
    net::{SocketAddr, TcpStream, UdpSocket},
};

use crate::server::server::Server;

/*fn read_stream(stream: &mut TcpStream) -> (Vec<String>, Vec<String>) {
    let mut buf = [0; 1024];

    match stream.read(&mut but) {
        Ok(0) => break,
        Ok(b) => {
            let buf_str = std::str::from_utf8(&buf[..b]).expect("invalid utf-8 sequence");
        }
    }
}
async fn send_port_to_client() {}
**/
pub async fn handle_udp(ip: &str, port: &str) -> UdpSocket {
    UdpSocket::bind(format!("{}:{}", ip, port)).unwrap()
}

async fn send(socket: TcpStream, addr: SocketAddr, msg: &str) -> Result<()> {
    {
        let socket = UdpSocket::bind("127.0.0.1:34254").expect("couldn't bind address");

        let mut buf = [0; 1024];
        let (amt, src) = socket.recv_from(&mut buf).expect("couldn't receive data");

        let buf = &mut buf[..amt];
        buf.reverse();
        socket
            .send_to(buf, addr)
            .expect("couldn't send data to the socket");
    }

    Ok(())
}
