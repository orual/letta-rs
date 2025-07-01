//! API endpoint implementations.
//!
//! This module contains the implementation of all Letta API endpoints,
//! organized by functional area.

pub mod agents;
pub mod batch;
pub mod blocks;
pub mod groups;
pub mod health;
pub mod identities;
pub mod jobs;
pub mod memory;
pub mod messages;
pub mod misc;
pub mod models;
pub mod projects;
pub mod providers;
pub mod runs;
pub mod sources;
pub mod tags;
pub mod telemetry;
pub mod templates;
pub mod tools;
pub mod voice;

// Re-export API structs
pub use agents::AgentApi;
pub use batch::BatchApi;
pub use blocks::BlocksApi;
pub use groups::GroupApi;
pub use health::HealthApi;
pub use identities::IdentitiesApi;
pub use jobs::{JobApi, StepApi};
pub use memory::MemoryApi;
pub use messages::MessageApi;
pub use models::ModelsApi;
pub use projects::ProjectApi;
pub use providers::ProvidersApi;
pub use runs::RunApi;
pub use sources::{AgentSourceApi, SourceApi};
pub use tags::TagsApi;
pub use telemetry::TelemetryApi;
pub use templates::TemplateApi;
pub use tools::ToolApi;
pub use voice::VoiceApi;
