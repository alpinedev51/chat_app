use clap::{Parser, Subcommand};
use std::net::ToSocketAddrs;

mod client;
mod server;

#[derive(Parser)]
#[command(name = "tcp_chat")]
#[command(about = "A simple TCP chat application in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Server {
        hostname: String,
        port: u16,
        passcode: String,
    },
    Client {
        hostname: String,
        port: u16,
        passcode: String,
        username: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Server {
            hostname,
            port,
            passcode,
        }) => {
            let addr = format!("{}:{}", hostname, port);
            match addr.to_socket_addrs() {
                Ok(mut resolved) => {
                    if let Some(socket_addr) = resolved.next() {
                        println!("Starting server at {}", socket_addr);
                        server::start_server(&socket_addr, passcode);
                    }
                }
                Err(e) => eprintln!("Error resolving address: {}", e),
            }
        }
        Some(Commands::Client {
            hostname,
            port,
            passcode,
            username,
        }) => {
            let addr = format!("{}:{}", hostname, port);
            match addr.to_socket_addrs() {
                Ok(mut resolved) => {
                    if let Some(socket_addr) = resolved.next() {
                        println!(
                            "Connecting client to server at {} as {}",
                            socket_addr, username
                        );
                        client::connect_client_to_server(&socket_addr, passcode, username);
                    }
                }
                Err(e) => eprintln!("Error resolving address: {}", e),
            }
        }
        None => {}
    }
}
