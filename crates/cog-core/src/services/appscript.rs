//! Google Apps Script API service client.
//!
//! Wraps the Apps Script REST API v1.
//! <https://developers.google.com/apps-script/api/reference/rest>

use crate::error::{Error, Result};
use crate::http::HttpClient;
use crate::types::appscript::*;

const BASE: &str = "https://script.googleapis.com/v1";

// ---------------------------------------------------------------------------
// Service
// ---------------------------------------------------------------------------

/// Async client for the Google Apps Script API.
#[derive(Debug, Clone)]
pub struct AppScriptService {
    http: HttpClient,
    token: String,
}

impl AppScriptService {
    /// Create a new `AppScriptService`.
    pub fn new(http: HttpClient, token: String) -> Self {
        Self { http, token }
    }

    fn parse<T: serde::de::DeserializeOwned>(&self, bytes: &[u8]) -> Result<T> {
        serde_json::from_slice(bytes).map_err(|e| Error::Other(format!("JSON parse error: {e}")))
    }

    /// Get script project metadata.
    pub async fn get(&self, script_id: &str) -> Result<Project> {
        let url = format!("{BASE}/projects/{script_id}");
        let resp = self.http.get(&url, &self.token).await?;
        self.parse(&resp)
    }

    /// Get the content (source files) of a script project.
    pub async fn content(&self, script_id: &str) -> Result<ScriptContent> {
        let url = format!("{BASE}/projects/{script_id}/content");
        let resp = self.http.get(&url, &self.token).await?;
        self.parse(&resp)
    }

    /// Create a new script project.
    pub async fn create(&self, title: &str) -> Result<Project> {
        let body = serde_json::to_vec(&serde_json::json!({ "title": title }))
            .map_err(|e| Error::Other(e.to_string()))?;
        let url = format!("{BASE}/projects");
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Run a function in a script project.
    pub async fn run(
        &self,
        script_id: &str,
        function_name: &str,
        parameters: Option<&[serde_json::Value]>,
    ) -> Result<ExecutionResponse> {
        let body = serde_json::to_vec(&serde_json::json!({
            "function": function_name,
            "parameters": parameters.unwrap_or(&[]),
        }))
        .map_err(|e| Error::Other(e.to_string()))?;
        let url = format!("{BASE}/scripts/{script_id}:run");
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }
}
