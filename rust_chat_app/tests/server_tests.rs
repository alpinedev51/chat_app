use rust_chat_app::{Client, ServerConfig, ServerError};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;

mod common;

#[test]
fn test_server_config_creation() {
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    assert!(addr.is_ipv4());
    let config = ServerConfig::new(addr, "testpass", 5);
    assert!(config.is_ok());

    let config = ServerConfig::new(addr, "", 5);
    assert!(matches!(config.unwrap_err(), ServerError::InvalidPasscode));

    let config = ServerConfig::new(addr, "testpass", 0);
    assert!(matches!(config.unwrap_err(), ServerError::InvalidConfig(_)));
}

#[test]
fn test_client_creation() {
    let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let listener = TcpListener::bind(addr).unwrap();
    let server_addr = listener.local_addr().unwrap();

    thread::spawn(move || {
        listener.accept().unwrap();
    });

    let stream = TcpStream::connect(server_addr).unwrap();
    let client = Client::new(stream);
    assert!(client.is_ok());
}
