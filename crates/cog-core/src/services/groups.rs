//! Google Groups (Cloud Identity) API service client.
//!
//! Wraps the Cloud Identity REST API v1 for group operations.
//! <https://cloud.google.com/identity/docs/reference/rest>

use crate::error::{Error, Result};
use crate::http::HttpClient;
use crate::types::groups::*;
use serde::Deserialize;

const BASE: &str = "https://cloudidentity.googleapis.com/v1";

// ---------------------------------------------------------------------------
// API list-response wrappers (not part of the public types)
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListGroupsResponse {
    #[serde(default)]
    groups: Vec<Group>,
    #[serde(default)]
    next_page_token: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListMembershipsResponse {
    #[serde(default)]
    memberships: Vec<GroupMember>,
    #[serde(default)]
    next_page_token: Option<String>,
}

// ---------------------------------------------------------------------------
// Service
// ---------------------------------------------------------------------------

/// Async client for the Google Groups API (via Cloud Identity).
#[derive(Debug, Clone)]
pub struct GroupsService {
    http: HttpClient,
    token: String,
}

impl GroupsService {
    /// Create a new `GroupsService`.
    pub fn new(http: HttpClient, token: String) -> Self {
        Self { http, token }
    }

    fn parse<T: serde::de::DeserializeOwned>(&self, bytes: &[u8]) -> Result<T> {
        serde_json::from_slice(bytes).map_err(|e| Error::Other(format!("JSON parse error: {e}")))
    }

    /// List groups in the organization.
    ///
    /// `parent` is a resource name like `customers/my_customer`.
    pub async fn list(
        &self,
        parent: &str,
        page_token: Option<&str>,
    ) -> Result<(Vec<Group>, Option<String>)> {
        let mut url = format!(
            "{BASE}/groups?parent={}",
            urlencoding(parent)
        );
        if let Some(pt) = page_token {
            url.push_str(&format!("&pageToken={}", urlencoding(pt)));
        }
        let resp = self.http.get(&url, &self.token).await?;
        let list: ListGroupsResponse = self.parse(&resp)?;
        Ok((list.groups, list.next_page_token))
    }

    /// List members of a group.
    ///
    /// `group_name` is a resource name like `groups/abc123`.
    pub async fn members(
        &self,
        group_name: &str,
        page_token: Option<&str>,
    ) -> Result<(Vec<GroupMember>, Option<String>)> {
        let mut url = format!("{BASE}/{group_name}/memberships");
        let mut sep = '?';
        if let Some(pt) = page_token {
            url.push_str(&format!("{sep}pageToken={}", urlencoding(pt)));
            sep = '&';
        }
        let _ = sep; // suppress unused warning
        let resp = self.http.get(&url, &self.token).await?;
        let list: ListMembershipsResponse = self.parse(&resp)?;
        Ok((list.memberships, list.next_page_token))
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn urlencoding(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}
