//! # Invisible Relay
//!
//! Relay node implementation for the Invisible mix network.
//!
//! ## Overview
//!
//! Relay nodes are the backbone of Invisible's privacy-preserving network.
//! Each relay node:
//! - Processes Sphinx packets
//! - Implements batch-shuffle-forward mixing
//! - Generates cover traffic
//! - Enforces rate limiting
//! - Provides metrics for monitoring
//!
//! ## Architecture
//!
//! - **HTTP API:** Accept incoming packets, provide health checks
//! - **Packet Processor:** Process Sphinx packets, decrypt routing info
//! - **Mix Queue:** Batch and shuffle packets before forwarding
//! - **Cover Traffic:** Generate indistinguishable dummy packets
//! - **Metrics:** Export Prometheus metrics

#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    rust_2018_idioms,
    unused_qualifications,
    missing_debug_implementations
)]

pub mod error;
pub mod node;
pub mod api;
pub mod metrics;

pub use error::{RelayError, Result};
pub use node::RelayNode;
