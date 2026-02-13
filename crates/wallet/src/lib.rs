//! # Invisible Shadow Wallet
//!
//! Non-custodial, privacy-first cryptocurrency wallet.
//!
//! ## Supported Currencies
//!
//! - **XMR (Monero):** Privacy by default
//! - **ZEC (Zcash):** Shielded transactions  
//! - **BTC (Bitcoin):** CoinJoin mixing
//! - **ETH (Ethereum):** Tornado Cash integration
//!
//! ## Features
//!
//! - **Non-Custodial:** Users control private keys
//! - **Privacy Parity:** All transactions use privacy features
//! - **Atomic Swaps:** Cross-chain via HTLC
//! - **WalletConnect:** DeFi integration

#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    rust_2018_idioms,
    unused_qualifications,
    missing_debug_implementations
)]

pub mod error;
pub mod wallet;
pub mod btc;
pub mod xmr;

pub use error::{WalletError, Result};
pub use wallet::{ShadowWallet, WalletConfig, Balance, Currency};
