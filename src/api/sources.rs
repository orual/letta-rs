//! Source API endpoints.

use crate::client::LettaClient;
use crate::error::LettaResult;
use crate::types::agent::Agent;
use crate::types::memory::Passage;
use crate::types::source::{
    CreateSourceRequest, FileMetadata, FileUploadResponse, GetFileParams, ListFilesParams,
    ListPassagesParams, Source, UpdateSourceRequest,
};
use crate::types::LettaId;
use bytes::Bytes;
use reqwest::multipart::{Form, Part};
use serde_json::Value;

/// Source API operations.
#[derive(Debug)]
pub struct SourceApi<'a> {
    client: &'a LettaClient,
}

impl<'a> SourceApi<'a> {
    /// Create a new source API instance.
    pub fn new(client: &'a LettaClient) -> Self {
        Self { client }
    }

    /// List all sources.
    pub async fn list(&self) -> LettaResult<Vec<Source>> {
        self.client.get("v1/sources/").await
    }

    /// Create a new source.
    pub async fn create(&self, request: CreateSourceRequest) -> LettaResult<Source> {
        self.client.post("v1/sources/", &request).await
    }

    /// Get a source by ID.
    pub async fn get(&self, source_id: &LettaId) -> LettaResult<Source> {
        self.client.get(&format!("v1/sources/{}", source_id)).await
    }

    /// Update a source.
    pub async fn update(
        &self,
        source_id: &LettaId,
        request: UpdateSourceRequest,
    ) -> LettaResult<Source> {
        self.client
            .patch(&format!("v1/sources/{}", source_id), &request)
            .await
    }

    /// Delete a source.
    pub async fn delete(&self, source_id: &LettaId) -> LettaResult<Option<Value>> {
        self.client
            .delete(&format!("v1/sources/{}", source_id))
            .await
    }

    /// Get source count.
    pub async fn count(&self) -> LettaResult<i32> {
        self.client.get("v1/sources/count").await
    }

    /// Get source ID by name.
    pub async fn get_by_name(&self, source_name: &str) -> LettaResult<String> {
        self.client
            .get(&format!("v1/sources/name/{}", source_name))
            .await
    }

    /// Upload a file to a source.
    /// Returns either a job response (local) or file metadata (cloud).
    pub async fn upload_file(
        &self,
        source_id: &LettaId,
        file_name: String,
        file_data: Bytes,
        content_type: Option<String>,
    ) -> LettaResult<FileUploadResponse> {
        let form = Form::new();
        let mut part = Part::bytes(file_data.to_vec()).file_name(file_name);

        if let Some(ct) = content_type {
            part = part.mime_str(&ct)?;
        }

        let form = form.part("file", part);

        self.client
            .post_multipart(&format!("v1/sources/{}/upload", source_id), form)
            .await
    }

    /// List files in a source.
    pub async fn list_files(
        &self,
        source_id: &LettaId,
        params: Option<ListFilesParams>,
    ) -> LettaResult<Vec<FileMetadata>> {
        self.client
            .get_with_query(&format!("v1/sources/{}/files", source_id), &params)
            .await
    }

    /// Get file metadata.
    pub async fn get_file(
        &self,
        source_id: &LettaId,
        file_id: &LettaId,
        params: Option<GetFileParams>,
    ) -> LettaResult<FileMetadata> {
        self.client
            .get_with_query(
                &format!("v1/sources/{}/files/{}", source_id, file_id),
                &params,
            )
            .await
    }

    /// Delete a file from a source.
    pub async fn delete_file(&self, source_id: &LettaId, file_id: &LettaId) -> LettaResult<()> {
        self.client
            .delete_no_response(&format!("v1/sources/{}/{}", source_id, file_id))
            .await
    }

    /// List passages from a source.
    pub async fn list_passages(
        &self,
        source_id: &LettaId,
        params: Option<ListPassagesParams>,
    ) -> LettaResult<Vec<Passage>> {
        self.client
            .get_with_query(&format!("v1/sources/{}/passages", source_id), &params)
            .await
    }

    /// Get agent source sub-API for a specific agent.
    pub fn agent_sources(&self, agent_id: LettaId) -> AgentSourceApi {
        AgentSourceApi::new(self.client, agent_id)
    }
}

/// Agent source sub-API operations.
#[derive(Debug)]
pub struct AgentSourceApi<'a> {
    client: &'a LettaClient,
    agent_id: LettaId,
}

impl<'a> AgentSourceApi<'a> {
    /// Create a new agent source API instance.
    pub fn new(client: &'a LettaClient, agent_id: LettaId) -> Self {
        Self { client, agent_id }
    }

    /// List sources attached to the agent.
    pub async fn list(&self) -> LettaResult<Vec<Source>> {
        self.client
            .get(&format!("v1/agents/{}/sources", self.agent_id))
            .await
    }

    /// Attach a source to the agent.
    pub async fn attach(&self, source_id: &LettaId) -> LettaResult<Agent> {
        self.client
            .patch(
                &format!("v1/agents/{}/sources/attach/{}", self.agent_id, source_id),
                &(),
            )
            .await
    }

    /// Detach a source from the agent.
    pub async fn detach(&self, source_id: &LettaId) -> LettaResult<Agent> {
        self.client
            .patch(
                &format!("v1/agents/{}/sources/detach/{}", self.agent_id, source_id),
                &(),
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientConfig;
    use std::str::FromStr;

    #[test]
    fn test_source_api_creation() {
        let config = ClientConfig::new("http://localhost:8283").unwrap();
        let client = LettaClient::new(config).unwrap();
        let _api = SourceApi::new(&client);
    }

    #[test]
    fn test_agent_source_api_creation() {
        let config = ClientConfig::new("http://localhost:8283").unwrap();
        let client = LettaClient::new(config).unwrap();
        let _api = AgentSourceApi::new(
            &client,
            LettaId::from_str("agent-550e8400-e29b-41d4-a716-446655440000").unwrap(),
        );
    }
}
