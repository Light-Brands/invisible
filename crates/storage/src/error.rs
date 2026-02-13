//! Error types for storage operations

use thiserror::Error;

/// Result type for storage operations
pub type Result<T> = std::result::Result<T, StorageError>;

/// Errors that can occur in storage operations
#[derive(Error, Debug)]
pub enum StorageError {
    /// Database error
    #[error("Database error: {0}")]
    DatabaseError(String),

    /// Encryption error
    #[error("Encryption error: {0}")]
    EncryptionError(String),

    /// Key derivation failed
    #[error("Key derivation error: {0}")]
    KeyDerivationError(String),

    /// Invalid passphrase
    #[error("Invalid passphrase")]
    InvalidPassphrase,

    /// Record not found
    #[error("Record not found: {0}")]
    NotFound(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Migration error
    #[error("Migration error: {0}")]
    MigrationError(String),

    /// IO error
    #[error("IO error: {0}")]
    IoError(String),
}

impl From<rusqlite::Error> for StorageError {
    fn from(err: rusqlite::Error) -> Self {
        StorageError::DatabaseError(err.to_string())
    }
}

impl From<serde_json::Error> for StorageError {
    fn from(err: serde_json::Error) -> Self {
        StorageError::SerializationError(err.to_string())
    }
}

impl From<std::io::Error> for StorageError {
    fn from(err: std::io::Error) -> Self {
        StorageError::IoError(err.to_string())
    }
}
