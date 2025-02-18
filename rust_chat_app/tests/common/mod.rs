use rust_chat_app::{ChatServer, ServerConfig};
use std::net::SocketAddr;

pub fn setup_test_server() -> (ChatServer, SocketAddr) {
    let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let config = ServerConfig::new(addr, "testpass", 5).unwrap();
    let server = ChatServer::new(config);
    (server, addr)
}
