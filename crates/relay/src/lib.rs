//! # Invisible Relay
//!
//! Mix node relay server implementation for the Invisible network.
//!
//! Relay nodes process Sphinx packets through the mixnet, providing:
//! - Onion routing with batch-shuffle-forward mixing
//! - Dead drop message storage
//! - Cover traffic generation
//! - Geographic diversity

#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    rust_2018_idioms,
    unused_qualifications,
    missing_debug_implementations
)]

pub mod error;
pub mod node;
pub mod server;

pub use error::{RelayError, Result};
pub use node::{MixNode, NodeConfig, NodeStats};
pub use server::RelayServer;
