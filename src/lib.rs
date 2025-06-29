//! # Letta Rust Client
//!
//! A robust, fully-featured Rust client for the [Letta REST API](https://docs.letta.com/api-reference/overview).
//! 
//! Letta is a platform for building stateful AI agents with persistent memory and context
//! across conversations. This client provides a comprehensive, idiomatic Rust interface
//! to all Letta API endpoints.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use letta_rs::{LettaClient, ClientConfig};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Connect to local Letta server
//!     let client = LettaClient::new(
//!         ClientConfig::new("http://localhost:8283")?
//!     )?;
//!     
//!     // List all agents (note: not yet implemented)
//!     // let agents = client.agents().list().await?;
//!     // println!("Found {} agents", agents.len());
//!     
//!     println!("Client created successfully!");
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
#![allow(clippy::module_name_repetitions)]

pub mod api;
pub mod auth;
pub mod client;
pub mod error;
pub mod streaming;
pub mod types;
pub mod utils;

// Re-export main types for convenience
pub use client::{LettaClient, ClientConfig, ClientBuilder};
pub use error::{LettaError, LettaResult};
pub use types::*;