//! Memory API endpoints.

use crate::client::LettaClient;
use crate::error::LettaResult;
use crate::types::memory::MemoryBlock;

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

    /// Get core memory blocks for an agent.
    pub async fn get_core_memory(&self, _agent_id: &str) -> LettaResult<Vec<MemoryBlock>> {
        todo!("Implement core memory retrieval")
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