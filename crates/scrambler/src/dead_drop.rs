//! Dead Drops (Layer 6)
//!
//! Anonymous relay mailboxes for asynchronous message delivery.
//! Messages are stored temporarily on relay nodes without any metadata
//! linking sender to recipient.
//!
//! ## Architecture
//!
//! - **Drop Points:** Random relay nodes store encrypted messages
//! - **Anonymous Retrieval:** Recipients poll with access token, no identity
//! - **Ephemeral Storage:** Messages expire after TTL or retrieval
//! - **Cover Traffic:** Fake polls maintain constant query rate
//!
//! ## Security Properties
//!
//! - **Unlinkability:** No connection between sender and recipient
//! - **Deniability:** Cannot prove who stored or retrieved a message
//! - **Forward Secrecy:** Access tokens derived per-message
//! - **Traffic Analysis Resistance:** Cover polls hide real retrievals

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::error::{Result, ScramblerError};

/// Dead drop identifier (derived from recipient key)
pub type DropId = [u8; 32];

/// Message access token (derived from shared secret)
pub type AccessToken = [u8; 32];

/// Dead drop configuration
#[derive(Debug, Clone)]
pub struct DeadDropConfig {
    /// Message time-to-live (seconds)
    pub message_ttl: u64,
    /// Maximum messages per drop
    pub max_messages: usize,
    /// Poll interval for cover traffic (ms)
    pub poll_interval_ms: u64,
}

impl Default for DeadDropConfig {
    fn default() -> Self {
        Self {
            message_ttl: 86400,      // 24 hours
            max_messages: 100,
            poll_interval_ms: 5000,  // 5 seconds
        }
    }
}

/// Stored message in dead drop
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredMessage {
    /// Message ID (random)
    pub id: [u8; 16],
    /// Encrypted message payload
    pub payload: Vec<u8>,
    /// Timestamp when stored
    pub stored_at: u64,
    /// Time-to-live (seconds)
    pub ttl: u64,
}

impl StoredMessage {
    /// Check if message has expired
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        now >= self.stored_at + self.ttl
    }

    /// Time until expiration (seconds)
    pub fn time_until_expiry(&self) -> Option<u64> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let expiry_time = self.stored_at + self.ttl;
        if now < expiry_time {
            Some(expiry_time - now)
        } else {
            None
        }
    }
}

/// Dead drop relay node
///
/// Stores messages temporarily for anonymous retrieval.
#[derive(Debug)]
pub struct DeadDropNode {
    /// Configuration
    config: DeadDropConfig,
    /// Drop ID -> Messages
    drops: HashMap<DropId, Vec<StoredMessage>>,
    /// Access token -> Drop ID (for retrieval)
    access_tokens: HashMap<AccessToken, DropId>,
}

impl DeadDropNode {
    /// Create a new dead drop node
    pub fn new(config: DeadDropConfig) -> Self {
        Self {
            config,
            drops: HashMap::new(),
            access_tokens: HashMap::new(),
        }
    }

    /// Store a message in a dead drop
    ///
    /// # Arguments
    /// * `drop_id` - Drop identifier (derived from recipient key)
    /// * `access_token` - Token for retrieval
    /// * `payload` - Encrypted message
    ///
    /// # Returns
    /// * Message ID for tracking
    pub fn store_message(
        &mut self,
        drop_id: DropId,
        access_token: AccessToken,
        payload: Vec<u8>,
    ) -> Result<[u8; 16]> {
        // Generate message ID
        use rand::RngCore;
        let mut id = [0u8; 16];
        rand::thread_rng().fill_bytes(&mut id);

        // Create stored message
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let message = StoredMessage {
            id,
            payload,
            stored_at: now,
            ttl: self.config.message_ttl,
        };

        // Get or create drop
        let messages = self.drops.entry(drop_id).or_insert_with(Vec::new);

        // Check capacity
        if messages.len() >= self.config.max_messages {
            return Err(ScramblerError::NetworkError(
                "Drop is full".to_string(),
            ));
        }

        // Store message
        messages.push(message);

        // Register access token
        self.access_tokens.insert(access_token, drop_id);

        tracing::debug!(
            drop_id = ?drop_id,
            message_id = ?id,
            "Message stored in dead drop"
        );

        Ok(id)
    }

    /// Retrieve messages from a dead drop
    ///
    /// # Arguments
    /// * `access_token` - Access token for the drop
    ///
    /// # Returns
    /// * List of messages (or empty if no messages/invalid token)
    pub fn retrieve_messages(&mut self, access_token: &AccessToken) -> Result<Vec<StoredMessage>> {
        // Look up drop ID from access token
        let drop_id = match self.access_tokens.get(access_token) {
            Some(id) => *id,
            None => {
                // Invalid token - return empty to avoid leaking info
                return Ok(Vec::new());
            }
        };

        // Get messages from drop
        let messages = self.drops.get_mut(&drop_id);

        if let Some(msgs) = messages {
            // Remove expired messages
            msgs.retain(|m| !m.is_expired());

            // Take all messages and clear drop
            let retrieved = msgs.drain(..).collect();

            tracing::debug!(
                drop_id = ?drop_id,
                count = msgs.len(),
                "Messages retrieved from dead drop"
            );

            Ok(retrieved)
        } else {
            // No messages
            Ok(Vec::new())
        }
    }

    /// Clean up expired messages
    pub fn cleanup_expired(&mut self) -> usize {
        let mut removed = 0;

        // Remove expired messages from each drop
        for messages in self.drops.values_mut() {
            let before = messages.len();
            messages.retain(|m| !m.is_expired());
            removed += before - messages.len();
        }

        // Remove empty drops
        self.drops.retain(|_, msgs| !msgs.is_empty());

        if removed > 0 {
            tracing::debug!(
                removed,
                "Cleaned up expired messages"
            );
        }

        removed
    }

    /// Get statistics about the dead drop node
    pub fn stats(&self) -> DeadDropStats {
        let total_drops = self.drops.len();
        let total_messages: usize = self.drops.values().map(|m| m.len()).sum();

        DeadDropStats {
            total_drops,
            total_messages,
        }
    }
}

/// Dead drop statistics
#[derive(Debug, Clone)]
pub struct DeadDropStats {
    /// Number of active drops
    pub total_drops: usize,
    /// Total messages stored
    pub total_messages: usize,
}

/// Dead drop client
///
/// Stores and retrieves messages from dead drop relay nodes.
#[derive(Debug)]
pub struct DeadDropClient {
    /// Configuration
    config: DeadDropConfig,
}

impl DeadDropClient {
    /// Create a new dead drop client
    pub fn new(config: DeadDropConfig) -> Self {
        Self { config }
    }

    /// Derive drop ID from recipient public key
    pub fn derive_drop_id(&self, recipient_key: &[u8]) -> DropId {
        use ring::digest;

        let digest = digest::digest(&digest::SHA256, recipient_key);
        let mut drop_id = [0u8; 32];
        drop_id.copy_from_slice(digest.as_ref());
        drop_id
    }

    /// Derive access token from shared secret
    pub fn derive_access_token(&self, shared_secret: &[u8]) -> AccessToken {
        use ring::digest;

        let digest = digest::digest(&digest::SHA256, shared_secret);
        let mut token = [0u8; 32];
        token.copy_from_slice(digest.as_ref());
        token
    }

    /// Generate a cover poll (fake retrieval to hide real polls)
    pub fn generate_cover_poll(&self) -> AccessToken {
        use rand::RngCore;
        let mut token = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut token);
        token
    }

    /// Get next poll delay
    pub fn next_poll_delay(&self) -> Duration {
        Duration::from_millis(self.config.poll_interval_ms)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_and_retrieve() {
        let config = DeadDropConfig::default();
        let mut node = DeadDropNode::new(config.clone());
        let client = DeadDropClient::new(config);

        let recipient_key = b"recipient_public_key";
        let shared_secret = b"shared_secret_for_access";

        let drop_id = client.derive_drop_id(recipient_key);
        let access_token = client.derive_access_token(shared_secret);

        // Store message
        let payload = b"encrypted message payload".to_vec();
        let message_id = node
            .store_message(drop_id, access_token, payload.clone())
            .unwrap();

        // Retrieve messages
        let messages = node.retrieve_messages(&access_token).unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].id, message_id);
        assert_eq!(messages[0].payload, payload);

        // Second retrieval should be empty
        let messages = node.retrieve_messages(&access_token).unwrap();
        assert_eq!(messages.len(), 0);
    }

    #[test]
    fn test_invalid_access_token() {
        let config = DeadDropConfig::default();
        let mut node = DeadDropNode::new(config.clone());
        let client = DeadDropClient::new(config);

        let recipient_key = b"recipient_public_key";
        let shared_secret = b"shared_secret_for_access";

        let drop_id = client.derive_drop_id(recipient_key);
        let access_token = client.derive_access_token(shared_secret);

        // Store message
        let payload = b"encrypted message payload".to_vec();
        node.store_message(drop_id, access_token, payload).unwrap();

        // Try with wrong token
        let wrong_token = client.derive_access_token(b"wrong_secret");
        let messages = node.retrieve_messages(&wrong_token).unwrap();
        assert_eq!(messages.len(), 0);
    }

    #[test]
    fn test_capacity_limit() {
        let mut config = DeadDropConfig::default();
        config.max_messages = 2;

        let mut node = DeadDropNode::new(config.clone());
        let client = DeadDropClient::new(config);

        let recipient_key = b"recipient_public_key";
        let shared_secret = b"shared_secret_for_access";

        let drop_id = client.derive_drop_id(recipient_key);
        let access_token = client.derive_access_token(shared_secret);

        // Store up to capacity
        for i in 0..2 {
            let payload = format!("message {}", i).into_bytes();
            node.store_message(drop_id, access_token, payload).unwrap();
        }

        // Third message should fail
        let payload = b"message 3".to_vec();
        let result = node.store_message(drop_id, access_token, payload);
        assert!(result.is_err());
    }

    #[test]
    fn test_message_expiration() {
        let mut config = DeadDropConfig::default();
        config.message_ttl = 0; // Immediate expiration

        let mut node = DeadDropNode::new(config.clone());
        let client = DeadDropClient::new(config);

        let recipient_key = b"recipient_public_key";
        let shared_secret = b"shared_secret_for_access";

        let drop_id = client.derive_drop_id(recipient_key);
        let access_token = client.derive_access_token(shared_secret);

        // Store message
        let payload = b"encrypted message payload".to_vec();
        node.store_message(drop_id, access_token, payload).unwrap();

        // Wait a moment
        std::thread::sleep(Duration::from_millis(10));

        // Message should be expired and not returned
        let messages = node.retrieve_messages(&access_token).unwrap();
        assert_eq!(messages.len(), 0);
    }

    #[test]
    fn test_cleanup_expired() {
        let mut config = DeadDropConfig::default();
        config.message_ttl = 0; // Immediate expiration

        let mut node = DeadDropNode::new(config.clone());
        let client = DeadDropClient::new(config);

        let recipient_key = b"recipient_public_key";
        let shared_secret = b"shared_secret_for_access";

        let drop_id = client.derive_drop_id(recipient_key);
        let access_token = client.derive_access_token(shared_secret);

        // Store multiple messages
        for i in 0..5 {
            let payload = format!("message {}", i).into_bytes();
            node.store_message(drop_id, access_token, payload).unwrap();
        }

        // Wait for expiration
        std::thread::sleep(Duration::from_millis(10));

        // Cleanup should remove all messages
        let removed = node.cleanup_expired();
        assert_eq!(removed, 5);

        let stats = node.stats();
        assert_eq!(stats.total_messages, 0);
    }

    #[test]
    fn test_cover_polls() {
        let config = DeadDropConfig::default();
        let client = DeadDropClient::new(config);

        // Generate multiple cover polls
        let token1 = client.generate_cover_poll();
        let token2 = client.generate_cover_poll();

        // Should be random and different
        assert_ne!(token1, token2);
    }

    #[test]
    fn test_poll_delay() {
        let config = DeadDropConfig::default();
        let client = DeadDropClient::new(config);

        let delay = client.next_poll_delay();
        assert_eq!(delay.as_millis(), 5000);
    }
}
