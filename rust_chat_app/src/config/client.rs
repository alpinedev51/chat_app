use crate::errors::server::Result;
use std::collections::HashMap;
use std::net::{SocketAddr, TcpStream};
use std::sync::{Arc, Mutex};

pub type ClientMap = Arc<Mutex<HashMap<SocketAddr, Client>>>;

// Client struct that consolidates client related data. Namely the socket/tcp stream dedicated to
// the client and the socket address it is bound to
#[derive(Debug)]
pub struct Client {
    pub(crate) stream: TcpStream,
    pub(crate) addr: SocketAddr,
}

impl Client {
    pub fn new(stream: TcpStream) -> Result<Self> {
        let addr = stream.peer_addr()?;
        Ok(Self { stream, addr })
    }
}
