//! Tool-related types.

use crate::types::common::{ResourceId, Timestamp};
use serde::{Deserialize, Serialize};

/// Tool definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    /// Tool ID.
    pub id: ResourceId,
    /// Tool name.
    pub name: String,
    /// Tool description.
    pub description: Option<String>,
    /// When the tool was created.
    pub created_at: Timestamp,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_serialization() {
        let tool = Tool {
            id: ResourceId::new_v4(),
            name: "calculator".to_string(),
            description: Some("Basic calculator tool".to_string()),
            created_at: chrono::Utc::now(),
        };

        let json = serde_json::to_string(&tool).unwrap();
        let _deserialized: Tool = serde_json::from_str(&json).unwrap();
    }
}