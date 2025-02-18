use log::error;
use std::net::{SocketAddr, TcpStream};

// Server Configuration specifying the socket address, server passcode, and maximum number of
// clients that can connect at the same time
#[derive(Clone)]
pub struct ServerConfig {
    socket_addr: SocketAddr,
    passcode: String,
    max_clients: usize,
}

impl ServerConfig {
    pub fn new(socket_addr: SocketAddr, passcode: &str, max_clients: usize) -> Result<Self> {
        if passcode.is_empty() {
            return Err(ServerError::InvalidPasscode);
        }
        if max_clients <= 0 {
            return Err(ServerError::InvalidConfig(
                "max_clients must be greater than 0".into(),
            ));
        }
        Ok(ServerConfig {
            socket_addr,
            passcode: passcode.to_string(),
            max_clients,
        })
    }
}

// Client struct that consolidates client related data. Namely the socket/tcp stream dedicated to
// the client and the socket address it is bound to
#[derive(Debug)]
struct Client {
    stream: TcpStream,
    addr: SocketAddr,
}

impl Client {
    fn new(stream: TcpStream) -> Result<(Self)> {
        let addr = stream.peer_addr()?;
        Ok(Self { stream, addr })
    }
}
