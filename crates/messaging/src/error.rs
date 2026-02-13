//! Messaging error types

use thiserror::Error;

pub type Result<T> = std::result::Result<T, MessagingError>;

#[derive(Error, Debug)]
pub enum MessagingError {
    #[error("Crypto error: {0}")]
    CryptoError(String),
    #[error("Storage error: {0}")]
    StorageError(String),
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Session not found: {0}")]
    SessionNotFound(String),
    #[error("Invalid message format: {0}")]
    InvalidFormat(String),
}

impl From<invisible_crypto::CryptoError> for MessagingError {
    fn from(err: invisible_crypto::CryptoError) -> Self {
        MessagingError::CryptoError(err.to_string())
    }
}
