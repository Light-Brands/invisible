//! # Invisible Messaging Engine
//!
//! Core messaging functionality with Double Ratchet encryption.
//!
//! ## Features
//!
//! - End-to-end encryption with Signal Protocol
//! - Message queuing and delivery
//! - Conversation management
//! - Read receipts and typing indicators
//! - File attachments
//! - Burn rooms (self-destructing conversations)

#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    rust_2018_idioms,
    unused_qualifications,
    missing_debug_implementations
)]

pub mod error;
pub mod conversation;
pub mod message;
pub mod session;
pub mod attachment;

pub use error::{MessagingError, Result};
pub use conversation::{Conversation, ConversationType};
pub use message::{Message, MessageStatus, MessageType};
pub use session::MessagingSession;
