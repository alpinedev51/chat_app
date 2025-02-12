use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use thiserror::Error;

type ClientMap = Arc<Mutex<HashMap<SocketAddr, TcpStream>>>;

// Represents possible errors that can occur in the chat server
#[derive(Debug, Error)]
pub enum ServerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to acquire lock on client map")]
    LockError,
    #[error("Invalid passcode")]
    InvalidPasscode,
}

#[derive(Clone)]
pub struct ServerConfig {
    socket_addr: SocketAddr,
    passcode: String,
    max_clients: usize,
}

struct Client {
    stream: TcpStream,
    addr: SocketAddr,
}

pub struct ChatServer {
    config: ServerConfig,
    active_clients: ClientMap,
}

fn handle_client(stream: TcpStream, active_clients: &ClientMap, passcode: &str) {}

fn handle_connection(
    stream: TcpStream,
    active_clients: &ClientMap,
    passcode: &str,
) -> std::io::Result<()> {
    let addr = stream.peer_addr()?;
    println!("New client connection: {}", addr);

    let active_clients = Arc::clone(active_clients);
    let passcode = passcode.to_string();

    thread::spawn(move || {
        handle_client(stream, &active_clients, &passcode);
    });

    Ok(())
}

pub fn start_server(socket_addr: &SocketAddr, passcode: &str) -> std::io::Result<()> {
    // Create server parent socket
    // Bind server parent socket
    // Set server parent socket to listening state
    // Accept connections to server parent socket
    // Create and bind a server child socket to accepted connections
    // Verify passcode received from client on child socket. If wrong, close connection
    // Handle client in separate thread
    let active_clients: ClientMap = Arc::new(Mutex::new(HashMap::new()));
    let listener = TcpListener::bind(socket_addr)?;

    println!("Server listening on {}", socket_addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = handle_connection(stream, &active_clients, passcode) {
                    eprintln!("Error handling connection {}", e);
                }
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e)
            }
        }
    }
    Ok(())
}
