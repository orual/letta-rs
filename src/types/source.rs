//! Source-related types.

use crate::types::common::{ResourceId, Timestamp};
use serde::{Deserialize, Serialize};

/// Data source.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    /// Source ID.
    pub id: ResourceId,
    /// Source name.
    pub name: String,
    /// Source description.
    pub description: Option<String>,
    /// When the source was created.
    pub created_at: Timestamp,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_serialization() {
        let source = Source {
            id: ResourceId::new_v4(),
            name: "documents".to_string(),
            description: Some("Document collection".to_string()),
            created_at: chrono::Utc::now(),
        };

        let json = serde_json::to_string(&source).unwrap();
        let _deserialized: Source = serde_json::from_str(&json).unwrap();
    }
}