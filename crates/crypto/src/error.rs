//! Error types for cryptographic operations

use thiserror::Error;

/// Result type for crypto operations
pub type Result<T> = std::result::Result<T, CryptoError>;

/// Errors that can occur during cryptographic operations
#[derive(Error, Debug)]
pub enum CryptoError {
    /// Invalid key format or size
    #[error("Invalid key: {0}")]
    InvalidKey(String),

    /// Key agreement failed
    #[error("Key agreement failed: {0}")]
    KeyAgreementFailed(String),

    /// Encryption failed
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),

    /// Decryption failed
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),

    /// Signature verification failed
    #[error("Signature verification failed")]
    SignatureVerificationFailed,

    /// Message authentication failed
    #[error("Message authentication failed")]
    AuthenticationFailed,

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Key derivation failed
    #[error("Key derivation failed: {0}")]
    KeyDerivationFailed(String),

    /// Invalid message format
    #[error("Invalid message format: {0}")]
    InvalidMessageFormat(String),

    /// Ratchet state error
    #[error("Ratchet state error: {0}")]
    RatchetStateError(String),

    /// Generic cryptographic error
    #[error("Cryptographic operation failed: {0}")]
    CryptoError(String),
}

impl From<bincode::Error> for CryptoError {
    fn from(err: bincode::Error) -> CryptoError {
        CryptoError::SerializationError(err.to_string())
    }
}

impl From<serde_json::Error> for CryptoError {
    fn from(err: serde_json::Error) -> CryptoError {
        CryptoError::SerializationError(err.to_string())
    }
}
