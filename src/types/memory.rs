//! Memory-related types.

use crate::types::common::{Metadata, ResourceId, Timestamp};
use serde::{Deserialize, Serialize};

/// Memory block for agent context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryBlock {
    /// Block identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Block label (e.g., "human", "persona").
    pub label: String,
    /// Block value/content.
    pub value: String,
    /// Character limit for the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Whether this is a template.
    #[serde(default)]
    pub is_template: bool,
    /// Whether to preserve on migration.
    #[serde(default)]
    pub preserve_on_migration: bool,
    /// Whether the block is read-only.
    #[serde(default)]
    pub read_only: bool,
    /// Block description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Block metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    /// Block name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Organization ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// Created by user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_id: Option<String>,
    /// Last updated by user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by_id: Option<String>,
    /// Created timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    /// Updated timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
}

/// Archival memory entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchivalMemory {
    /// Memory ID.
    pub id: ResourceId,
    /// Agent ID this memory belongs to.
    pub agent_id: ResourceId,
    /// Memory content.
    pub content: String,
    /// Optional embedding vector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding: Option<Vec<f32>>,
    /// When the memory was created.
    pub created_at: Timestamp,
}

/// Request to insert archival memory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertArchivalMemoryRequest {
    /// Memory content.
    pub content: String,
}

/// Core memory response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreMemory {
    /// Memory blocks organized by label.
    #[serde(flatten)]
    pub blocks: std::collections::HashMap<String, MemoryBlock>,
}

/// Core memory update request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCoreMemoryRequest {
    /// New value for the memory block.
    pub value: String,
}

/// Request to update a memory block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMemoryBlockRequest {
    /// Block label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Block value/content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Character limit for the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Block name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether to preserve on migration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_on_migration: Option<bool>,
    /// Whether the block is read-only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// Block description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Block metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

/// Request to create a memory block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMemoryBlockRequest {
    /// Block label.
    pub label: String,
    /// Block value/content.
    pub value: String,
    /// Character limit for the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Block name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether this is a template.
    #[serde(default)]
    pub is_template: bool,
    /// Whether to preserve on migration.
    #[serde(default)]
    pub preserve_on_migration: bool,
    /// Whether the block is read-only.
    #[serde(default)]
    pub read_only: bool,
    /// Block description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Block metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

/// Memory query parameters.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MemoryQueryParams {
    /// Query text for semantic search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Limit number of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Pagination cursor (before).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Pagination cursor (after).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

/// Request to summarize agent messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummarizeMessagesRequest {
    /// Force a summarization (by default will only trigger when context is near limit).
    #[serde(default)]
    pub force: bool,
}

/// Response from message summarization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummarizeMessagesResponse {
    /// The generated summary.
    pub summary: String,
    /// Number of messages summarized.
    pub messages_summarized: u32,
}

/// Recall memory summary containing all memory types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecallMemorySummary {
    /// Core memory blocks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_memory: Option<CoreMemory>,
    /// Archival memory entries.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub archival_memory: Vec<ArchivalMemory>,
    /// Recall memory entries.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recall_memory: Vec<RecallMemoryEntry>,
}

/// Recall memory entry (message history).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecallMemoryEntry {
    /// Message ID.
    pub id: String,
    /// Agent ID.
    pub agent_id: String,
    /// Message role.
    pub role: String,
    /// Message content.
    pub content: String,
    /// When the message was created.
    pub created_at: Timestamp,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_block_serialization() {
        let block = MemoryBlock {
            id: Some("block-123".to_string()),
            label: "human".to_string(),
            value: "The human's name is Alice.".to_string(),
            limit: Some(1000),
            is_template: false,
            preserve_on_migration: true,
            read_only: false,
            description: Some("Human information".to_string()),
            metadata: None,
            name: None,
            organization_id: None,
            created_by_id: None,
            last_updated_by_id: None,
            created_at: None,
            updated_at: None,
        };

        let json = serde_json::to_string(&block).unwrap();
        let deserialized: MemoryBlock = serde_json::from_str(&json).unwrap();
        assert_eq!(block.label, deserialized.label);
        assert_eq!(block.value, deserialized.value);
    }

    #[test]
    fn test_archival_memory_serialization() {
        let memory = ArchivalMemory {
            id: "block-00000000-0000-0000-0000-000000000000".to_string(),
            agent_id: "agent-00000000-0000-0000-0000-000000000000".to_string(),
            content: "Important information to remember".to_string(),
            embedding: Some(vec![0.1, 0.2, 0.3]),
            created_at: chrono::Utc::now(),
        };

        let json = serde_json::to_string(&memory).unwrap();
        let deserialized: ArchivalMemory = serde_json::from_str(&json).unwrap();
        assert_eq!(memory.content, deserialized.content);
    }

    #[test]
    fn test_memory_query_params() {
        let params = MemoryQueryParams {
            query: Some("search term".to_string()),
            limit: Some(10),
            before: None,
            after: None,
        };

        let json = serde_json::to_string(&params).unwrap();
        assert!(json.contains("query"));
        assert!(!json.contains("before")); // Should be skipped when None
    }
}
