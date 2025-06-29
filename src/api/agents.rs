//! Agent API endpoints.

use crate::client::LettaClient;
use crate::error::LettaResult;
use crate::types::Agent;

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
    pub async fn list(&self) -> LettaResult<Vec<Agent>> {
        todo!("Implement agent list")
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