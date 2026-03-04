//! Google Forms API service client.
//!
//! Wraps the Forms REST API v1.
//! <https://developers.google.com/forms/api/reference/rest>

use crate::error::{Error, Result};
use crate::http::HttpClient;
use crate::types::forms::*;
use serde::Deserialize;

const BASE: &str = "https://forms.googleapis.com/v1";

// ---------------------------------------------------------------------------
// API response wrappers (not part of the public types)
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListResponsesResponse {
    #[serde(default)]
    responses: Vec<FormResponse>,
    #[serde(default)]
    next_page_token: Option<String>,
}

// ---------------------------------------------------------------------------
// Service
// ---------------------------------------------------------------------------

/// Async client for the Google Forms API.
#[derive(Debug, Clone)]
pub struct FormsService {
    http: HttpClient,
    token: String,
}

impl FormsService {
    /// Create a new `FormsService`.
    pub fn new(http: HttpClient, token: String) -> Self {
        Self { http, token }
    }

    fn url(&self, path: &str) -> String {
        format!("{BASE}{path}")
    }

    fn parse<T: serde::de::DeserializeOwned>(&self, bytes: &[u8]) -> Result<T> {
        serde_json::from_slice(bytes).map_err(|e| Error::Other(format!("JSON parse error: {e}")))
    }

    // -- forms --------------------------------------------------------------

    /// Get form metadata, questions, and settings.
    pub async fn get(&self, form_id: &str) -> Result<Form> {
        let url = self.url(&format!("/forms/{form_id}"));
        let resp = self.http.get(&url, &self.token).await?;
        self.parse(&resp)
    }

    /// Create a new form with a title.
    pub async fn create(&self, title: &str) -> Result<Form> {
        let body = serde_json::to_vec(&serde_json::json!({
            "info": {
                "title": title
            }
        }))
        .map_err(|e| Error::Other(e.to_string()))?;
        let url = self.url("/forms");
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    // -- responses ----------------------------------------------------------

    /// List form responses.
    pub async fn responses_list(
        &self,
        form_id: &str,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<(Vec<FormResponse>, Option<String>)> {
        let mut url = self.url(&format!("/forms/{form_id}/responses"));
        let mut sep = '?';
        if let Some(n) = page_size {
            url.push_str(&format!("{sep}pageSize={n}"));
            sep = '&';
        }
        if let Some(pt) = page_token {
            url.push_str(&format!("{sep}pageToken={}", urlencoding(pt)));
        }
        let resp = self.http.get(&url, &self.token).await?;
        let list: ListResponsesResponse = self.parse(&resp)?;
        Ok((list.responses, list.next_page_token))
    }

    /// Get a single form response by ID.
    pub async fn responses_get(&self, form_id: &str, response_id: &str) -> Result<FormResponse> {
        let url = self.url(&format!("/forms/{form_id}/responses/{response_id}"));
        let resp = self.http.get(&url, &self.token).await?;
        self.parse(&resp)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn urlencoding(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}
