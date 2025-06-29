//! Message API endpoints.

use crate::client::LettaClient;
use crate::error::LettaResult;
use crate::types::{CreateMessagesRequest, LettaMessageUnion, LettaResponse, ListMessagesRequest};
use reqwest::header::HeaderMap;

/// Message API operations.
#[derive(Debug)]
pub struct MessageApi<'a> {
    client: &'a LettaClient,
}

impl<'a> MessageApi<'a> {
    /// Create a new message API instance.
    pub fn new(client: &'a LettaClient) -> Self {
        Self { client }
    }

    /// List messages for an agent.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The ID of the agent whose messages to list
    /// * `params` - Optional query parameters for filtering and pagination
    ///
    /// # Errors
    ///
    /// Returns a [`LettaError`] if the request fails or if the response cannot be parsed.
    pub async fn list(
        &self,
        agent_id: &str,
        params: Option<ListMessagesRequest>,
    ) -> LettaResult<Vec<LettaMessageUnion>> {
        let url = self
            .client
            .base_url()
            .join(&format!("v1/agents/{}/messages", agent_id))?;

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

        let messages: Vec<LettaMessageUnion> = response.json().await?;
        Ok(messages)
    }

    /// Send messages to an agent and get response.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The ID of the agent to send messages to
    /// * `request` - The message creation request with messages and options
    ///
    /// # Errors
    ///
    /// Returns a [`LettaError`] if the request fails or if the response cannot be parsed.
    pub async fn create(
        &self,
        agent_id: &str,
        request: CreateMessagesRequest,
    ) -> LettaResult<LettaResponse> {
        let url = self
            .client
            .base_url()
            .join(&format!("v1/agents/{}/messages", agent_id))?;

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

        let letta_response: LettaResponse = response.json().await?;
        Ok(letta_response)
    }

    /// Reset an agent's message history.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The ID of the agent whose messages to reset
    /// * `add_default_initial_messages` - Whether to add default initial messages
    ///
    /// # Errors
    ///
    /// Returns a [`LettaError`] if the request fails or if the response cannot be parsed.
    pub async fn reset(
        &self,
        agent_id: &str,
        add_default_initial_messages: Option<bool>,
    ) -> LettaResult<crate::types::Agent> {
        let url = self
            .client
            .base_url()
            .join(&format!("v1/agents/{}/reset-messages", agent_id))?;

        let mut headers = HeaderMap::new();
        self.client.auth().apply_to_headers(&mut headers)?;
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let mut body = serde_json::Map::new();
        if let Some(add_default) = add_default_initial_messages {
            body.insert(
                "add_default_initial_messages".to_string(),
                serde_json::Value::Bool(add_default),
            );
        }

        let response = self
            .client
            .http()
            .patch(url)
            .headers(headers)
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await?;
            return Err(crate::error::LettaError::from_response(status, body));
        }

        let agent: crate::types::Agent = response.json().await?;
        Ok(agent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientConfig;

    #[test]
    fn test_message_api_creation() {
        let config = ClientConfig::new("http://localhost:8283").unwrap();
        let client = LettaClient::new(config).unwrap();
        let _api = MessageApi::new(&client);
    }
}
