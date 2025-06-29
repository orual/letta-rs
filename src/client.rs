//! HTTP client and configuration for the Letta API.

use crate::auth::AuthConfig;
use crate::error::{LettaError, LettaResult};
use reqwest::header::HeaderMap;
use std::time::Duration;
use url::Url;

/// Configuration for the Letta client.
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Base URL for the Letta API.
    pub base_url: Url,
    /// Authentication configuration.
    pub auth: AuthConfig,
    /// Request timeout.
    pub timeout: Duration,
    /// Additional headers to include with requests.
    pub headers: HeaderMap,
}

impl ClientConfig {
    /// Create a new client configuration.
    pub fn new(base_url: impl AsRef<str>) -> LettaResult<Self> {
        let base_url = Url::parse(base_url.as_ref())?;
        Ok(Self {
            base_url,
            auth: AuthConfig::default(),
            timeout: Duration::from_secs(30),
            headers: HeaderMap::new(),
        })
    }

    /// Set the authentication configuration.
    pub fn auth(mut self, auth: AuthConfig) -> Self {
        self.auth = auth;
        self
    }

    /// Set the request timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

/// Main Letta API client.
#[derive(Debug, Clone)]
pub struct LettaClient {
    http: reqwest::Client,
    config: ClientConfig,
}

impl LettaClient {
    /// Create a new Letta client.
    pub fn new(config: ClientConfig) -> LettaResult<Self> {
        let http = reqwest::Client::builder()
            .timeout(config.timeout)
            .default_headers(config.headers.clone())
            .build()?;

        Ok(Self { http, config })
    }

    /// Get the base URL.
    pub fn base_url(&self) -> &Url {
        &self.config.base_url
    }

    /// Get the HTTP client.
    pub fn http(&self) -> &reqwest::Client {
        &self.http
    }

    /// Get the authentication configuration.
    pub fn auth(&self) -> &AuthConfig {
        &self.config.auth
    }

    /// Get the agent API.
    pub fn agents(&self) -> crate::api::AgentApi<'_> {
        crate::api::AgentApi::new(self)
    }

    /// Get the message API.
    pub fn messages(&self) -> crate::api::MessageApi<'_> {
        crate::api::MessageApi::new(self)
    }

    /// Get the memory API.
    pub fn memory(&self) -> crate::api::MemoryApi<'_> {
        crate::api::MemoryApi::new(self)
    }

    /// Get the source API.
    pub fn sources(&self) -> crate::api::SourceApi<'_> {
        crate::api::SourceApi::new(self)
    }

    /// Get the tool API.
    pub fn tools(&self) -> crate::api::ToolApi<'_> {
        crate::api::ToolApi::new(self)
    }
}

/// Builder for creating a Letta client.
#[derive(Debug, Default)]
pub struct ClientBuilder {
    base_url: Option<String>,
    auth: Option<AuthConfig>,
    timeout: Option<Duration>,
    headers: Option<HeaderMap>,
}

impl ClientBuilder {
    /// Create a new client builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the base URL.
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Set the authentication.
    pub fn auth(mut self, auth: AuthConfig) -> Self {
        self.auth = Some(auth);
        self
    }

    /// Set the timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Build the client.
    pub fn build(self) -> LettaResult<LettaClient> {
        let base_url = self
            .base_url
            .ok_or_else(|| LettaError::config("Base URL is required"))?;

        let mut config = ClientConfig::new(base_url)?;

        if let Some(auth) = self.auth {
            config = config.auth(auth);
        }

        if let Some(timeout) = self.timeout {
            config = config.timeout(timeout);
        }

        LettaClient::new(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_config() {
        let config = ClientConfig::new("http://localhost:8283").unwrap();
        assert_eq!(config.base_url.as_str(), "http://localhost:8283/");
        assert_eq!(config.timeout, Duration::from_secs(30));
    }

    #[test]
    fn test_client_builder() {
        let client = ClientBuilder::new()
            .base_url("http://localhost:8283")
            .timeout(Duration::from_secs(60))
            .build()
            .unwrap();

        assert_eq!(client.base_url().as_str(), "http://localhost:8283/");
    }
}
