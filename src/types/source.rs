//! Source-related types.

use crate::types::agent::EmbeddingConfig;
use crate::types::common::{Metadata, Timestamp};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// File processing status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FileProcessingStatus {
    /// File is pending processing.
    Pending,
    /// File is being processed.
    Processing,
    /// File processing completed.
    Completed,
    /// File processing failed.
    Failed,
}

/// File metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    /// File name.
    pub file_name: String,
    /// File type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    /// File size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
}

/// Data source.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    /// Source ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Source name.
    pub name: String,
    /// Source description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Instructions for how to use the source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// Embedding configuration.
    pub embedding_config: EmbeddingConfig,
    /// Source metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    /// Created by user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_id: Option<String>,
    /// Last updated by user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by_id: Option<String>,
    /// When the source was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    /// When the source was last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
}

/// Source file representation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceFile {
    /// File ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    /// File data (base64 encoded).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_data: Option<String>,
    /// File name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}

/// File upload wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FileUpload {
    /// File upload.
    File {
        /// File details.
        file: SourceFile,
    },
}

/// Passage stored in archival memory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Passage {
    /// Passage ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The text of the passage.
    pub text: String,
    /// The embedding of the passage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding: Option<Vec<f32>>,
    /// Embedding configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_config: Option<EmbeddingConfig>,
    /// Agent ID associated with this passage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// Source ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// File ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    /// File name (for source passages).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// Passage metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    /// Whether this passage is deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    /// Created by user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_id: Option<String>,
    /// Last updated by user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by_id: Option<String>,
    /// When the passage was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    /// When the passage was last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
}

/// Create source request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSourceRequest {
    /// Source name.
    pub name: String,
    /// Source description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Instructions for how to use the source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// Embedding configuration.
    pub embedding_config: EmbeddingConfig,
    /// Source metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

/// Query parameters for listing sources.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListSourcesParams {
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

/// Upload file to source request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadFileToSourceRequest {
    /// File data or reference.
    pub file: FileUpload,
}

/// List passages parameters.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListPassagesParams {
    /// Filter by agent ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// Filter by source ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::agent::EmbeddingEndpointType;

    #[test]
    fn test_source_serialization() {
        let source = Source {
            id: Some("source-123".to_string()),
            name: "documents".to_string(),
            description: Some("Document collection".to_string()),
            instructions: Some("Use for general knowledge".to_string()),
            embedding_config: EmbeddingConfig {
                embedding_endpoint_type: Some(EmbeddingEndpointType::Openai),
                embedding_endpoint: None,
                embedding_model: Some("text-embedding-ada-002".to_string()),
                embedding_dim: Some(1536),
                embedding_chunk_size: Some(300),
                handle: None,
                azure_config: None,
                extra: HashMap::new(),
            },
            metadata: None,
            created_by_id: None,
            last_updated_by_id: None,
            created_at: Some(chrono::Utc::now()),
            updated_at: None,
        };

        let json = serde_json::to_string(&source).unwrap();
        let deserialized: Source = serde_json::from_str(&json).unwrap();
        assert_eq!(source.name, deserialized.name);
    }

    #[test]
    fn test_passage_serialization() {
        let passage = Passage {
            id: Some("passage-123".to_string()),
            text: "This is a test passage.".to_string(),
            embedding: Some(vec![0.1, 0.2, 0.3]),
            embedding_config: None,
            agent_id: Some("agent-456".to_string()),
            source_id: Some("source-789".to_string()),
            file_id: None,
            file_name: None,
            metadata: None,
            is_deleted: Some(false),
            created_by_id: None,
            last_updated_by_id: None,
            created_at: Some(chrono::Utc::now()),
            updated_at: None,
        };

        let json = serde_json::to_string(&passage).unwrap();
        let deserialized: Passage = serde_json::from_str(&json).unwrap();
        assert_eq!(passage.text, deserialized.text);
    }

    #[test]
    fn test_file_upload_serialization() {
        let file_upload = FileUpload::File {
            file: SourceFile {
                file_id: Some("file-123".to_string()),
                file_data: Some("base64encodeddata".to_string()),
                filename: Some("test.txt".to_string()),
            },
        };

        let json = serde_json::to_string(&file_upload).unwrap();
        assert!(json.contains("\"type\":\"file\""));
    }
}
