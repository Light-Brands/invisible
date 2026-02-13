//! Messaging session management
use crate::Result;

pub struct MessagingSession {
    // TODO: Double Ratchet state
    // TODO: Message queue
}

impl MessagingSession {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
}

impl Default for MessagingSession {
    fn default() -> Self {
        Self::new().unwrap()
    }
}
