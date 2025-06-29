//! Common types used across the Letta API.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Unique identifier for resources in the Letta API.
pub type ResourceId = Uuid;

/// Timestamp type used throughout the API.
pub type Timestamp = DateTime<Utc>;

/// Generic pagination parameters for list operations.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaginationParams {
    /// Cursor for pagination (before this ID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Cursor for pagination (after this ID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Maximum number of items to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Whether to return results in ascending order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending: Option<bool>,
}

impl PaginationParams {
    /// Create new pagination parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the before cursor.
    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Set the after cursor.
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Set the limit.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set ascending order.
    pub fn ascending(mut self, ascending: bool) -> Self {
        self.ascending = Some(ascending);
        self
    }
}

/// Generic response wrapper for paginated results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    /// The items in this page.
    pub items: Vec<T>,
    /// Total count of items (if available).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<u64>,
    /// Cursor for the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// Cursor for the previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_cursor: Option<String>,
    /// Whether there are more items available.
    #[serde(default)]
    pub has_more: bool,
}

impl<T> PaginatedResponse<T> {
    /// Create a new paginated response.
    pub fn new(items: Vec<T>) -> Self {
        Self {
            items,
            total: None,
            next_cursor: None,
            prev_cursor: None,
            has_more: false,
        }
    }

    /// Get the number of items in this page.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if this page is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get an iterator over the items.
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.items.iter()
    }
}

impl<T> IntoIterator for PaginatedResponse<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

/// Standard error response from the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    /// Error message.
    pub message: String,
    /// Optional error code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Optional additional details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<HashMap<String, serde_json::Value>>,
}

/// Health check response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    /// Health status.
    pub status: String,
    /// Optional version information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Timestamp of the health check.
    pub timestamp: Timestamp,
}

/// Count response for count endpoints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountResponse {
    /// The count value.
    pub count: u64,
}

/// Generic metadata for resources.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Metadata {
    /// Arbitrary key-value metadata.
    #[serde(flatten)]
    pub data: HashMap<String, serde_json::Value>,
}

impl Metadata {
    /// Create new empty metadata.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a key-value pair.
    pub fn insert(&mut self, key: String, value: serde_json::Value) {
        self.data.insert(key, value);
    }

    /// Get a value by key.
    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.data.get(key)
    }

    /// Check if metadata is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

/// Sort order for list operations.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    /// Ascending order.
    Asc,
    /// Descending order.
    Desc,
}

impl Default for SortOrder {
    fn default() -> Self {
        Self::Desc
    }
}

/// Common query parameters for list operations.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListParams {
    /// Pagination parameters.
    #[serde(flatten)]
    pub pagination: PaginationParams,
    /// Optional search query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Optional sort field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<SortOrder>,
}

impl ListParams {
    /// Create new list parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the search query.
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }

    /// Set the sort field.
    pub fn sort_by(mut self, sort_by: impl Into<String>) -> Self {
        self.sort_by = Some(sort_by.into());
        self
    }

    /// Set the sort order.
    pub fn sort_order(mut self, order: SortOrder) -> Self {
        self.sort_order = Some(order);
        self
    }

    /// Set pagination parameters.
    pub fn pagination(mut self, pagination: PaginationParams) -> Self {
        self.pagination = pagination;
        self
    }

    /// Set the limit.
    pub fn limit(mut self, limit: u32) -> Self {
        self.pagination.limit = Some(limit);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_params() {
        let params = PaginationParams::new()
            .limit(10)
            .after("cursor123")
            .ascending(true);

        assert_eq!(params.limit, Some(10));
        assert_eq!(params.after.as_deref(), Some("cursor123"));
        assert_eq!(params.ascending, Some(true));
    }

    #[test]
    fn test_paginated_response() {
        let items = vec![1, 2, 3];
        let response = PaginatedResponse::new(items);

        assert_eq!(response.len(), 3);
        assert!(!response.is_empty());
        assert!(!response.has_more);
    }

    #[test]
    fn test_metadata() {
        let mut metadata = Metadata::new();
        assert!(metadata.is_empty());

        metadata.insert("key".to_string(), serde_json::Value::String("value".to_string()));
        assert!(!metadata.is_empty());
        assert!(metadata.get("key").is_some());
    }

    #[test]
    fn test_list_params() {
        let params = ListParams::new()
            .query("search term")
            .sort_by("created_at")
            .sort_order(SortOrder::Asc)
            .limit(20);

        assert_eq!(params.query.as_deref(), Some("search term"));
        assert_eq!(params.sort_by.as_deref(), Some("created_at"));
        assert!(matches!(params.sort_order, Some(SortOrder::Asc)));
        assert_eq!(params.pagination.limit, Some(20));
    }
}