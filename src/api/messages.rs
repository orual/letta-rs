//! Message API endpoints.

use crate::client::LettaClient;
use crate::error::{LettaError, LettaResult};
use crate::types::{
    CreateMessagesRequest, LettaId, LettaMessageUnion, LettaResponse, LettaStopReason,
    LettaUsageStatistics, ListMessagesRequest,
};
use eventsource_stream::Eventsource;
use futures::stream::{Stream, StreamExt};
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use std::pin::Pin;

/// Streaming event types from the message stream.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StreamingEvent {
    /// A message from the agent.
    Message(LettaMessageUnion),
    /// Stop reason indicating why processing stopped.
    StopReason(LettaStopReason),
    /// Usage statistics for the conversation.
    Usage(LettaUsageStatistics),
}

/// Streaming response type containing the event stream.
pub type MessageStream = Pin<Box<dyn Stream<Item = LettaResult<StreamingEvent>> + Send>>;

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
        agent_id: &LettaId,
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
        agent_id: &LettaId,
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
        agent_id: &LettaId,
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

    /// Send messages to an agent and stream the response.
    ///
    /// This method uses Server-Sent Events (SSE) to stream the response, allowing
    /// for real-time updates as the agent processes the messages.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The ID of the agent to send messages to
    /// * `request` - The message creation request with messages and options
    /// * `stream_tokens` - Whether to stream individual tokens (true) or complete messages (false)
    ///
    /// # Returns
    ///
    /// A stream of [`StreamingEvent`] items that can be consumed asynchronously.
    ///
    /// # Errors
    ///
    /// Returns a [`LettaError`] if the request fails or if the response cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use letta_rs::types::{MessageCreate, MessageRole, CreateMessagesRequest, MessageCreateContent};
    /// # use letta_rs::api::messages::StreamingEvent;
    /// # use letta_rs::LettaId;
    /// # use std::str::FromStr;
    /// # use futures::StreamExt;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = letta_rs::LettaClient::new(
    /// #     letta_rs::client::ClientConfig::new("http://localhost:8283")?
    /// # )?;
    /// let mut stream = client
    ///     .messages()
    ///     .create_stream(
    ///         &LettaId::from_str("agent-00000000-0000-0000-0000-000000000000").unwrap(),
    ///         CreateMessagesRequest {
    ///             messages: vec![MessageCreate {
    ///                 role: MessageRole::User,
    ///                 content: "Hello!".into(),
    ///                 ..Default::default()
    ///             }],
    ///             ..Default::default()
    ///         },
    ///         false,
    ///     )
    ///     .await?;
    ///
    /// while let Some(event) = stream.next().await {
    ///     match event? {
    ///         StreamingEvent::Message(msg) => println!("Message: {:?}", msg),
    ///         StreamingEvent::StopReason(reason) => println!("Stop: {:?}", reason),
    ///         StreamingEvent::Usage(usage) => println!("Usage: {:?}", usage),
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_stream(
        &self,
        agent_id: &LettaId,
        request: CreateMessagesRequest,
        stream_tokens: bool,
    ) -> LettaResult<MessageStream> {
        // Build the URL with streaming endpoint
        let url = self
            .client
            .base_url()
            .join(&format!("v1/agents/{}/messages/stream", agent_id))?;

        // Add query parameter for token streaming
        let url = if stream_tokens {
            url::Url::parse_with_params(url.as_str(), &[("stream_tokens", "true")])?
        } else {
            url
        };

        // Build headers
        let mut headers = HeaderMap::new();
        self.client.auth().apply_to_headers(&mut headers)?;
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("Accept", "text/event-stream".parse().unwrap());

        // Send the request
        let response = self
            .client
            .http()
            .post(url)
            .headers(headers)
            .json(&request)
            .send()
            .await?;

        // Check for HTTP errors
        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await?;
            return Err(LettaError::from_response(status, body));
        }

        // Create the event stream
        let stream = response
            .bytes_stream()
            .eventsource()
            .filter_map(|result| async move {
                match result {
                    Ok(event) => {
                        // Skip events without data
                        if event.data.is_empty() || event.data == "[DONE]" {
                            return None;
                        }

                        // Parse the event data
                        match serde_json::from_str::<StreamingEvent>(&event.data) {
                            Ok(parsed) => Some(Ok(parsed)),
                            Err(e) => {
                                // Skip parsing errors (like the Python SDK)
                                eprintln!("Failed to parse SSE event: {}", e);
                                None
                            }
                        }
                    }
                    Err(e) => Some(Err(LettaError::streaming(format!(
                        "SSE stream error: {}",
                        e
                    )))),
                }
            });

        Ok(Box::pin(stream))
    }

    /// Update a message.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The ID of the agent that owns the message
    /// * `message_id` - The ID of the message to update
    /// * `request` - The update request with the new message content
    ///
    /// # Returns
    ///
    /// The updated message as a [`LettaMessageUnion`].
    ///
    /// # Errors
    ///
    /// Returns a [`LettaError`] if the request fails or if the response cannot be parsed.
    pub async fn update(
        &self,
        agent_id: &LettaId,
        message_id: &LettaId,
        request: crate::types::UpdateMessageRequest,
    ) -> LettaResult<LettaMessageUnion> {
        let url = self
            .client
            .base_url()
            .join(&format!("v1/agents/{}/messages/{}", agent_id, message_id))?;

        let mut headers = HeaderMap::new();
        self.client.auth().apply_to_headers(&mut headers)?;
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let response = self
            .client
            .http()
            .patch(url)
            .headers(headers)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await?;
            return Err(crate::error::LettaError::from_response(status, body));
        }

        let message: LettaMessageUnion = response.json().await?;
        Ok(message)
    }

    /// Create messages asynchronously and return a run object.
    ///
    /// This method submits messages for processing in the background and returns
    /// immediately with a run ID that can be used to check the status.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The ID of the agent to send messages to
    /// * `request` - The message creation request with messages and options
    ///
    /// # Returns
    ///
    /// A [`Run`] object containing the run ID and status information.
    ///
    /// # Errors
    ///
    /// Returns a [`LettaError`] if the request fails or if the response cannot be parsed.
    pub async fn create_async(
        &self,
        agent_id: &LettaId,
        request: CreateMessagesRequest,
    ) -> LettaResult<crate::types::Run> {
        let url = self
            .client
            .base_url()
            .join(&format!("v1/agents/{}/messages/async", agent_id))?;

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

        let run: crate::types::Run = response.json().await?;
        Ok(run)
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
