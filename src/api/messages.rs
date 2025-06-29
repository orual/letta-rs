//! Message API endpoints.

use crate::client::LettaClient;
use crate::error::LettaResult;
use crate::types::Message;

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

    /// Send a message to an agent.
    pub async fn send(&self, _agent_id: &str, _content: &str) -> LettaResult<Message> {
        todo!("Implement message send")
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