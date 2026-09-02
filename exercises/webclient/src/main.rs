use std::io::prelude::*;
use std::net::TcpStream;
use anyhow::Result;

fn main() -> Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8000")?;

    let request = stream.write("GET / HTTP/1.1\r\n\r\n".as_bytes())?; // ignore the Result

    let mut buffer = [0u8; 1024];

    let n: usize = stream.read(&mut buffer)?; // ignore this too
    if n == 0 {
        return Ok(());
    }

    let response = String::from_utf8_lossy(&buffer[..n]);

    println!("{}", response);
    Ok(())
}
