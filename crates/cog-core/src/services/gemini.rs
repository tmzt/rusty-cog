//! Gemini web conversation service client.
//!
//! Feature-gated behind `gemini-web`. STRICTLY READ-ONLY -- no write methods
//! are exposed to avoid unintended side effects on a user's Gemini account.
//!
//! This module interacts with the Gemini conversation history. The exact
//! endpoint URLs are subject to change as the API matures.

use crate::error::{Error, Result};
use crate::http::HttpClient;

const BASE: &str = "https://generativelanguage.googleapis.com/v1beta";

/// Async client for the Gemini conversation history (read-only).
#[derive(Debug, Clone)]
pub struct GeminiService {
    http: HttpClient,
    token: String,
}

impl GeminiService {
    /// Create a new `GeminiService`.
    pub fn new(http: HttpClient, token: String) -> Self {
        Self { http, token }
    }

    /// List conversations.
    pub async fn list_conversations(
        &self,
        _page_size: Option<u32>,
        _page_token: Option<&str>,
    ) -> Result<(Vec<serde_json::Value>, Option<String>)> {
        Err(Error::Other("list_conversations not yet implemented".into()))
    }

    /// Get a single conversation by ID.
    pub async fn get_conversation(&self, _conversation_id: &str) -> Result<serde_json::Value> {
        Err(Error::Other("get_conversation not yet implemented".into()))
    }

    /// Search conversations by query string.
    pub async fn search_conversations(&self, _query: &str) -> Result<Vec<serde_json::Value>> {
        Err(Error::Other("search_conversations not yet implemented".into()))
    }
}
