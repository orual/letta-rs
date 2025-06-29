//! Message-related types.

use crate::types::common::{ResourceId, Timestamp};
use serde::{Deserialize, Serialize};

/// Message role.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    /// User message.
    User,
    /// Assistant message.
    Assistant,
    /// System message.
    System,
}

/// Message content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Message ID.
    pub id: ResourceId,
    /// Message role.
    pub role: MessageRole,
    /// Message content.
    pub content: String,
    /// When the message was created.
    pub created_at: Timestamp,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_serialization() {
        let message = Message {
            id: ResourceId::new_v4(),
            role: MessageRole::User,
            content: "Hello, world!".to_string(),
            created_at: chrono::Utc::now(),
        };

        let json = serde_json::to_string(&message).unwrap();
        let _deserialized: Message = serde_json::from_str(&json).unwrap();
    }
}