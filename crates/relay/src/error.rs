//! Error types for the Relay

use thiserror::Error;

/// Result type for relay operations
pub type Result<T> = std::result::Result<T, RelayError>;

/// Errors that can occur in the Relay
#[derive(Error, Debug)]
pub enum RelayError {
    /// Packet processing failed
    #[error("Packet processing error: {0}")]
    PacketError(String),

    /// Network operation failed
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Invalid packet format
    #[error("Invalid packet: {0}")]
    InvalidPacket(String),
}

impl From<invisible_scrambler::ScramblerError> for RelayError {
    fn from(err: invisible_scrambler::ScramblerError) -> Self {
        RelayError::PacketError(err.to_string())
    }
}

impl From<std::io::Error> for RelayError {
    fn from(err: std::io::Error) -> Self {
        RelayError::NetworkError(err.to_string())
    }
}
