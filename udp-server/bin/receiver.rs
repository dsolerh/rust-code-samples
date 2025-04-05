use std::{io, net::UdpSocket};

fn main() -> io::Result<()> {
    let address = "127.0.0.1:34254";
    let socket = UdpSocket::bind(address)?;
    println!("socket bind to address: {address}");

    let mut buf = [0; 2048];
    loop {
        // Receives a single datagram message on the socket. If `buf` is too small to hold
        // the message, it will be cut off.
        let (amt, src) = socket.recv_from(&mut buf)?;
        println!(
            "message received: {:?} from {src}",
            String::from_utf8(buf[..amt].to_vec())
        );

        // Redeclare `buf` as slice of the received data and send reverse data back to origin.
        let buf = &mut buf[..amt];
        buf.reverse();
        socket.send_to(buf, &src)?;
    }
}
