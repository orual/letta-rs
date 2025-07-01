//! # Letta Rust Client
//!
//! A robust, fully-featured Rust client for the [Letta REST API](https://docs.letta.com/api-reference/overview).
//!
//! Letta is a platform for building stateful AI agents with persistent memory and context
//! across conversations. This client provides a comprehensive, idiomatic Rust interface
//! to all Letta API endpoints with full type safety.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use letta::{LettaClient, LettaEnvironment};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Connect to local Letta server
//!     let client = LettaClient::local()?;
//!
//!     // Or connect to Letta Cloud with an API token
//!     // let client = LettaClient::cloud("your-api-token")?;
//!
//!     // Or use the builder for custom configuration
//!     // let client = LettaClient::builder()
//!     //     .environment(LettaEnvironment::Cloud)
//!     //     .auth(letta::auth::AuthConfig::bearer("your-token"))
//!     //     .base_url("https://custom.letta.com")  // optional override
//!     //     .build()?;
//!
//!     // List all agents
//!     let agents = client.agents().list(None).await?;
//!     println!("Found {} agents", agents.len());
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Creating and Interacting with Agents
//!
//! ```rust,no_run
//! use letta::{LettaClient, types::*};
//! use futures::StreamExt;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = LettaClient::local()?;
//!
//!     // Create a new agent
//!     let agent_request = CreateAgentRequest {
//!         name: Some("Assistant".to_string()),
//!         agent_type: Some(AgentType::MemGPT),
//!         model: Some("letta/letta-free".to_string()),
//!         embedding: Some("letta/letta-free".to_string()),
//!         ..Default::default()
//!     };
//!
//!     let agent = client.agents().create(agent_request).await?;
//!     println!("Created agent: {}", agent.id);
//!
//!     // Send a simple message
//!     let request = CreateMessagesRequest {
//!         messages: vec![MessageCreate::user("Hello! Can you help me with Rust?")],
//!         ..Default::default()
//!     };
//!     
//!     // Or create a more complex message with multiple content parts
//!     let complex_message = MessageCreate {
//!         role: MessageRole::User,
//!         content: MessageCreateContent::ContentParts(vec![
//!             ContentPart::Text(TextContent {
//!                 text: "Here's an image that shows my error:".to_string(),
//!             }),
//!             ContentPart::Image(ImageContent {
//!                 image_url: ImageUrl {
//!                     url: "https://example.com/error-screenshot.png".to_string(),
//!                     detail: None,
//!                 },
//!             }),
//!         ]),
//!         name: Some("Developer".to_string()),
//!         ..Default::default()
//!     };
//!     
//!     let response = client
//!         .messages()
//!         .create(&agent.id, request)
//!         .await?;
//!
//!     // Process the response messages
//!     for message in response.messages {
//!         match &message {
//!             LettaMessageUnion::ReasoningMessage(m) => {
//!                 println!("Thinking: {}", m.reasoning);
//!             }
//!             LettaMessageUnion::ToolReturnMessage(t) => {
//!                 println!("Tool returned: {:?}", t.tool_return);
//!             }
//!             LettaMessageUnion::AssistantMessage(m) => {
//!                 println!("Assistant: {}", m.content);
//!             }
//!             _ => {} // Other message types
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Memory Management
//!
//! ```rust,no_run
//! use letta::{LettaClient, types::*};
//! use std::str::FromStr;
//! use futures::StreamExt;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = LettaClient::local()?;
//!     let agent_id = LettaId::from_str("agent-00000000-0000-0000-0000-000000000000")?;
//!
//!     // Get a specific core memory block
//!     let block = client
//!         .memory()
//!         .get_core_memory_block(&agent_id, "human")
//!         .await?;
//!     
//!     println!("Current human memory: {}", block.value);
//!
//!     // Update core memory block
//!     let request = UpdateMemoryBlockRequest {
//!         value: Some("Name: Bob\nRole: Manager".to_string()),
//!         label: None,
//!         limit: None,
//!         name: None,
//!         preserve_on_migration: None,
//!         read_only: None,
//!         description: None,
//!         metadata: None,
//!     };
//!     let updated = client
//!         .memory()
//!         .update_core_memory_block(&agent_id, "human", request)
//!         .await?;
//!     
//!     println!("Updated memory block: {}", updated.id.as_ref().unwrap());
//!
//!     // Add to archival memory
//!     let archival_request = CreateArchivalMemoryRequest {
//!         text: "Important: Project deadline is next Friday".to_string(),
//!     };
//!     let memories = client
//!         .memory()
//!         .create_archival_memory(&agent_id, archival_request)
//!         .await?;
//!     
//!     println!("Inserted {} archival memory passages", memories.len());
//!
//!     // Search archival memory with semantic search
//!     let search_params = PaginationParams {
//!         limit: Some(5),
//!         ..Default::default()
//!     };
//!     let mut stream = client
//!         .memory()
//!         .archival_paginated(&agent_id, Some(search_params));
//!
//!     while let Some(result) = stream.next().await {
//!         let result = result?;
//!         println!("Found: {}", result.text);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Pagination Support
//!
//! ```rust,no_run
//! use letta::{LettaClient, types::*};
//! use futures::StreamExt;
//! use std::str::FromStr;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = LettaClient::local()?;
//!
//!     // Use pagination to handle large result sets
//!     let params = PaginationParams {
//!         limit: Some(10),
//!         ..Default::default()
//!     };
//!     let mut agent_stream = client
//!         .agents()
//!         .paginated(Some(params));
//!
//!     // Automatically fetches next pages as needed
//!     while let Some(agent) = agent_stream.next().await {
//!         let agent = agent?;
//!         println!("Agent: {} ({})", agent.name, agent.id);
//!     }
//!
//!     // Also works with archival memory
//!     let agent_id = LettaId::from_str("agent-00000000-0000-0000-0000-000000000000")?;
//!     let memory_params = PaginationParams {
//!         limit: Some(20),
//!         ..Default::default()
//!     };
//!     let mut memory_stream = client
//!         .memory()
//!         .archival_paginated(&agent_id, Some(memory_params));
//!
//!     while let Some(memory) = memory_stream.next().await {
//!         let memory = memory?;
//!         println!("Memory: {}", memory.text);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Features
//!
//! - **Complete API Coverage**: All 20+ Letta REST API endpoints
//! - **Async/Await**: Full async support with tokio
//! - **Streaming**: Server-sent events for real-time messaging
//! - **Pagination**: Automatic cursor-based pagination with `PaginatedStream`
//! - **Type Safety**: Comprehensive type definitions with serde
//! - **Error Handling**: Rich error types with miette diagnostics
//! - **Authentication**: Bearer token and API key support
//! - **Documentation**: Extensive docs with examples
//!
//! ## API Sections
//!
//! - [`agents`](crate::api::agents) - Agent management and lifecycle
//! - [`messages`](crate::api::messages) - Real-time messaging with streaming
//! - [`memory`](crate::api::memory) - Memory management (core, archival, blocks)
//! - [`tools`](crate::api::tools) - Tool management and execution
//! - [`sources`](crate::api::sources) - Document and data source management
//! - [`blocks`](crate::api::blocks) - Memory block operations
//! - [`groups`](crate::api::groups) - Multi-agent conversations
//! - [`jobs`](crate::api::jobs) - Asynchronous job management
//! - [`projects`](crate::api::projects) - Project organization
//! - [`templates`](crate::api::templates) - Agent templates
//! - [`runs`](crate::api::runs) - Execution tracking
//! - [`models`](crate::api::models) - Model configuration
//! - [`providers`](crate::api::providers) - LLM provider management
//! - [`identities`](crate::api::identities) - Identity management
//! - [`tags`](crate::api::tags) - Tag-based organization
//! - [`batch`](crate::api::batch) - Batch processing
//! - [`telemetry`](crate::api::telemetry) - Usage tracking
//! - [`voice`](crate::api::voice) - Voice conversations (beta)
//!
//! ## Error Handling
//!
//! All operations return [`Result<T, LettaError>`](crate::error::LettaError) with
//! comprehensive error information and suggestions via [`miette`].

#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::module_name_repetitions)]

pub mod api;
pub mod auth;
pub mod client;
pub mod environment;
pub mod error;
pub mod pagination;
pub mod retry;
pub mod streaming;
pub mod types;
pub mod utils;

#[cfg(test)]
pub mod test_helpers;

// Re-export main types for convenience
pub use client::{ClientBuilder, ClientConfig, LettaClient};
pub use environment::LettaEnvironment;
pub use error::{ErrorContext, LettaError, LettaResult};
pub use types::*;

/// Maximum number of retries for API calls
pub const MAX_RETRIES: u32 = 3;

/// Convenience type alias for Results in this crate.
pub type Result<T> = std::result::Result<T, LettaError>;

// Re-export streaming types
pub use api::messages::{MessageStream, StreamingEvent};
