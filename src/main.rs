pub mod decoder;

use clap::Parser;
use std::net::UdpSocket;

/// Command line arguments.
#[derive(Parser)]
struct Cli {
    /// The IP address to read from
    ip_address: String,
    /// The port number to read from
    port_number: u32,
}

/*
 * Connect to socket and receive DIS messages.
 */
fn main() {
    let args = Cli::parse();

    println!(
        "IP: {}, Port: {}",
        args.ip_address, args.port_number
    );

    let addr = format!("{}:{}", args.ip_address, args.port_number);

    println!("Create UDP socket {}", addr);

    let socket = UdpSocket::bind(addr).expect("Could not bind to address");
    let mut buf = [0; 1000];

    loop {
        // Receives a datagram message on the socket
        let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).expect("Didn't receive data");

        println!(
            "Received {} from: {}",
            number_of_bytes,
            src_addr.to_string()
        );

        let filled_buf = &mut buf[..number_of_bytes];

        decoder::decode(filled_buf);
    }
}
