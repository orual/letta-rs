//! Tool-related types.

use crate::types::common::{LettaId, Metadata, Timestamp};
use serde::{Deserialize, Serialize};

/// Tool type enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolType {
    /// Core Letta tools.
    #[serde(rename = "letta_core")]
    LettaCore,
    /// Memory core tools.
    #[serde(rename = "letta_memory_core")]
    LettaMemoryCore,
    /// Multi-agent core tools.
    #[serde(rename = "letta_multi_agent_core")]
    LettaMultiAgentCore,
    /// Sleeptime core tools.
    #[serde(rename = "letta_sleeptime_core")]
    LettaSleeptimeCore,
    /// Voice sleeptime core tools.
    #[serde(rename = "letta_voice_sleeptime_core")]
    LettaVoiceSleeptimeCore,
    /// Built-in tools.
    #[serde(rename = "letta_builtin")]
    LettaBuiltin,
    /// Files core tools.
    #[serde(rename = "letta_files_core")]
    LettaFilesCore,
    /// External Composio tools.
    #[serde(rename = "external_composio")]
    ExternalComposio,
    /// External LangChain tools.
    #[serde(rename = "external_langchain")]
    ExternalLangchain,
    /// External MCP tools.
    #[serde(rename = "external_mcp")]
    ExternalMcp,
    /// Custom tools.
    Custom,
    /// Other tool types.
    #[serde(other)]
    Other,
}

/// Source type enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SourceType {
    /// Python source code.
    Python,
    /// JavaScript source code.
    JavaScript,
    /// Other source types.
    #[serde(other)]
    Other,
}

/// Pip requirement for a tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipRequirement {
    /// Package name.
    pub package: String,
    /// Version constraint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Tool definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    /// Tool ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<LettaId>,
    /// Tool type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_type: Option<ToolType>,
    /// Tool description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Source type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<SourceType>,
    /// Organization ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<LettaId>,
    /// Tool name.
    pub name: String,
    /// Tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Source code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code: Option<String>,
    /// JSON schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_schema: Option<serde_json::Value>,
    /// Args JSON schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args_json_schema: Option<serde_json::Value>,
    /// Return character limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_char_limit: Option<u32>,
    /// Pip requirements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pip_requirements: Option<Vec<PipRequirement>>,
    /// Created by user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_id: Option<LettaId>,
    /// Last updated by user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by_id: Option<LettaId>,
    /// Tool metadata.
    #[serde(skip_serializing_if = "Option::is_none", rename = "metadata_")]
    pub metadata: Option<Metadata>,
    /// When the tool was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    /// When the tool was last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
}

/// Tool creation request.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateToolRequest {
    /// Tool description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Source code.
    pub source_code: String,
    /// Source type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<SourceType>,
    /// JSON schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_schema: Option<serde_json::Value>,
    /// Args JSON schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args_json_schema: Option<serde_json::Value>,
    /// Return character limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_char_limit: Option<u32>,
    /// Pip requirements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pip_requirements: Option<Vec<PipRequirement>>,
}

/// Query parameters for listing tools.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListToolsParams {
    /// Limit number of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Pagination cursor (before).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Pagination cursor (after).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Filter by tool name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Tool update request.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateToolRequest {
    /// Tool description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Source code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code: Option<String>,
    /// Tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Return character limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_char_limit: Option<u32>,
    /// Pip requirements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pip_requirements: Option<Vec<PipRequirement>>,
    /// Tool metadata.
    #[serde(skip_serializing_if = "Option::is_none", rename = "metadata_")]
    pub metadata: Option<Metadata>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_tool_serialization() {
        let tool = Tool {
            id: Some(LettaId::from_str("tool-550e8400-e29b-41d4-a716-446655440000").unwrap()),
            tool_type: Some(ToolType::Custom),
            name: "calculator".to_string(),
            description: Some("Basic calculator tool".to_string()),
            source_type: Some(SourceType::Python),
            organization_id: None,
            tags: Some(vec!["math".to_string(), "utility".to_string()]),
            source_code: Some("def calculate(x, y): return x + y".to_string()),
            json_schema: Some(serde_json::json!({
                "name": "calculator",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "x": {"type": "number"},
                        "y": {"type": "number"}
                    }
                }
            })),
            args_json_schema: None,
            return_char_limit: Some(1000),
            pip_requirements: None,
            created_by_id: None,
            last_updated_by_id: None,
            metadata: None,
            created_at: Some(chrono::Utc::now()),
            updated_at: None,
        };

        let json = serde_json::to_string(&tool).unwrap();
        let deserialized: Tool = serde_json::from_str(&json).unwrap();
        assert_eq!(tool.name, deserialized.name);
    }

    #[test]
    fn test_tool_type_serialization() {
        assert_eq!(
            serde_json::to_string(&ToolType::LettaCore).unwrap(),
            "\"letta_core\""
        );
        assert_eq!(
            serde_json::to_string(&ToolType::Custom).unwrap(),
            "\"custom\""
        );
        assert_eq!(
            serde_json::to_string(&ToolType::LettaFilesCore).unwrap(),
            "\"letta_files_core\""
        );
    }

    #[test]
    fn test_create_tool_request() {
        let request = CreateToolRequest {
            description: Some("My custom tool".to_string()),
            source_code: "def my_tool(): pass".to_string(),
            source_type: Some(SourceType::Python),
            json_schema: Some(serde_json::json!({
                "name": "my_tool",
                "parameters": {"type": "object"}
            })),
            tags: Some(vec!["custom".to_string()]),
            return_char_limit: None,
            pip_requirements: None,
            args_json_schema: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"source_code\":\"def my_tool(): pass\""));
    }
}
