use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("localhost:6379")?;
    stream.write(b"*1\r\n$4\r\nPING\r\n")?;

    let mut buffer = [0; 512];
    stream.read(&mut buffer[..])?;
    let response = std::str::from_utf8(&buffer).unwrap();

    println!("{}", &response);

    Ok(())
}
