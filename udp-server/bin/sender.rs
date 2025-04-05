use std::{
    io::{self, BufRead},
    net::UdpSocket,
    thread,
};

fn main() -> io::Result<()> {
    let address = "127.0.0.1:34255";
    let socket = UdpSocket::bind(address)?;
    let socket_receiver = socket.try_clone()?;
    println!("socket bind to address: {address}");

    thread::spawn(move || {
        let mut buf = [0; 2048];
        loop {
            let (amt, src) = socket_receiver.recv_from(&mut buf).expect("receive failed");
            println!(
                "Received: {:?} from {}",
                String::from_utf8(buf[..amt].to_vec()),
                src
            );
        }
    });

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        println!("Line read from stdin '{}'", line);
        if &line == "BYE" {
            break;
        }
        socket
            .send_to(line.as_bytes(), "127.0.0.1:34254")
            .expect("Error on send");
    }

    Ok(())
}
