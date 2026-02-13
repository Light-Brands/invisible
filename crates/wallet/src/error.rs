//! Error types for Shadow Wallet

use thiserror::Error;

/// Result type for wallet operations
pub type Result<T> = std::result::Result<T, WalletError>;

/// Wallet errors
#[derive(Error, Debug)]
pub enum WalletError {
    /// Insufficient balance
    #[error("Insufficient balance: need {needed}, have {available}")]
    InsufficientBalance {
        /// Amount needed
        needed: u64,
        /// Amount available
        available: u64,
    },

    /// Invalid address
    #[error("Invalid address: {0}")]
    InvalidAddress(String),

    /// Transaction failed
    #[error("Transaction failed: {0}")]
    TransactionFailed(String),

    /// Network error
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Cryptographic error
    #[error("Crypto error: {0}")]
    CryptoError(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

impl From<invisible_crypto::CryptoError> for WalletError {
    fn from(err: invisible_crypto::CryptoError) -> Self {
        WalletError::CryptoError(err.to_string())
    }
}
