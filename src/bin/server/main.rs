use clap::Parser;
use stash_rs::mem_stash::MemStash;
use std::path::PathBuf;

use std::error::Error;
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config: PathBuf,
}

use serde::Deserialize;
#[derive(Deserialize, Debug)]
enum StashType {
    MemStash,
}

#[derive(Deserialize, Debug)]
struct ServerConfig {
    port: u16,
    stash_type: StashType,
}

use stash_rs::stash::Stash;
use std::io;
use tokio::io::Interest;
use tokio::net::{TcpListener, TcpStream};
struct Server {
    listener: TcpListener,
    stash: Box<dyn Stash>,
}

impl Server {
    pub async fn new(config: ServerConfig) -> Self {
        Self {
            // TODO: bad unwrap here
            listener: TcpListener::bind(("127.0.0.1", config.port)).await.unwrap(),
            stash: Box::new(MemStash::new()),
        }
    }

    pub async fn run(&self) {
        loop {
            let (socket, _) = self.listener.accept().await.unwrap();
            process_socket(socket).await;
        }
    }
}

async fn process_socket(socket: TcpStream) {
    println!("new client");
    loop {
        let ready = socket
            .ready(Interest::READABLE | Interest::WRITABLE)
            .await
            .unwrap();

        if ready.is_readable() {
            let mut data = vec![0; 1024];
            // Try to read data, this may still fail with `WouldBlock`
            // if the readiness event is a false positive.
            match socket.try_read(&mut data) {
                Ok(n) => {
                    println!("read {} bytes", n);
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    return;
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let config_str = fs::read_to_string(cli.config).expect("Failed to read TOML file");
    let server_config: ServerConfig =
        toml::from_str(&config_str).expect("Failed to parse TOML file");

    let server = Server::new(server_config).await;
    server.run().await;
}
