//! Agent API endpoints.

use crate::client::LettaClient;
use crate::error::LettaResult;
use crate::types::{
    Agent, AgentsSearchRequest, AgentsSearchResponse, CreateAgentRequest, ImportAgentRequest,
    ListAgentsParams,
};
use reqwest::header::HeaderMap;
use reqwest::multipart::{Form, Part};
use std::path::Path;

/// Agent API operations.
#[derive(Debug)]
pub struct AgentApi<'a> {
    client: &'a LettaClient,
}

impl<'a> AgentApi<'a> {
    /// Create a new agent API instance.
    pub fn new(client: &'a LettaClient) -> Self {
        Self { client }
    }

    /// List all agents.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional query parameters for filtering and pagination
    ///
    /// # Errors
    ///
    /// Returns a [`LettaError`] if the request fails or if the response cannot be parsed.
    pub async fn list(&self, params: Option<ListAgentsParams>) -> LettaResult<Vec<Agent>> {
        let url = self.client.base_url().join("v1/agents")?;

        let mut headers = HeaderMap::new();
        self.client.auth().apply_to_headers(&mut headers)?;
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let mut request = self.client.http().get(url).headers(headers);

        if let Some(params) = params {
            request = request.query(&params);
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await?;
            return Err(crate::error::LettaError::from_response(status, body));
        }

        let agents: Vec<Agent> = response.json().await?;
        Ok(agents)
    }

    /// Create a new agent.
    ///
    /// # Arguments
    ///
    /// * `request` - The agent creation request
    ///
    /// # Errors
    ///
    /// Returns a [`LettaError`] if the request fails or if the response cannot be parsed.
    pub async fn create(&self, request: CreateAgentRequest) -> LettaResult<Agent> {
        let url = self.client.base_url().join("v1/agents")?;

        let mut headers = HeaderMap::new();
        self.client.auth().apply_to_headers(&mut headers)?;
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let response = self
            .client
            .http()
            .post(url)
            .headers(headers)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await?;
            return Err(crate::error::LettaError::from_response(status, body));
        }

        let agent: Agent = response.json().await?;
        Ok(agent)
    }

    /// Get a specific agent by ID.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The ID of the agent to retrieve
    ///
    /// # Errors
    ///
    /// Returns a [`LettaError`] if the request fails or if the response cannot be parsed.
    pub async fn get(&self, agent_id: &str) -> LettaResult<Agent> {
        let url = self
            .client
            .base_url()
            .join(&format!("v1/agents/{}", agent_id))?;

        let mut headers = HeaderMap::new();
        self.client.auth().apply_to_headers(&mut headers)?;
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let response = self.client.http().get(url).headers(headers).send().await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await?;
            return Err(crate::error::LettaError::from_response(status, body));
        }

        let agent: Agent = response.json().await?;
        Ok(agent)
    }

    /// Delete an agent by ID.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The ID of the agent to delete
    ///
    /// # Errors
    ///
    /// Returns a [`LettaError`] if the request fails.
    pub async fn delete(&self, agent_id: &str) -> LettaResult<()> {
        let url = self
            .client
            .base_url()
            .join(&format!("v1/agents/{}", agent_id))?;

        let mut headers = HeaderMap::new();
        self.client.auth().apply_to_headers(&mut headers)?;

        let response = self
            .client
            .http()
            .delete(url)
            .headers(headers)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await?;
            return Err(crate::error::LettaError::from_response(status, body));
        }

        Ok(())
    }

    /// Summarize an agent's conversation history to a target message length.
    ///
    /// This endpoint summarizes the current message history for a given agent,
    /// truncating and compressing it down to the specified `max_message_length`.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The ID of the agent whose conversation to summarize
    /// * `max_message_length` - Maximum number of messages to retain after summarization
    ///
    /// # Errors
    ///
    /// Returns a [`LettaError`] if the request fails or if the response cannot be parsed.
    pub async fn summarize_agent_conversation(
        &self,
        agent_id: &str,
        max_message_length: u32,
    ) -> LettaResult<Agent> {
        let url = self
            .client
            .base_url()
            .join(&format!("v1/agents/{}/summarize", agent_id))?;

        let mut headers = HeaderMap::new();
        self.client.auth().apply_to_headers(&mut headers)?;
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let response = self
            .client
            .http()
            .post(url)
            .headers(headers)
            .query(&[("max_message_length", max_message_length)])
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await?;
            return Err(crate::error::LettaError::from_response(status, body));
        }

        let agent: Agent = response.json().await?;
        Ok(agent)
    }

    /// Get the count of all agents associated with a given user.
    ///
    /// # Errors
    ///
    /// Returns a [`LettaError`] if the request fails or if the response cannot be parsed.
    pub async fn count(&self) -> LettaResult<u32> {
        let url = self.client.base_url().join("v1/agents/count")?;

        let mut headers = HeaderMap::new();
        self.client.auth().apply_to_headers(&mut headers)?;

        let response = self.client.http().get(url).headers(headers).send().await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await?;
            return Err(crate::error::LettaError::from_response(status, body));
        }

        let count: u32 = response.json().await?;
        Ok(count)
    }

    /// Export the serialized JSON representation of an agent, formatted with indentation.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The ID of the agent to export
    ///
    /// # Errors
    ///
    /// Returns a [`LettaError`] if the request fails or if the response cannot be parsed.
    pub async fn export_file(&self, agent_id: &str) -> LettaResult<String> {
        let url = self
            .client
            .base_url()
            .join(&format!("v1/agents/{}/export", agent_id))?;

        let mut headers = HeaderMap::new();
        self.client.auth().apply_to_headers(&mut headers)?;

        let response = self.client.http().get(url).headers(headers).send().await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await?;
            return Err(crate::error::LettaError::from_response(status, body));
        }

        let export_data = response.text().await?;
        Ok(export_data)
    }

    /// Import an agent from a file.
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the agent file to import
    /// * `request` - Import parameters
    ///
    /// # Errors
    ///
    /// Returns a [`LettaError`] if the request fails or if the response cannot be parsed.
    pub async fn import_file(
        &self,
        file_path: &Path,
        request: ImportAgentRequest,
    ) -> LettaResult<Agent> {
        let url = self.client.base_url().join("v1/agents/import")?;

        // Read the file
        let file_content = tokio::fs::read(file_path).await?;
        let file_name = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("agent.json");

        // Create multipart form
        let file_part = Part::bytes(file_content)
            .file_name(file_name.to_string())
            .mime_str("application/json")?;

        let form = Form::new().part("file", file_part);

        // Add query parameters to the form
        let mut params = Vec::new();

        if let Some(append_copy_suffix) = request.append_copy_suffix {
            params.push(("append_copy_suffix", append_copy_suffix.to_string()));
        }
        if let Some(override_existing_tools) = request.override_existing_tools {
            params.push((
                "override_existing_tools",
                override_existing_tools.to_string(),
            ));
        }
        if let Some(project_id) = request.project_id {
            params.push(("project_id", project_id));
        }
        if let Some(strip_messages) = request.strip_messages {
            params.push(("strip_messages", strip_messages.to_string()));
        }

        let mut headers = HeaderMap::new();
        self.client.auth().apply_to_headers(&mut headers)?;
        // Don't set Content-Type header - reqwest will set it automatically for multipart

        let mut request_builder = self
            .client
            .http()
            .post(url)
            .headers(headers)
            .multipart(form);

        if !params.is_empty() {
            request_builder = request_builder.query(&params);
        }

        let response = request_builder.send().await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await?;
            return Err(crate::error::LettaError::from_response(status, body));
        }

        let agent: Agent = response.json().await?;
        Ok(agent)
    }

    /// Search for agents using various criteria.
    ///
    /// # Arguments
    ///
    /// * `request` - Search parameters and filters
    ///
    /// # Errors
    ///
    /// Returns a [`LettaError`] if the request fails or if the response cannot be parsed.
    pub async fn search(&self, request: AgentsSearchRequest) -> LettaResult<AgentsSearchResponse> {
        let url = self.client.base_url().join("v1/agents/search")?;

        let mut headers = HeaderMap::new();
        self.client.auth().apply_to_headers(&mut headers)?;
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let response = self
            .client
            .http()
            .post(url)
            .headers(headers)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await?;
            return Err(crate::error::LettaError::from_response(status, body));
        }

        let search_response: AgentsSearchResponse = response.json().await?;
        Ok(search_response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientConfig;

    #[test]
    fn test_agent_api_creation() {
        let config = ClientConfig::new("http://localhost:8283").unwrap();
        let client = LettaClient::new(config).unwrap();
        let _api = AgentApi::new(&client);
    }
}
