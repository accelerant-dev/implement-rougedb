use std::env;
use std::io::prelude::*;
use std::net::TcpStream;
use std::str;

static USAGE: &'static str = "
rcom: command-line communicator for Redis-compatible databases

Usage:
  rcom [options] ping
  rcom [options] get [<key>]
  rcom [options] set <key> [<value>]
  rcom (-h|--help)

Options:
  --server   The server to connect to [default: localhost:6379]
  -x         Read the last argument from STDIN.
";

#[derive(Debug)]
struct Config {
    server: String,
    command: Command,
    _read_payload_from_stdin: bool,
}

#[derive(Debug)]
enum Command {
    Ping,
    Get { key: Vec<u8> },
    Set { key: Vec<u8>, value: Vec<u8> },
}

impl Command {
    fn send<W: Write>(&self, dest: &mut W) -> std::io::Result<usize> {
        let written = match self {
            Command::Ping => dest.write(b"*1\r\n$4\r\nPING\r\n")?,
            Command::Get { key } => encode_and_write(dest, b"GET", &[key])?,
            Command::Set { key, value } => encode_and_write(dest, b"SET", &[key, value])?,
        };

        Ok(written)
    }
}

fn encode_and_write<W: Write>(
    dest: &mut W,
    command: &[u8],
    args: &[&[u8]],
) -> std::io::Result<usize> {
    let mut bytes_written = 0;
    let n_args = (1 + args.len()).to_string();

    bytes_written += dest.write(b"*")?;
    bytes_written += dest.write(n_args.as_bytes())?;
    bytes_written += dest.write(b"\r\n")?;

    bytes_written += dest.write(b"$")?;
    bytes_written += dest.write(command.len().to_string().as_bytes())?;
    bytes_written += dest.write(b"\r\n")?;
    bytes_written += dest.write(command)?;
    bytes_written += dest.write(b"\r\n")?;

    for arg in args {
        bytes_written += dest.write(b"$")?;
        bytes_written += dest.write(arg.len().to_string().as_bytes())?;
        bytes_written += dest.write(b"\r\n")?;
        bytes_written += dest.write(arg)?;
        bytes_written += dest.write(b"\r\n")?;
    }

    Ok(bytes_written)
}

impl Config {
    fn from_env() -> Result<Config, &'static str> {
        let args: Vec<_> = env::args().into_iter().collect();
        let mut pos = 1;
        let mut server = "localhost:6379".to_string();
        let mut use_stdin = false;

        if args.len() == 1 {
            usage();
        }

        for (i, pair) in args.as_slice().windows(2).enumerate() {
            let mut pair = pair.iter();
            match pair.next() {
                Some(s) if s == "--server" => {
                    server = pair.next().unwrap().clone();
                    pos += i * 2;
                }
                Some(s) if s.starts_with("--server=") => {
                    server = s["--server=".len()..].to_string();
                    pos += i * 2 - 1;
                }
                Some(s) if s == "-h" || s == "--help" => {
                    usage();
                }
                Some(s) if s == "-x" => {
                    use_stdin = true;
                    pos += 1;
                }
                Some(_) => continue,
                None => todo!(),
            }
        }

        let mut buffer = Vec::new();
        if use_stdin {
            let mut stdin = std::io::stdin();
            stdin.read_to_end(&mut buffer).unwrap();
        }

        let command_raw = args[pos].to_uppercase();
        let command = match (command_raw.as_str(), use_stdin) {
            ("PING", _) => Command::Ping,
            ("GET", false) => Command::Get {
                key: args[pos + 1].clone().into(),
            },
            ("GET", true) => Command::Get { key: buffer },
            ("SET", false) => Command::Set {
                key: args[pos + 1].clone().into(),
                value: args[pos + 2].clone().into(),
            },
            ("SET", true) => Command::Set {
                key: args[pos + 1].clone().into(),
                value: buffer,
            },
            _ => todo!("{command_raw}"),
        };

        Ok(Config {
            server,
            command,
            _read_payload_from_stdin: use_stdin,
        })
    }
}

fn usage() {
    println!("{}", USAGE);
    std::process::exit(1);
}

fn main() -> std::io::Result<()> {
    let conf = Config::from_env().unwrap();
    dbg!(&conf);

    let mut stream = TcpStream::connect(conf.server)?;
    let bytes_sent = conf.command.send(&mut stream)?;
    eprintln!("info: {bytes_sent} bytes sent");

    let mut buffer = [0; 512];
    stream.read(&mut buffer[..])?;

    match std::str::from_utf8(&buffer) {
        Ok(response) => println!("{}", &response),
        Err(_invalid_utf8) => std::io::stdout().write_all(&buffer)?,
    };

    Ok(())
}
