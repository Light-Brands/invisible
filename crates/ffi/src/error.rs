//! FFI error types

use thiserror::Error;

/// FFI result type
pub type FfiResult<T> = std::result::Result<T, FfiError>;

/// FFI errors
#[derive(Error, Debug)]
pub enum FfiError {
    /// Crypto error
    #[error("Crypto error: {0}")]
    CryptoError(String),

    /// Wallet error
    #[error("Wallet error: {0}")]
    WalletError(String),

    /// Storage error
    #[error("Storage error: {0}")]
    StorageError(String),

    /// Invalid argument
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    /// Not initialized
    #[error("Not initialized")]
    NotInitialized,
}

impl From<invisible_crypto::CryptoError> for FfiError {
    fn from(err: invisible_crypto::CryptoError) -> Self {
        FfiError::CryptoError(err.to_string())
    }
}

impl From<invisible_wallet::WalletError> for FfiError {
    fn from(err: invisible_wallet::WalletError) -> Self {
        FfiError::WalletError(err.to_string())
    }
}

impl From<invisible_storage::StorageError> for FfiError {
    fn from(err: invisible_storage::StorageError) -> Self {
        FfiError::StorageError(err.to_string())
    }
}
