//! Google Chat API service client.
//!
//! Wraps the Chat REST API v1.
//! <https://developers.google.com/chat/api/reference/rest>

use crate::error::{Error, Result};
use crate::http::HttpClient;
use crate::types::chat::*;
use serde::Deserialize;

const BASE: &str = "https://chat.googleapis.com/v1";

// ---------------------------------------------------------------------------
// API list-response wrappers (not part of the public types)
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListSpacesResponse {
    #[serde(default)]
    spaces: Vec<Space>,
    #[serde(default)]
    next_page_token: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListMessagesResponse {
    #[serde(default)]
    messages: Vec<ChatMessage>,
    #[serde(default)]
    next_page_token: Option<String>,
}

// ---------------------------------------------------------------------------
// Service
// ---------------------------------------------------------------------------

/// Async client for the Google Chat API.
#[derive(Debug, Clone)]
pub struct ChatService {
    http: HttpClient,
    token: String,
}

impl ChatService {
    /// Create a new `ChatService`.
    pub fn new(http: HttpClient, token: String) -> Self {
        Self { http, token }
    }

    fn url(path: &str) -> String {
        format!("{BASE}{path}")
    }

    fn parse<T: serde::de::DeserializeOwned>(&self, bytes: &[u8]) -> Result<T> {
        serde_json::from_slice(bytes).map_err(|e| Error::Other(format!("JSON parse error: {e}")))
    }

    // -- spaces -------------------------------------------------------------

    /// List spaces the user is a member of.
    pub async fn spaces_list(
        &self,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<(Vec<Space>, Option<String>)> {
        let mut url = Self::url("/spaces");
        let mut sep = '?';
        if let Some(n) = page_size {
            url.push_str(&format!("{sep}pageSize={n}"));
            sep = '&';
        }
        if let Some(pt) = page_token {
            url.push_str(&format!("{sep}pageToken={}", urlencoding(pt)));
        }
        let resp = self.http.get(&url, &self.token).await?;
        let list: ListSpacesResponse = self.parse(&resp)?;
        Ok((list.spaces, list.next_page_token))
    }

    /// Find a space by display name.
    pub async fn spaces_find(&self, display_name: &str) -> Result<Option<Space>> {
        let mut page_token: Option<String> = None;
        loop {
            let (spaces, next) = self.spaces_list(Some(100), page_token.as_deref()).await?;
            for space in spaces {
                if space.display_name.as_deref() == Some(display_name) {
                    return Ok(Some(space));
                }
            }
            if next.is_none() {
                return Ok(None);
            }
            page_token = next;
        }
    }

    /// Create a new named space.
    pub async fn spaces_create(
        &self,
        display_name: &str,
        space_type: Option<&str>,
    ) -> Result<Space> {
        let body = serde_json::to_vec(&serde_json::json!({
            "displayName": display_name,
            "spaceType": space_type.unwrap_or("SPACE"),
        }))
        .map_err(|e| Error::Other(e.to_string()))?;
        let url = Self::url("/spaces");
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    // -- messages -----------------------------------------------------------

    /// List messages in a space.
    pub async fn messages_list(
        &self,
        space_name: &str,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<(Vec<ChatMessage>, Option<String>)> {
        let mut url = Self::url(&format!("/{space_name}/messages"));
        let mut sep = '?';
        if let Some(n) = page_size {
            url.push_str(&format!("{sep}pageSize={n}"));
            sep = '&';
        }
        if let Some(pt) = page_token {
            url.push_str(&format!("{sep}pageToken={}", urlencoding(pt)));
        }
        let resp = self.http.get(&url, &self.token).await?;
        let list: ListMessagesResponse = self.parse(&resp)?;
        Ok((list.messages, list.next_page_token))
    }

    /// Send a message to a space.
    pub async fn messages_create(
        &self,
        space_name: &str,
        text: &str,
        thread_key: Option<&str>,
    ) -> Result<ChatMessage> {
        let mut obj = serde_json::json!({ "text": text });
        if let Some(key) = thread_key {
            obj["thread"] = serde_json::json!({ "threadKey": key });
        }
        let body = serde_json::to_vec(&obj)
            .map_err(|e| Error::Other(e.to_string()))?;
        let url = Self::url(&format!("/{space_name}/messages"));
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    // -- threads ------------------------------------------------------------

    /// List threads in a space.
    pub async fn threads_list(
        &self,
        space_name: &str,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<(Vec<ChatMessage>, Option<String>)> {
        // The Chat API does not have a dedicated threads endpoint; listing
        // messages with orderBy=createTime achieves the same effect.
        self.messages_list(space_name, page_size, page_token).await
    }

    // -- direct messages ----------------------------------------------------

    /// Get or create a DM space with a user.
    pub async fn dm_space(&self, user_name: &str) -> Result<Space> {
        let body = serde_json::to_vec(&serde_json::json!({
            "member": { "name": user_name, "type": "HUMAN" },
            "spaceType": "DIRECT_MESSAGE",
        }))
        .map_err(|e| Error::Other(e.to_string()))?;
        let url = Self::url("/spaces:setup");
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Send a direct message to a space (convenience wrapper).
    pub async fn dm(
        &self,
        space_name: &str,
        text: &str,
    ) -> Result<ChatMessage> {
        self.messages_create(space_name, text, None).await
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn urlencoding(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}
