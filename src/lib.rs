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
//! use letta_rs::{LettaClient, LettaEnvironment};
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
//!     //     .auth(letta_rs::auth::AuthConfig::bearer("your-token"))
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
//! use letta_rs::{LettaClient, types::*};
//! use serde_json::json;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = LettaClient::local()?;
//!
//!     // Create a new agent with custom memory
//!     let agent_request = CreateAgentRequest {
//!         name: "Assistant".to_string(),
//!         agent_type: Some(AgentType::MemGPT),
//!         llm_config: Some(json!({
//!             "model_endpoint_type": "openai",
//!             "model": "gpt-4",
//!         })),
//!         memory_blocks: Some(vec![
//!             CreateBlock {
//!                 block_type: BlockType::Human,
//!                 value: "Name: Alice\nRole: Developer".to_string(),
//!                 label: Some("human".to_string()),
//!                 ..Default::default()
//!             },
//!             CreateBlock {
//!                 block_type: BlockType::Persona,
//!                 value: "You are a helpful coding assistant.".to_string(),
//!                 label: Some("persona".to_string()),
//!                 ..Default::default()
//!             }
//!         ]),
//!         ..Default::default()
//!     };
//!
//!     let agent = client.agents().create(agent_request).await?;
//!     println!("Created agent: {}", agent.id);
//!
//!     // Send a message and stream the response
//!     let stream = client
//!         .messages()
//!         .send_streamed(&agent.id, "Hello! Can you help me with Rust?", None)
//!         .await?;
//!
//!     // Handle streaming responses
//!     tokio::pin!(stream);
//!     while let Some(event) = stream.next().await {
//!         match event? {
//!             StreamingEvent::AssistantMessage(msg) => {
//!                 print!("{}", msg.message);
//!             }
//!             StreamingEvent::FunctionCall(call) => {
//!                 println!("\nCalling function: {}", call.name);
//!             }
//!             _ => {}
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
//! use letta_rs::{LettaClient, types::*};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = LettaClient::local()?;
//!     let agent_id = LettaId::new("agent-123");
//!
//!     // Update core memory
//!     client
//!         .memory()
//!         .update_core_memory(&agent_id, "human", "Name: Bob\nRole: Manager")
//!         .await?;
//!
//!     // Add to archival memory
//!     let memory = client
//!         .memory()
//!         .insert_archival_memory(&agent_id, "Important: Project deadline is next Friday")
//!         .await?;
//!
//!     // Search archival memory with semantic search
//!     let results = client
//!         .memory()
//!         .search_archival_memory(&agent_id, "project deadline", Some(5))
//!         .await?;
//!
//!     for result in results {
//!         println!("Found: {} (relevance: {})", result.text, result.score.unwrap_or(0.0));
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Pagination Support
//!
//! ```rust,no_run
//! use letta_rs::LettaClient;
//! use futures::StreamExt;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = LettaClient::local()?;
//!
//!     // Use pagination to handle large result sets
//!     let mut agent_stream = client
//!         .agents()
//!         .paginated()
//!         .limit(10)
//!         .build();
//!
//!     // Automatically fetches next pages as needed
//!     while let Some(agent) = agent_stream.next().await {
//!         let agent = agent?;
//!         println!("Agent: {} ({})", agent.name, agent.id);
//!     }
//!
//!     // Also works with archival memory
//!     let agent_id = letta_rs::LettaId::new("agent-123");
//!     let mut memory_stream = client
//!         .memory()
//!         .archival_paginated(&agent_id)
//!         .text("important")
//!         .limit(20)
//!         .build();
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

// Re-export main types for convenience
pub use client::{ClientBuilder, ClientConfig, LettaClient};
pub use environment::LettaEnvironment;
pub use error::{LettaError, LettaResult};
pub use types::*;

/// Maximum number of retries for API calls
pub const MAX_RETRIES: u32 = 3;

/// Convenience type alias for Results in this crate.
pub type Result<T> = std::result::Result<T, LettaError>;

// Re-export streaming types
pub use api::messages::{MessageStream, StreamingEvent};
