//! # Invisible Storage
//!
//! Encrypted local storage using SQLCipher for:
//! - Messages and conversations
//! - Contacts and identity keys
//! - Wallet data and transaction history
//! - Application settings
//!
//! ## Security Features
//!
//! - **Encryption at Rest:** All data encrypted with SQLCipher (AES-256)
//! - **Key Derivation:** Argon2id KDF from user passphrase + 2FA
//! - **Zeroization:** Sensitive data cleared from memory
//! - **No Plaintext:** Even temporary files are encrypted
//!
//! ## Database Schema
//!
//! - `messages` - End-to-end encrypted messages
//! - `conversations` - Conversation metadata
//! - `contacts` - Contact identity keys and info
//! - `keys` - Ratchet state and pre-keys
//! - `wallet_accounts` - Wallet accounts and balances
//! - `transactions` - Transaction history

#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    rust_2018_idioms,
    unused_qualifications,
    missing_debug_implementations
)]

pub mod error;
pub mod database;
pub mod messages;
pub mod contacts;
pub mod wallet;
pub mod migrations;

pub use error::{StorageError, Result};
pub use database::{Database, DatabaseConfig};
