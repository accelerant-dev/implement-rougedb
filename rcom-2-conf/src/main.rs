use std::env;
use std::io::prelude::*;
use std::net::TcpStream;
use std::str;

#[derive(Debug)]
struct Config {
    server: String,
    command: Command,
}

#[derive(Debug)]
enum Command {
    Ping,
}

impl Command {
    fn send<W: Write>(&self, dest: &mut W) -> std::io::Result<usize> {
        let written = match self {
            Command::Ping => dest.write(b"*1\r\n$4\r\nPING\r\n")?,
        };

        Ok(written)
    }
}

impl Config {
    fn from_env() -> Result<Config, &'static str> {
        let args: Vec<_> = env::args().into_iter().collect();
        let pos = 1;
        let server = "localhost:6379".to_string();

        let command_raw = args[pos].to_uppercase();
        let command = match command_raw.as_str() {
            "PING" => Command::Ping,
            _ => todo!("{command_raw}"),
        };

        Ok(Config { server, command })
    }
}

fn main() -> std::io::Result<()> {
    let conf = Config::from_env().unwrap();
    // dbg!(&conf);

    let mut stream = TcpStream::connect(conf.server)?;
    let bytes_sent = conf.command.send(&mut stream)?;
    eprintln!("info: {bytes_sent} bytes sent");

    let mut buffer = [0; 512];
    stream.read(&mut buffer[..])?;
    let response = std::str::from_utf8(&buffer).unwrap();
    println!("{}", &response);

    Ok(())
}
