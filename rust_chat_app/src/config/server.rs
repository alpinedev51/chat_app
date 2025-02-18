use crate::errors::server::{Result, ServerError};
use std::net::SocketAddr;

// Server Configuration specifying the socket address, server passcode, and maximum number of
// clients that can connect at the same time
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub(crate) socket_addr: SocketAddr,
    pub(crate) passcode: String,
    pub(crate) max_clients: usize,
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
