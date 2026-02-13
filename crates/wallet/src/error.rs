//! Error types for wallet operations

use thiserror::Error;

/// Result type for wallet operations
pub type Result<T> = std::result::Result<T, WalletError>;

/// Errors that can occur in wallet operations
#[derive(Error, Debug)]
pub enum WalletError {
    /// Cryptographic operation failed
    #[error("Crypto error: {0}")]
    CryptoError(String),

    /// Key derivation failed
    #[error("Key derivation error: {0}")]
    KeyDerivationError(String),

    /// Invalid mnemonic phrase
    #[error("Invalid mnemonic: {0}")]
    InvalidMnemonic(String),

    /// Invalid address format
    #[error("Invalid address: {0}")]
    InvalidAddress(String),

    /// Insufficient balance
    #[error("Insufficient balance: have {have}, need {need}")]
    InsufficientBalance { have: String, need: String },

    /// Transaction failed
    #[error("Transaction error: {0}")]
    TransactionError(String),

    /// Network error
    #[error("Network error: {0}")]
    NetworkError(String),

    /// RPC error
    #[error("RPC error: {0}")]
    RpcError(String),

    /// Storage error
    #[error("Storage error: {0}")]
    StorageError(String),

    /// Swap error
    #[error("Swap error: {0}")]
    SwapError(String),

    /// Unsupported currency
    #[error("Unsupported currency: {0}")]
    UnsupportedCurrency(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

impl From<invisible_crypto::CryptoError> for WalletError {
    fn from(err: invisible_crypto::CryptoError) -> Self {
        WalletError::CryptoError(err.to_string())
    }
}
