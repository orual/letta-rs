//! Tool API endpoints.

use crate::client::LettaClient;
use crate::error::LettaResult;
use crate::types::tool::{CreateToolRequest, ListToolsParams, Tool, UpdateToolRequest};
use crate::types::LettaId;

/// Tool API operations.
#[derive(Debug)]
pub struct ToolApi<'a> {
    client: &'a LettaClient,
}

impl<'a> ToolApi<'a> {
    /// Create a new tool API instance.
    pub fn new(client: &'a LettaClient) -> Self {
        Self { client }
    }

    /// List all tools.
    pub async fn list(&self, params: Option<ListToolsParams>) -> LettaResult<Vec<Tool>> {
        let url = "/v1/tools/";
        if let Some(params) = params {
            let query = serde_urlencoded::to_string(&params)?;
            if !query.is_empty() {
                let url_with_query = format!("{}?{}", url, query);
                return self.client.get(&url_with_query).await;
            }
        }
        self.client.get(url).await
    }

    /// Create a new tool.
    pub async fn create(&self, request: CreateToolRequest) -> LettaResult<Tool> {
        self.client.post("/v1/tools/", &request).await
    }

    /// Get a specific tool by ID.
    pub async fn get(&self, tool_id: &LettaId) -> LettaResult<Tool> {
        let url = format!("/v1/tools/{}", tool_id);
        self.client.get(&url).await
    }

    /// Update a tool.
    pub async fn update(&self, tool_id: &LettaId, request: UpdateToolRequest) -> LettaResult<Tool> {
        let url = format!("/v1/tools/{}", tool_id);
        self.client.patch(&url, &request).await
    }

    /// Delete a tool.
    pub async fn delete(&self, tool_id: &LettaId) -> LettaResult<()> {
        let url = format!("/v1/tools/{}", tool_id);
        self.client.delete(&url).await
    }

    /// Get count of tools.
    pub async fn count(&self) -> LettaResult<u32> {
        let response: serde_json::Value = self.client.get("/v1/tools/count").await?;
        // The response is just a bare number
        response.as_u64().map(|v| v as u32).ok_or_else(|| {
            crate::error::LettaError::validation("Invalid count response - expected number")
        })
    }

    /// Upsert a tool (create or update).
    pub async fn upsert(&self, request: CreateToolRequest) -> LettaResult<Tool> {
        self.client.put("/v1/tools/", &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientConfig;

    #[test]
    fn test_tool_api_creation() {
        let config = ClientConfig::new("http://localhost:8283").unwrap();
        let client = LettaClient::new(config).unwrap();
        let _api = ToolApi::new(&client);
    }
}
