//! HTTP client and configuration for the Letta API.

use crate::auth::AuthConfig;
use crate::environment::LettaEnvironment;
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

    /// Create a new client for Letta Cloud with the given API token.
    pub fn cloud(token: impl Into<String>) -> LettaResult<Self> {
        ClientBuilder::new()
            .environment(LettaEnvironment::Cloud)
            .auth(AuthConfig::bearer(token))
            .build()
    }

    /// Create a new client for a self-hosted/local Letta server.
    pub fn local() -> LettaResult<Self> {
        ClientBuilder::new()
            .environment(LettaEnvironment::SelfHosted)
            .build()
    }

    /// Create a new client builder.
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
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

    /// Get the health API.
    pub fn health(&self) -> crate::api::HealthApi<'_> {
        crate::api::HealthApi::new(self)
    }

    /// Get the blocks API.
    pub fn blocks(&self) -> crate::api::BlocksApi<'_> {
        crate::api::BlocksApi::new(self)
    }

    // HTTP helper methods

    /// Make a GET request.
    pub async fn get<T>(&self, path: &str) -> LettaResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = self.base_url().join(path.trim_start_matches('/'))?;
        let mut headers = HeaderMap::new();
        self.auth().apply_to_headers(&mut headers)?;

        let response = self.http().get(url).headers(headers).send().await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await?;
            return Err(LettaError::from_response(status, body));
        }

        Ok(response.json().await?)
    }

    /// Make a POST request with a JSON body.
    pub async fn post<T, B>(&self, path: &str, body: &B) -> LettaResult<T>
    where
        T: serde::de::DeserializeOwned,
        B: serde::Serialize + ?Sized,
    {
        let url = self.base_url().join(path.trim_start_matches('/'))?;
        let mut headers = HeaderMap::new();
        self.auth().apply_to_headers(&mut headers)?;
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let response = self
            .http()
            .post(url)
            .headers(headers)
            .json(body)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await?;
            return Err(LettaError::from_response(status, body));
        }

        Ok(response.json().await?)
    }

    /// Make a PATCH request with a JSON body.
    pub async fn patch<T, B>(&self, path: &str, body: &B) -> LettaResult<T>
    where
        T: serde::de::DeserializeOwned,
        B: serde::Serialize + ?Sized,
    {
        let url = self.base_url().join(path.trim_start_matches('/'))?;
        let mut headers = HeaderMap::new();
        self.auth().apply_to_headers(&mut headers)?;
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let response = self
            .http()
            .patch(url)
            .headers(headers)
            .json(body)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await?;
            return Err(LettaError::from_response(status, body));
        }

        Ok(response.json().await?)
    }

    /// Make a PATCH request without a body.
    pub async fn patch_no_body<T>(&self, path: &str) -> LettaResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = self.base_url().join(path.trim_start_matches('/'))?;
        let mut headers = HeaderMap::new();
        self.auth().apply_to_headers(&mut headers)?;

        let response = self.http().patch(url).headers(headers).send().await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await?;
            return Err(LettaError::from_response(status, body));
        }

        Ok(response.json().await?)
    }

    /// Make a PUT request with a JSON body.
    pub async fn put<T, B>(&self, path: &str, body: &B) -> LettaResult<T>
    where
        T: serde::de::DeserializeOwned,
        B: serde::Serialize + ?Sized,
    {
        let url = self.base_url().join(path.trim_start_matches('/'))?;
        let mut headers = HeaderMap::new();
        self.auth().apply_to_headers(&mut headers)?;
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let response = self
            .http()
            .put(url)
            .headers(headers)
            .json(body)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await?;
            return Err(LettaError::from_response(status, body));
        }

        Ok(response.json().await?)
    }

    /// Make a DELETE request.
    pub async fn delete<T>(&self, path: &str) -> LettaResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = self.base_url().join(path.trim_start_matches('/'))?;
        let mut headers = HeaderMap::new();
        self.auth().apply_to_headers(&mut headers)?;

        let response = self.http().delete(url).headers(headers).send().await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await?;
            return Err(LettaError::from_response(status, body));
        }

        Ok(response.json().await?)
    }

    /// Make a DELETE request expecting no response body.
    pub async fn delete_no_response(&self, path: &str) -> LettaResult<()> {
        let url = self.base_url().join(path.trim_start_matches('/'))?;
        let mut headers = HeaderMap::new();
        self.auth().apply_to_headers(&mut headers)?;

        let response = self.http().delete(url).headers(headers).send().await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await?;
            return Err(LettaError::from_response(status, body));
        }

        Ok(())
    }

    /// Make a GET request with query parameters.
    pub async fn get_with_query<T, Q>(&self, path: &str, query: &Q) -> LettaResult<T>
    where
        T: serde::de::DeserializeOwned,
        Q: serde::Serialize + ?Sized,
    {
        let url = self.base_url().join(path.trim_start_matches('/'))?;
        let mut headers = HeaderMap::new();
        self.auth().apply_to_headers(&mut headers)?;

        let response = self
            .http()
            .get(url)
            .headers(headers)
            .query(query)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await?;
            return Err(LettaError::from_response(status, body));
        }

        Ok(response.json().await?)
    }

    /// Make a POST request with multipart form data.
    pub async fn post_multipart<T>(
        &self,
        path: &str,
        form: reqwest::multipart::Form,
    ) -> LettaResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = self.base_url().join(path.trim_start_matches('/'))?;
        let mut headers = HeaderMap::new();
        self.auth().apply_to_headers(&mut headers)?;

        let response = self
            .http()
            .post(url)
            .headers(headers)
            .multipart(form)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await?;
            return Err(LettaError::from_response(status, body));
        }

        Ok(response.json().await?)
    }
}

/// Builder for creating a Letta client.
#[derive(Debug, Default)]
pub struct ClientBuilder {
    environment: Option<LettaEnvironment>,
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

    /// Set the environment (Cloud or SelfHosted).
    pub fn environment(mut self, env: LettaEnvironment) -> Self {
        self.environment = Some(env);
        self
    }

    /// Set the base URL (overrides environment).
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
        // Check if we have an explicit base URL
        let has_explicit_url = self.base_url.is_some();

        // Determine base URL: explicit base_url takes precedence over environment
        let base_url = if let Some(url) = self.base_url {
            url
        } else {
            // Use environment or default to Cloud
            let env = self.environment.unwrap_or_default();
            env.base_url().to_string()
        };

        let mut config = ClientConfig::new(base_url)?;

        // Apply authentication
        if let Some(auth) = self.auth {
            config = config.auth(auth);
        } else if self.environment.unwrap_or_default().requires_auth() && !has_explicit_url {
            // Warn if using cloud environment without auth
            eprintln!("Warning: Cloud environment typically requires authentication");
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

    #[test]
    fn test_environment_based_client() {
        // Test cloud environment
        let client = ClientBuilder::new()
            .environment(LettaEnvironment::Cloud)
            .auth(AuthConfig::bearer("test-token"))
            .build()
            .unwrap();
        assert_eq!(client.base_url().as_str(), "https://api.letta.com/");

        // Test self-hosted environment
        let client = ClientBuilder::new()
            .environment(LettaEnvironment::SelfHosted)
            .build()
            .unwrap();
        assert_eq!(client.base_url().as_str(), "http://localhost:8283/");
    }

    #[test]
    fn test_convenience_constructors() {
        // Test cloud constructor
        let client = LettaClient::cloud("test-token").unwrap();
        assert_eq!(client.base_url().as_str(), "https://api.letta.com/");

        // Test local constructor
        let client = LettaClient::local().unwrap();
        assert_eq!(client.base_url().as_str(), "http://localhost:8283/");
    }

    #[test]
    fn test_base_url_overrides_environment() {
        // base_url should override environment setting
        let client = ClientBuilder::new()
            .environment(LettaEnvironment::Cloud)
            .base_url("http://custom.example.com")
            .build()
            .unwrap();
        assert_eq!(client.base_url().as_str(), "http://custom.example.com/");
    }
}
