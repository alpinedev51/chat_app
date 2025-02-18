use crate::config::client::{Client, ClientMap};
use crate::config::server::ServerConfig;
use crate::errors::server::{Result, ServerError};
use log::{error, info};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

// Chat server struct which consolidates the server's configuration details, the active clients map
// (shared across threads), and the vector of active threads
pub struct ChatServer {
    config: ServerConfig,
    active_clients: ClientMap,
    threads: Vec<JoinHandle<()>>,
}

impl ChatServer {
    pub fn new(config: ServerConfig) -> Self {
        Self {
            config,
            active_clients: Arc::new(Mutex::new(HashMap::new())),
            threads: Vec::new(),
        }
    }

    fn handle_client(clients: &ClientMap, client_addr: SocketAddr) -> Result<()> {
        loop {
            let mut clients_lock = clients
                .lock()
                .map_err(|e| ServerError::LockError(e.to_string()))?;

            let client = match clients_lock.get_mut(&client_addr) {
                Some(client) => client,
                None => {
                    error!("Client {} not found in active clients", client_addr);
                    return Ok(());
                }
            };

            let mut buffer = [0u8; 1024];

            match client.stream.read(&mut buffer) {
                Ok(0) => {
                    info!("Client {} disconnected", client_addr);
                    clients_lock.remove(&client_addr);
                    break;
                }
                Ok(n) => {
                    let message = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
                    info!("Received message from {}: {}", client_addr, message);

                    for (addr, other_client) in clients_lock.iter_mut() {
                        if *addr != client_addr {
                            if let Err(e) = other_client
                                .stream
                                .write_all(format!("{}: {}\n", client_addr, message).as_bytes())
                            {
                                error!("Failed to send message to {}: {}", addr, e);
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Error reading from client {}: {}", client_addr, e);
                    clients_lock.remove(&client_addr);
                    break;
                }
            }
            // Release lock before next iteration
            drop(clients_lock);
        }

        Ok(())
    }

    fn handle_connection(&mut self, stream: TcpStream) -> Result<()> {
        stream.set_read_timeout(Some(std::time::Duration::from_secs(30)))?;

        let mut client = Client::new(stream)?;
        info!("New client connection: {}", client.addr);

        self.authenticate_client(&mut client).map_err(|e| {
            error!("Authentication failed for {}: {}", client.addr, e);
            e
        })?;
        info!("Client authenticated successfully: {}", client.addr);

        let active_clients = Arc::clone(&self.active_clients);
        let max_clients = self.config.max_clients;
        let client_addr = client.addr;

        {
            let mut clients = active_clients
                .lock()
                .map_err(|e| ServerError::LockError(e.to_string()))?;
            if clients.len() >= max_clients {
                error!("Maximum clients reached");
                return Err(ServerError::MaxClientsReached);
            }

            info!("Client {} added to active clients", client_addr);
            clients.insert(client_addr, client);
        }

        let handle = thread::spawn(move || {
            if let Err(e) = ChatServer::handle_client(&active_clients, client_addr) {
                error!("Error handling client {}: {}", client_addr, e);
            }
        });

        self.threads.push(handle);
        Ok(())
    }

    fn authenticate_client(&self, client: &mut Client) -> Result<()> {
        client.stream.write_all(b"PASSCODE\n")?;

        let mut buffer = [0u8; 1024];
        let n = client.stream.read(&mut buffer)?;

        let received_passcode = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
        if received_passcode != self.config.passcode {
            return Err(ServerError::InvalidPasscode);
        }

        client.stream.write_all(b"AUTH_SUCCESS\n")?;

        Ok(())
    }

    pub fn start(&mut self) -> Result<()> {
        let listener = TcpListener::bind(&self.config.socket_addr)?;
        info!("Server listening on {}", self.config.socket_addr);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    if let Err(e) = self.handle_connection(stream) {
                        error!("Error handling connection: {}", e);
                    }
                }
                Err(e) => error!("Connection failed: {}", e),
            }
        }
        Ok(())
    }

    pub fn shutdown(&mut self) -> Result<()> {
        for handle in self.threads.drain(..) {
            if let Err(e) = handle.join() {
                eprintln!("Error joining thread: {:?}", e);
            }
        }
        Ok(())
    }
}

pub fn start_server(socket_addr: SocketAddr, passcode: &str) -> Result<()> {
    let server_config = ServerConfig::new(socket_addr, passcode, 5).map_err(|e| {
        error!("Failed to create server config: {}", e);
        e
    })?;

    let mut server = ChatServer::new(server_config);

    server.start().map_err(|e| {
        error!("Failed to start server: {}", e);
        e
    })?;
    Ok(())
}
