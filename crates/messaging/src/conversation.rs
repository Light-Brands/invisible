//! Conversation management
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConversationType {
    OneOnOne,
    Group,
    BurnRoom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub name: Option<String>,
    pub conversation_type: ConversationType,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
