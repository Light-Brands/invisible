//! Error types for relay operations

use thiserror::Error;

/// Result type for relay operations
pub type Result<T> = std::result::Result<T, RelayError>;

/// Errors that can occur in relay operations
#[derive(Error, Debug)]
pub enum RelayError {
    /// Packet processing failed
    #[error("Packet processing error: {0}")]
    PacketError(String),

    /// Network error
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Mixing error
    #[error("Mixing error: {0}")]
    MixingError(String),

    /// Rate limit exceeded
    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    /// Storage error
    #[error("Storage error: {0}")]
    StorageError(String),

    /// Cryptographic error
    #[error("Crypto error: {0}")]
    CryptoError(String),

    /// Scrambler error
    #[error("Scrambler error: {0}")]
    ScramblerError(String),
}

impl From<invisible_crypto::CryptoError> for RelayError {
    fn from(err: invisible_crypto::CryptoError) -> Self {
        RelayError::CryptoError(err.to_string())
    }
}

impl From<invisible_scrambler::ScramblerError> for RelayError {
    fn from(err: invisible_scrambler::ScramblerError) -> Self {
        RelayError::ScramblerError(err.to_string())
    }
}
