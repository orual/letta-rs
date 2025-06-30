//! Memory API endpoints.

use crate::client::LettaClient;
use crate::error::LettaResult;
use crate::types::{
    memory::{
        ArchivalMemoryQueryParams, Block, CreateArchivalMemoryRequest, Memory, Passage,
        UpdateArchivalMemoryRequest, UpdateMemoryBlockRequest,
    },
    LettaId,
};

/// Memory API operations.
#[derive(Debug)]
pub struct MemoryApi<'a> {
    client: &'a LettaClient,
}

impl<'a> MemoryApi<'a> {
    /// Create a new memory API instance.
    pub fn new(client: &'a LettaClient) -> Self {
        Self { client }
    }

    /// Get the core memory of an agent.
    pub async fn get_core_memory(&self, agent_id: &LettaId) -> LettaResult<Memory> {
        let url = format!("/v1/agents/{}/core-memory", agent_id);
        self.client.get(&url).await
    }

    /// Get all core memory blocks for an agent.
    pub async fn list_core_memory_blocks(&self, agent_id: &LettaId) -> LettaResult<Vec<Block>> {
        let url = format!("/v1/agents/{}/core-memory/blocks", agent_id);
        self.client.get(&url).await
    }

    /// Get a specific core memory block by label.
    pub async fn get_core_memory_block(
        &self,
        agent_id: &LettaId,
        block_label: &str,
    ) -> LettaResult<Block> {
        let url = format!("/v1/agents/{}/core-memory/blocks/{}", agent_id, block_label);
        self.client.get(&url).await
    }

    /// Update a core memory block.
    pub async fn update_core_memory_block(
        &self,
        agent_id: &LettaId,
        block_label: &str,
        request: UpdateMemoryBlockRequest,
    ) -> LettaResult<Block> {
        let url = format!("/v1/agents/{}/core-memory/blocks/{}", agent_id, block_label);
        self.client.patch(&url, &request).await
    }

    /// Attach a memory block to an agent.
    pub async fn attach_memory_block(
        &self,
        agent_id: &LettaId,
        block_id: &LettaId,
    ) -> LettaResult<crate::types::agent::Agent> {
        let url = format!(
            "/v1/agents/{}/core-memory/blocks/attach/{}",
            agent_id, block_id
        );
        self.client.patch_no_body(&url).await
    }

    /// Detach a memory block from an agent.
    pub async fn detach_memory_block(
        &self,
        agent_id: &LettaId,
        block_id: &LettaId,
    ) -> LettaResult<crate::types::agent::Agent> {
        let url = format!(
            "/v1/agents/{}/core-memory/blocks/detach/{}",
            agent_id, block_id
        );
        self.client.patch_no_body(&url).await
    }

    // Archival Memory API

    /// List archival memory passages for an agent.
    pub async fn list_archival_memory(
        &self,
        agent_id: &LettaId,
        params: Option<ArchivalMemoryQueryParams>,
    ) -> LettaResult<Vec<Passage>> {
        let url = format!("/v1/agents/{}/archival-memory", agent_id);
        if let Some(params) = params {
            let query = serde_urlencoded::to_string(&params)?;
            if !query.is_empty() {
                let url_with_query = format!("{}?{}", url, query);
                return self.client.get(&url_with_query).await;
            }
        }
        self.client.get(&url).await
    }

    /// Create a new archival memory passage.
    pub async fn create_archival_memory(
        &self,
        agent_id: &LettaId,
        request: CreateArchivalMemoryRequest,
    ) -> LettaResult<Vec<Passage>> {
        let url = format!("/v1/agents/{}/archival-memory", agent_id);
        self.client.post(&url, &request).await
    }

    /// Update an archival memory passage.
    pub async fn update_archival_memory(
        &self,
        agent_id: &LettaId,
        memory_id: &LettaId,
        request: UpdateArchivalMemoryRequest,
    ) -> LettaResult<Vec<Passage>> {
        let url = format!("/v1/agents/{}/archival-memory/{}", agent_id, memory_id);
        self.client.patch(&url, &request).await
    }

    /// Delete an archival memory passage.
    pub async fn delete_archival_memory(
        &self,
        agent_id: &LettaId,
        memory_id: &LettaId,
    ) -> LettaResult<serde_json::Value> {
        let url = format!("/v1/agents/{}/archival-memory/{}", agent_id, memory_id);
        self.client.delete(&url).await
    }

    // Agent Tools API

    /// List tools attached to an agent.
    pub async fn list_agent_tools(
        &self,
        agent_id: &LettaId,
    ) -> LettaResult<Vec<crate::types::tool::Tool>> {
        let url = format!("/v1/agents/{}/tools", agent_id);
        self.client.get(&url).await
    }

    /// Attach a tool to an agent.
    pub async fn attach_tool_to_agent(
        &self,
        agent_id: &LettaId,
        tool_id: &LettaId,
    ) -> LettaResult<crate::types::agent::Agent> {
        let url = format!("/v1/agents/{}/tools/attach/{}", agent_id, tool_id);
        self.client.patch_no_body(&url).await
    }

    /// Detach a tool from an agent.
    pub async fn detach_tool_from_agent(
        &self,
        agent_id: &LettaId,
        tool_id: &LettaId,
    ) -> LettaResult<crate::types::agent::Agent> {
        let url = format!("/v1/agents/{}/tools/detach/{}", agent_id, tool_id);
        self.client.patch_no_body(&url).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientConfig;

    #[test]
    fn test_memory_api_creation() {
        let config = ClientConfig::new("http://localhost:8283").unwrap();
        let client = LettaClient::new(config).unwrap();
        let _api = MemoryApi::new(&client);
    }
}
