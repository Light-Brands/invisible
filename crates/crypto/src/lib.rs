//! # Invisible Crypto
//!
//! Core cryptographic primitives for Invisible messenger.
//!
//! This crate implements:
//! - X3DH key agreement protocol
//! - Double Ratchet encryption
//! - Post-quantum key exchange (PQXDH)
//! - Ed25519 signatures
//! - Key derivation and management
//!
//! ## Security
//!
//! - All operations use constant-time implementations
//! - Sensitive data is zeroized on drop
//! - Memory is locked to prevent swapping
//! - Property testing ensures cryptographic invariants

#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    rust_2018_idioms,
    unused_qualifications,
    missing_debug_implementations
)]

pub mod error;
pub mod keys;
pub mod x3dh;
pub mod double_ratchet;
pub mod kdf;
pub mod utils;

pub use error::{CryptoError, Result};
pub use keys::{IdentityKey, SignedPreKey, OneTimePreKey, KeyPair};
pub use x3dh::X3DHSession;
pub use double_ratchet::DoubleRatchet;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
