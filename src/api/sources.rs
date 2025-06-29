//! Source API endpoints.

use crate::client::LettaClient;
use crate::error::LettaResult;
use crate::types::Source;

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
        todo!("Implement source list")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientConfig;

    #[test]
    fn test_source_api_creation() {
        let config = ClientConfig::new("http://localhost:8283").unwrap();
        let client = LettaClient::new(config).unwrap();
        let _api = SourceApi::new(&client);
    }
}