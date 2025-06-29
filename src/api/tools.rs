//! Tool API endpoints.

use crate::client::LettaClient;
use crate::error::LettaResult;
use crate::types::Tool;

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
    pub async fn list(&self) -> LettaResult<Vec<Tool>> {
        todo!("Implement tool list")
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
