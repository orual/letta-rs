//! Agent-related types.

use crate::types::common::{Metadata, ResourceId, Timestamp};
use crate::types::memory::MemoryBlock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Environment variable for agent tool execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentEnvironmentVariable {
    /// The ID of the user that created this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_id: Option<String>,
    /// The ID of the user that last updated this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by_id: Option<String>,
    /// When the object was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    /// When the object was last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
    /// The human-friendly ID of the agent environment variable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the environment variable.
    pub key: String,
    /// The value of the environment variable.
    pub value: String,
    /// An optional description of the environment variable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the agent this environment variable belongs to.
    pub agent_id: String,
}

/// Agent type enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]

pub enum AgentType {
    /// Standard memgpt-style agent.
    #[serde(rename = "memgpt_agent")]
    MemGPT,
    /// version 2 memgpt agent.
    #[serde(rename = "memgpt_v2_agent")]
    MemGPTv2,
    /// React agent.
    #[serde(rename = "react_agent")]
    React,
    /// Workflow agent
    #[serde(rename = "workflow_agent")]
    Workflow,
    /// Split-thread agent
    #[serde(rename = "split_thread_agent")]
    SplitThread,
    /// Sleeptime agent
    #[serde(rename = "sleeptime_agent")]
    Sleeptime,
    /// Voice Conversation agent
    #[serde(rename = "voice_convo_agent")]
    VoiceConvo,
    /// Voice sleeptime agent
    #[serde(rename = "voice_sleeptime_agent")]
    VoiceSleeptime,
    /// Other agent types.
    #[serde(other)]
    Other,
}

impl Default for AgentType {
    fn default() -> Self {
        Self::MemGPT
    }
}

/// LLM configuration for an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    /// Model name/identifier.
    pub model: String,
    /// Model endpoint type.
    pub model_endpoint_type: ModelEndpointType,
    /// Model endpoint URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_endpoint: Option<String>,
    /// Context window size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_window: Option<u32>,
    /// Provider name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    /// Provider category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_category: Option<String>,
    /// Model wrapper.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_wrapper: Option<String>,
    /// Put inner thoughts in kwargs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put_inner_thoughts_in_kwargs: Option<bool>,
    /// Handle.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    /// Temperature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Max tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    /// Enable reasoner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_reasoner: Option<bool>,
    /// Reasoning effort.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<String>,
    /// Max reasoning tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_reasoning_tokens: Option<u32>,
    /// Additional configuration.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Available Model endpoints
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ModelEndpointType {
    /// Open AI
    Openai,
    /// Anthropic
    Anthropic,
    /// Cohere
    Cohere,
    /// Google AI
    GoogleAi,
    /// Google Vertex AI
    GoogleVertex,
    /// Azure AI
    Azure,
    /// Groq
    Groq,
    /// Ollama
    Ollama,
    /// Open Webui
    Webui,
    /// Open Webui (legacy api)
    #[serde(rename = "webui-legacy")]
    WebuiLegacy,
    /// LM Studio
    Lmstudio,
    /// LM Studio (legacy api)
    #[serde(rename = "lmstudio-legacy")]
    LmstudioLegacy,
    /// LM Studio (chat completions API)
    #[serde(rename = "lmstudio-chatcompletions")]
    LmstudioChatCompletions,
    /// Llamacpp
    Llamacpp,
    /// Koboldcpp
    Koboldcpp,
    /// vLLM
    Vllm,
    /// Hugging Face
    #[serde(rename = "hugging-face")]
    HuggingFace,
    /// Mistral
    Mistral,
    /// Together
    Together,
    /// Other model types.
    #[serde(other)]
    Other,
}

/// Available embedding endpoints
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EmbeddingEndpointType {
    /// OpenAI
    Openai,
    /// Azure
    Azure,
    /// Cohere
    Cohere,
    /// HuggingFace
    #[serde(rename = "hugging-face")]
    HuggingFace,
    /// Ollama
    Ollama,
    /// Other embedding types
    #[serde(other)]
    Other,
}

/// Azure-specific embedding configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AzureEmbeddingConfig {
    /// Azure endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_endpoint: Option<String>,
    /// Azure version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_version: Option<String>,
    /// Azure deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_deployment: Option<String>,
}

/// Embedding configuration for an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingConfig {
    /// Embedding model name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_model: Option<String>,
    /// Embedding endpoint type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_endpoint_type: Option<EmbeddingEndpointType>,
    /// Embedding endpoint URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_endpoint: Option<String>,
    /// Embedding dimension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_dim: Option<u32>,
    /// Embedding chunk size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_chunk_size: Option<u32>,
    /// Handle.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    /// Azure-specific configuration (flattened into the JSON).
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub azure_config: Option<AzureEmbeddingConfig>,
    /// Additional configuration.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Agent memory configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMemory {
    /// Memory blocks.
    #[serde(default)]
    pub blocks: Vec<MemoryBlock>,
    /// File-based memory blocks.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub file_blocks: Vec<MemoryBlock>,
    /// Prompt template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_template: Option<String>,
}

/// Tool reference for agents.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolReference {
    /// Tool ID reference.
    Id(String),
    /// Full tool object.
    Object(serde_json::Value),
}

/// Tool rule for controlling agent behavior.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ToolRule {
    /// Continue loop after tool execution.
    #[serde(rename = "continue_loop")]
    ContinueLoop {
        /// Tool name this rule applies to.
        tool_name: String,
        /// Optional prompt template.
        #[serde(skip_serializing_if = "Option::is_none")]
        prompt_template: Option<String>,
    },
    /// Exit loop after tool execution.
    #[serde(rename = "exit_loop")]
    ExitLoop {
        /// Tool name this rule applies to.
        tool_name: String,
        /// Optional prompt template.
        #[serde(skip_serializing_if = "Option::is_none")]
        prompt_template: Option<String>,
    },
    /// Terminal tool rule (deprecated, use exit_loop).
    Terminal {
        /// Tool name this rule applies to.
        tool_name: String,
        /// Optional prompt template.
        #[serde(skip_serializing_if = "Option::is_none")]
        prompt_template: Option<String>,
    },
    /// Max count per step constraint.
    MaxCountPerStep {
        /// Tool name this rule applies to.
        tool_name: String,
        /// Optional prompt template.
        #[serde(skip_serializing_if = "Option::is_none")]
        prompt_template: Option<String>,
        /// The max limit for the total number of times this tool can be invoked in a single step.
        max_count_limit: u32,
    },
    /// Conditional tool mapping based on output.
    Conditional {
        /// Tool name this rule applies to.
        tool_name: String,
        /// Optional prompt template.
        #[serde(skip_serializing_if = "Option::is_none")]
        prompt_template: Option<String>,
        /// The default child tool to be called.
        #[serde(skip_serializing_if = "Option::is_none")]
        default_child: Option<String>,
        /// The output case to check for mapping.
        child_output_mapping: HashMap<String, String>,
        /// Whether to throw an error when output doesn't match any case.
        #[serde(default)]
        require_output_mapping: bool,
    },
    /// Child tool rule.
    Child {
        /// Tool name this rule applies to.
        tool_name: String,
        /// Optional prompt template.
        #[serde(skip_serializing_if = "Option::is_none")]
        prompt_template: Option<String>,
        /// The name of the tool that can be a child of this tool.
        child_tool_name: String,
    },
    /// Parent tool rule.
    Parent {
        /// Tool name this rule applies to.
        tool_name: String,
        /// Optional prompt template.
        #[serde(skip_serializing_if = "Option::is_none")]
        prompt_template: Option<String>,
        /// The name of the tool that can be a parent of this tool.
        parent_tool_name: String,
    },
    /// Required before exit rule.
    RequiredBeforeExit {
        /// Tool name this rule applies to.
        tool_name: String,
        /// Optional prompt template.
        #[serde(skip_serializing_if = "Option::is_none")]
        prompt_template: Option<String>,
    },
    /// Init tool rule.
    Init {
        /// Tool name this rule applies to.
        tool_name: String,
        /// Optional prompt template.
        #[serde(skip_serializing_if = "Option::is_none")]
        prompt_template: Option<String>,
    },
}

/// Response format type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseFormatType {
    /// Text response.
    Text,
    /// JSON object response.
    JsonObject,
}

/// Response format configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseFormat {
    /// Response type.
    #[serde(rename = "type")]
    pub format_type: ResponseFormatType,
    /// JSON schema for validation (when type is json_object).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_schema: Option<serde_json::Value>,
}

/// Agent state and configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    /// Unique identifier for the agent.
    pub id: ResourceId,
    /// Agent name.
    pub name: String,
    /// System prompt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    /// Agent type.
    #[serde(default)]
    pub agent_type: AgentType,
    /// LLM configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm_config: Option<LLMConfig>,
    /// Embedding configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_config: Option<EmbeddingConfig>,
    /// Agent memory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<AgentMemory>,
    /// Available tools.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<ToolReference>,
    /// Data sources.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<serde_json::Value>,
    /// Agent tags.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    /// Agent description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Agent metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    /// Project ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    /// Created by user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_id: Option<String>,
    /// Last updated by user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by_id: Option<String>,
    /// When the agent was created.
    pub created_at: Timestamp,
    /// When the agent was last updated.
    pub updated_at: Timestamp,
    /// Tool rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_rules: Option<Vec<ToolRule>>,
    /// Message IDs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub message_ids: Vec<String>,
    /// Multi-agent group configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_agent_group: Option<serde_json::Value>,
    /// Template ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    /// Base template ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_template_id: Option<String>,
    /// Identity IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_ids: Option<Vec<String>>,
    /// Tool execution environment variables.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_exec_environment_variables: Option<Vec<AgentEnvironmentVariable>>,
    /// Organization ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// Timezone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Last run completion time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_completion: Option<Timestamp>,
    /// Last run duration in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_duration_ms: Option<u64>,
    /// Enable sleeptime.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_sleeptime: Option<bool>,
    /// Response format configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    /// Message buffer autoclear setting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_buffer_autoclear: Option<bool>,
}

/// Request to create a new agent.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateAgentRequest {
    /// Agent name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// System prompt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    /// Agent type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_type: Option<AgentType>,
    /// LLM configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm_config: Option<LLMConfig>,
    /// Embedding configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_config: Option<EmbeddingConfig>,
    /// Memory blocks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_blocks: Option<Vec<MemoryBlock>>,
    /// Tools to attach.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<String>>,
    /// Tool IDs to attach (alternative to tools).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_ids: Option<Vec<String>>,
    /// Source IDs to attach.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ids: Option<Vec<String>>,
    /// Agent tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Agent description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Agent metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    /// Agent timezone (IANA format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Include base tools.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_base_tools: Option<bool>,
    /// Include multi-agent tools.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_multi_agent_tools: Option<bool>,
    /// Include base tool rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_base_tool_rules: Option<bool>,
    /// Include default source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_default_source: Option<bool>,
    /// Tool rules to apply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_rules: Option<Vec<ToolRule>>,
    /// Initial message sequence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_message_sequence: Option<Vec<serde_json::Value>>,
    /// Tool execution environment variables.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_exec_environment_variables: Option<HashMap<String, String>>,
    /// Response format configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    /// Enable reasoner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_reasoner: Option<bool>,
    /// Message buffer auto-clear.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_buffer_autoclear: Option<bool>,
    /// Block IDs to attach.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_ids: Option<Vec<String>>,
    /// Model shorthand (alternative to llm_config).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Embedding shorthand (alternative to embedding_config).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding: Option<String>,
    /// Context window limit shorthand.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_window_limit: Option<u32>,
    /// Embedding chunk size shorthand.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_chunk_size: Option<u32>,
    /// Max tokens shorthand.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    /// Max reasoning tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_reasoning_tokens: Option<u32>,
    /// Create from template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_template: Option<String>,
    /// Mark as template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<bool>,
    /// Memory variables.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_variables: Option<HashMap<String, String>>,
    /// Project ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    /// Template ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    /// Base template ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_template_id: Option<String>,
    /// Identity IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_ids: Option<Vec<String>>,
    /// Enable sleeptime.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_sleeptime: Option<bool>,
}

impl CreateAgentRequest {
    /// Create a new agent request builder.
    pub fn builder() -> CreateAgentRequestBuilder {
        CreateAgentRequestBuilder::default()
    }
}

/// Builder for CreateAgentRequest.
#[derive(Debug, Default)]
pub struct CreateAgentRequestBuilder {
    request: CreateAgentRequest,
}

impl CreateAgentRequestBuilder {
    /// Set the agent name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.request.name = Some(name.into());
        self
    }

    /// Set the system prompt.
    pub fn system(mut self, system: impl Into<String>) -> Self {
        self.request.system = Some(system.into());
        self
    }

    /// Set the agent type.
    pub fn agent_type(mut self, agent_type: AgentType) -> Self {
        self.request.agent_type = Some(agent_type);
        self
    }

    /// Set the LLM configuration.
    pub fn llm_config(mut self, config: LLMConfig) -> Self {
        self.request.llm_config = Some(config);
        self
    }

    /// Set the embedding configuration.
    pub fn embedding_config(mut self, config: EmbeddingConfig) -> Self {
        self.request.embedding_config = Some(config);
        self
    }

    /// Add memory blocks.
    pub fn memory_blocks(mut self, blocks: Vec<MemoryBlock>) -> Self {
        self.request.memory_blocks = Some(blocks);
        self
    }

    /// Add a single memory block.
    pub fn memory_block(mut self, block: MemoryBlock) -> Self {
        self.request
            .memory_blocks
            .get_or_insert_with(Vec::new)
            .push(block);
        self
    }

    /// Add tools.
    pub fn tools(mut self, tools: Vec<String>) -> Self {
        self.request.tools = Some(tools);
        self
    }

    /// Add tags.
    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.request.tags = Some(tags);
        self
    }

    /// Set the description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.description = Some(description.into());
        self
    }

    /// Set the timezone.
    pub fn timezone(mut self, timezone: impl Into<String>) -> Self {
        self.request.timezone = Some(timezone.into());
        self
    }

    /// Set include base tools.
    pub fn include_base_tools(mut self, include: bool) -> Self {
        self.request.include_base_tools = Some(include);
        self
    }

    /// Set tool rules.
    pub fn tool_rules(mut self, rules: Vec<ToolRule>) -> Self {
        self.request.tool_rules = Some(rules);
        self
    }

    /// Set initial message sequence.
    pub fn initial_message_sequence(mut self, sequence: Vec<serde_json::Value>) -> Self {
        self.request.initial_message_sequence = Some(sequence);
        self
    }

    /// Set tool execution environment variables.
    pub fn tool_exec_environment_variables(mut self, vars: HashMap<String, String>) -> Self {
        self.request.tool_exec_environment_variables = Some(vars);
        self
    }

    /// Build the request.
    pub fn build(self) -> CreateAgentRequest {
        self.request
    }
}

/// Request to update an agent.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateAgentRequest {
    /// Agent name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// System prompt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    /// Agent type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_type: Option<AgentType>,
    /// LLM configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm_config: Option<LLMConfig>,
    /// Embedding configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_config: Option<EmbeddingConfig>,
    /// Agent tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Agent description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Agent metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

/// Request parameters for importing an agent from file.
#[derive(Debug, Clone, Default)]
pub struct ImportAgentRequest {
    /// Whether to append a copy suffix to the agent name if it already exists.
    pub append_copy_suffix: Option<bool>,
    /// Whether to override existing tools with the same name.
    pub override_existing_tools: Option<bool>,
    /// The project ID to associate the uploaded agent with.
    pub project_id: Option<String>,
    /// If set to True, strips all messages from the agent before importing.
    pub strip_messages: Option<bool>,
}

/// Search response for agents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentsSearchResponse {
    /// List of matching agents.
    pub agents: Vec<Agent>,
    /// Cursor for pagination.
    #[serde(rename = "nextCursor")]
    pub next_cursor: Option<String>,
}

/// Search request for agents.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentsSearchRequest {
    /// Search criteria (simplified for now).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<Vec<String>>,
    /// Project ID filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    /// Search combinator (only "AND" supported).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub combinator: Option<String>,
    /// Maximum number of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Pagination cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Sort field.
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending: Option<bool>,
}

/// Query parameters for listing agents.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListAgentsParams {
    /// Filter by name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Filter by tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Whether to match all tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_all_tags: Option<bool>,
    /// Pagination cursor (before).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Pagination cursor (after).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Pagination limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Search query text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_text: Option<String>,
    /// Filter by project ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    /// Filter by template ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    /// Filter by base template ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_template_id: Option<String>,
    /// Filter by identity ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<String>,
    /// Search by identifier keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier_keys: Option<Vec<String>>,
    /// Include relationships.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_relationships: Option<Vec<String>>,
    /// Sort order (ascending).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending: Option<bool>,
    /// Sort by field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

impl ListAgentsParams {
    /// Create a new list agents params builder.
    pub fn builder() -> ListAgentsParamsBuilder {
        ListAgentsParamsBuilder::default()
    }
}

/// Builder for ListAgentsParams.
#[derive(Debug, Default)]
pub struct ListAgentsParamsBuilder {
    params: ListAgentsParams,
}

impl ListAgentsParamsBuilder {
    /// Filter by name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.params.name = Some(name.into());
        self
    }

    /// Filter by tags.
    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.params.tags = Some(tags);
        self
    }

    /// Set pagination limit.
    pub fn limit(mut self, limit: u32) -> Self {
        self.params.limit = Some(limit);
        self
    }

    /// Set search query.
    pub fn query_text(mut self, query: impl Into<String>) -> Self {
        self.params.query_text = Some(query.into());
        self
    }

    /// Build the params.
    pub fn build(self) -> ListAgentsParams {
        self.params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_serialization() {
        let agent = Agent {
            id: "agent-00000000-0000-0000-0000-000000000000".to_string(),
            name: "Test Agent".to_string(),
            system: Some("You are a helpful assistant".to_string()),
            agent_type: AgentType::MemGPT,
            llm_config: Some(LLMConfig {
                model: "gpt-4".to_string(),
                model_endpoint_type: ModelEndpointType::Openai,
                model_endpoint: None,
                context_window: Some(8192),
                provider_name: None,
                provider_category: None,
                model_wrapper: None,
                put_inner_thoughts_in_kwargs: None,
                handle: None,
                temperature: None,
                max_tokens: None,
                enable_reasoner: None,
                reasoning_effort: None,
                max_reasoning_tokens: None,
                extra: HashMap::new(),
            }),
            embedding_config: None,
            memory: None,
            tools: vec![],
            sources: vec![],
            tags: vec!["test".to_string()],
            description: Some("A test agent".to_string()),
            metadata: None,
            project_id: None,
            created_by_id: None,
            last_updated_by_id: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            tool_rules: None,
            message_ids: vec![],
            multi_agent_group: None,
            template_id: None,
            base_template_id: None,
            identity_ids: None,
            tool_exec_environment_variables: None,
            organization_id: None,
            timezone: None,
            last_run_completion: None,
            last_run_duration_ms: None,
            enable_sleeptime: None,
            response_format: None,
            message_buffer_autoclear: None,
        };

        let json = serde_json::to_string(&agent).unwrap();
        let deserialized: Agent = serde_json::from_str(&json).unwrap();
        assert_eq!(agent.name, deserialized.name);
    }

    #[test]
    fn test_create_agent_request_builder() {
        let request = CreateAgentRequest::builder()
            .name("My Agent")
            .system("You are helpful")
            .agent_type(AgentType::MemGPTv2)
            .memory_block(MemoryBlock {
                id: None,
                label: "human".to_string(),
                value: "The human's name is Bob.".to_string(),
                limit: Some(5000),
                is_template: false,
                preserve_on_migration: false,
                read_only: false,
                description: None,
                metadata: None,
                name: None,
                organization_id: None,
                created_by_id: None,
                last_updated_by_id: None,
                created_at: None,
                updated_at: None,
            })
            .memory_block(MemoryBlock {
                id: None,
                label: "persona".to_string(),
                value: "I am Sam, a helpful assistant.".to_string(),
                limit: Some(5000),
                is_template: false,
                preserve_on_migration: false,
                read_only: false,
                description: None,
                metadata: None,
                name: None,
                organization_id: None,
                created_by_id: None,
                last_updated_by_id: None,
                created_at: None,
                updated_at: None,
            })
            .tags(vec!["test".to_string(), "demo".to_string()])
            .build();

        assert_eq!(request.name.as_deref(), Some("My Agent"));
        assert_eq!(request.memory_blocks.as_ref().unwrap().len(), 2);
        assert_eq!(request.tags.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_agent_type_serialization() {
        let agent_type = AgentType::Sleeptime;
        let json = serde_json::to_string(&agent_type).unwrap();
        assert_eq!(json, r#""sleeptime_agent""#);

        let deserialized: AgentType = serde_json::from_str(&json).unwrap();
        assert_eq!(agent_type, deserialized);
    }

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
    fn test_list_agents_params_builder() {
        let params = ListAgentsParams::builder()
            .name("test")
            .tags(vec!["production".to_string()])
            .limit(50)
            .query_text("search term")
            .build();

        assert_eq!(params.name.as_deref(), Some("test"));
        assert_eq!(params.limit, Some(50));
    }

    #[test]
    fn test_embedding_endpoint_type_serialization() {
        let endpoint_type = EmbeddingEndpointType::Openai;
        let json = serde_json::to_string(&endpoint_type).unwrap();
        assert_eq!(json, r#""openai""#);

        let deserialized: EmbeddingEndpointType = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, EmbeddingEndpointType::Openai);
    }

    #[test]
    fn test_model_endpoint_type_serialization() {
        let endpoint_type = ModelEndpointType::Anthropic;
        let json = serde_json::to_string(&endpoint_type).unwrap();
        assert_eq!(json, r#""anthropic""#);

        let deserialized: ModelEndpointType = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, ModelEndpointType::Anthropic);
    }

    #[test]
    fn test_azure_embedding_config_flattening() {
        let config = EmbeddingConfig {
            embedding_model: Some("text-embedding-ada-002".to_string()),
            embedding_endpoint_type: Some(EmbeddingEndpointType::Azure),
            embedding_endpoint: Some("https://myazure.openai.azure.com".to_string()),
            embedding_dim: Some(1536),
            embedding_chunk_size: Some(300),
            handle: None,
            azure_config: Some(AzureEmbeddingConfig {
                azure_endpoint: Some("https://myazure.openai.azure.com".to_string()),
                azure_version: Some("2023-05-15".to_string()),
                azure_deployment: Some("my-deployment".to_string()),
            }),
            extra: HashMap::new(),
        };

        let json = serde_json::to_string(&config).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        // Verify Azure fields are flattened to top level
        assert_eq!(parsed["azure_endpoint"], "https://myazure.openai.azure.com");
        assert_eq!(parsed["azure_version"], "2023-05-15");
        assert_eq!(parsed["azure_deployment"], "my-deployment");

        // Verify we can deserialize back
        let deserialized: EmbeddingConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(
            deserialized
                .azure_config
                .as_ref()
                .unwrap()
                .azure_version
                .as_deref(),
            Some("2023-05-15")
        );
    }

    #[test]
    fn test_create_agent_request_shorthand_fields() {
        let request = CreateAgentRequest {
            name: Some("Test Agent".to_string()),
            model: Some("gpt-4".to_string()),
            embedding: Some("text-embedding-ada-002".to_string()),
            context_window_limit: Some(8192),
            embedding_chunk_size: Some(512),
            max_tokens: Some(2048),
            from_template: Some("customer-support".to_string()),
            template: Some(true),
            ..Default::default()
        };

        let json = serde_json::to_string(&request).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["model"], "gpt-4");
        assert_eq!(parsed["embedding"], "text-embedding-ada-002");
        assert_eq!(parsed["context_window_limit"], 8192);
        assert_eq!(parsed["template"], true);
    }
}
