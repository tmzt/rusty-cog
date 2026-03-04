//! NotebookLM service client.
//!
//! Feature-gated behind `notebooklm`. Read-only access to NotebookLM
//! notebooks and their content.

use crate::error::{Error, Result};
use crate::http::HttpClient;

const BASE: &str = "https://notebooklm.googleapis.com/v1";

/// Async client for the NotebookLM API (read-only).
#[derive(Debug, Clone)]
pub struct NotebookLmService {
    http: HttpClient,
    token: String,
}

impl NotebookLmService {
    /// Create a new `NotebookLmService`.
    pub fn new(http: HttpClient, token: String) -> Self {
        Self { http, token }
    }

    /// List notebooks.
    pub async fn list(
        &self,
        _page_size: Option<u32>,
        _page_token: Option<&str>,
    ) -> Result<(Vec<serde_json::Value>, Option<String>)> {
        Err(Error::Other("list not yet implemented".into()))
    }

    /// Get a single notebook by ID.
    pub async fn get(&self, _notebook_id: &str) -> Result<serde_json::Value> {
        Err(Error::Other("get not yet implemented".into()))
    }
}
