//! Error types for the Scrambler

use thiserror::Error;

/// Result type for scrambler operations
pub type Result<T> = std::result::Result<T, ScramblerError>;

/// Errors that can occur in the Scrambler
#[derive(Error, Debug)]
pub enum ScramblerError {
    /// Sphinx packet processing failed
    #[error("Sphinx packet error: {0}")]
    SphinxError(String),

    /// Mixnet routing failed
    #[error("Mixnet routing error: {0}")]
    MixnetError(String),

    /// Cover traffic generation failed
    #[error("Cover traffic error: {0}")]
    CoverTrafficError(String),

    /// Shamir secret sharing failed
    #[error("Shamir secret sharing error: {0}")]
    ShamirError(String),

    /// Temporal delay scheduling failed
    #[error("Temporal delay error: {0}")]
    TemporalError(String),

    /// Invalid packet format
    #[error("Invalid packet format: {0}")]
    InvalidPacket(String),

    /// Cryptographic operation failed
    #[error("Crypto error: {0}")]
    CryptoError(String),

    /// Network operation failed
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// VPN connection error
    #[error("VPN error: {0}")]
    VpnError(String),
}

impl From<invisible_crypto::CryptoError> for ScramblerError {
    fn from(err: invisible_crypto::CryptoError) -> Self {
        ScramblerError::CryptoError(err.to_string())
    }
}
