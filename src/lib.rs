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
//! ## Features
//!
//! - **Complete API Coverage**: All Letta REST API endpoints
//! - **Async/Await**: Full async support with tokio
//! - **Streaming**: Server-sent events for real-time messaging
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
