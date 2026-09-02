use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{Read, Write};

use anyhow::Result;

fn handle_client(mut stream: TcpStream) -> Result<()> {
    let mut buffer = [0u8; 1024];

    let n: usize = stream.read(&mut buffer)?;

    if n == 0 {
        return Ok(());
    }

    let request = String::from_utf8_lossy(&buffer[..n]);
    // Split request into address line and header
    let Some((address, header)) = request.split_once("\n") else { todo!() };
    // Split address line into Mode, Path, and Version
    // Parse Header with key and value separated by a :
    let address_vec: Vec<&str> = address.splitn(3, " ").collect();

    println!(
        "Mode: {}\n
         Path: {}\n
         Version: {}\n
        ---\n
        {header}",
        address_vec[0],
        address_vec[1],
        address_vec[2]

    );

    let response = "HTTP/1.1 200 OK\r\n\r\nSuccess ECRI XOXO".to_string();

    let _ = stream.write(response.as_bytes());

    Ok(())
}


fn main() -> Result<()> {
    let ipaddr = [127, 0, 0, 1];
    let port = 8000;
    let addr = SocketAddr::from((ipaddr, port));
    let listener = TcpListener::bind(&addr)?;

    println!("Server listening on {}:{} ...",
        ipaddr
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join("."),
        port
    );

    for stream in listener.incoming() {
        handle_client(stream?)?;
    }
    Ok(())
}
