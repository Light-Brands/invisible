//! # Shadow Wallet
//!
//! Privacy-first cryptocurrency wallet supporting:
//! - Monero (XMR) - Privacy by default
//! - Zcash (ZEC) - Shielded transactions
//! - Bitcoin (BTC) - CoinJoin support
//! - Ethereum (ETH) - Privacy proxy
//!
//! ## Privacy Features
//!
//! - **Non-custodial:** User controls keys
//! - **Privacy Parity:** Same network obfuscation as messages
//! - **CoinJoin:** Bitcoin mixing for privacy
//! - **Atomic Swaps:** Cross-chain without intermediaries
//! - **DeFi Integration:** WalletConnect v2 with RPC proxy
//! - **Panic Wipe:** Duress PIN destroys wallet data

#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    rust_2018_idioms,
    unused_qualifications,
    missing_debug_implementations
)]

pub mod error;
pub mod types;
pub mod hd_wallet;
pub mod monero;
pub mod zcash;
pub mod bitcoin;
pub mod ethereum;
pub mod swap;

pub use error::{WalletError, Result};
pub use types::{Currency, Balance, Transaction};
pub use hd_wallet::HDWallet;
