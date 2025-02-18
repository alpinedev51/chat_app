use thiserror::Error;

// Represents possible errors that can occur in the chat server
#[derive(Debug, Error)]
pub enum ServerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to acquire lock on client map: {0}")]
    LockError(String),
    #[error("Invalid passcode")]
    InvalidPasscode,
    #[error("Invalid config: {0}")]
    InvalidConfig(String),
    #[error("Maximum clients reached")]
    MaxClientsReached,
    #[error("Client error: {0}")]
    ClientError(String),
}

pub type Result<T> = std::result::Result<T, ServerError>;
