//! # Invisible Client Library
//!
//! High-level client API for Invisible messenger.
//!
//! ## Features
//!
//! - Account management and authentication
//! - Sending and receiving messages
//! - Contact management
//! - Wallet operations
//! - Voice and video calls
//! - Settings and preferences

#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    rust_2018_idioms,
    unused_qualifications,
    missing_debug_implementations
)]

pub mod client;
pub mod account;
pub mod contacts;
pub mod messages;
pub mod calls;
pub mod sync;
pub mod error;
pub mod dashboard;

pub use client::InvisibleClient;
pub use error::{ClientError, Result};
pub use dashboard::{ServiceDashboard, HealthStatus, ServiceStatus};
