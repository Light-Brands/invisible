//! Message storage operations

use rusqlite::params;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::database::Database;
use crate::error::Result;

/// Stored message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredMessage {
    /// Message ID
    pub id: String,
    /// Conversation ID
    pub conversation_id: String,
    /// Sender ID
    pub sender_id: String,
    /// Encrypted content
    pub content: Vec<u8>,
    /// Timestamp
    pub timestamp: i64,
    /// Status (sent, delivered, read)
    pub status: String,
}

impl Database {
    /// Store a message
    pub fn store_message(&self, message: &StoredMessage) -> Result<()> {
        self.connection().execute(
            "INSERT INTO messages (id, conversation_id, sender_id, content, timestamp, status)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                &message.id,
                &message.conversation_id,
                &message.sender_id,
                &message.content,
                message.timestamp,
                &message.status,
            ],
        )?;
        Ok(())
    }

    /// Get messages for a conversation
    pub fn get_messages(&self, conversation_id: &str, limit: usize) -> Result<Vec<StoredMessage>> {
        let mut stmt = self.connection().prepare(
            "SELECT id, conversation_id, sender_id, content, timestamp, status
             FROM messages
             WHERE conversation_id = ?1
             ORDER BY timestamp DESC
             LIMIT ?2",
        )?;

        let messages = stmt
            .query_map(params![conversation_id, limit], |row| {
                Ok(StoredMessage {
                    id: row.get(0)?,
                    conversation_id: row.get(1)?,
                    sender_id: row.get(2)?,
                    content: row.get(3)?,
                    timestamp: row.get(4)?,
                    status: row.get(5)?,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;

        Ok(messages)
    }

    /// Delete a message
    pub fn delete_message(&self, message_id: &str) -> Result<()> {
        self.connection()
            .execute("DELETE FROM messages WHERE id = ?1", params![message_id])?;
        Ok(())
    }

    /// Delete all messages in a conversation (burn room)
    pub fn delete_conversation_messages(&self, conversation_id: &str) -> Result<()> {
        self.connection().execute(
            "DELETE FROM messages WHERE conversation_id = ?1",
            params![conversation_id],
        )?;
        Ok(())
    }
}
