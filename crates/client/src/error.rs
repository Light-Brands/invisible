//! Client error types

use thiserror::Error;

pub type Result<T> = std::result::Result<T, ClientError>;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Not authenticated")]
    NotAuthenticated,
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Crypto error: {0}")]
    CryptoError(String),
    #[error("Storage error: {0}")]
    StorageError(String),
    #[error("Messaging error: {0}")]
    MessagingError(String),
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}
