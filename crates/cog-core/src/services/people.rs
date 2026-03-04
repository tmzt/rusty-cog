//! Google People API service client.
//!
//! Wraps the People REST API v1 for profile and directory lookups.
//! <https://developers.google.com/people/api/rest>

use crate::error::{Error, Result};
use crate::http::HttpClient;
use crate::types::people::*;
use serde::Deserialize;

const BASE: &str = "https://people.googleapis.com/v1";

// ---------------------------------------------------------------------------
// API list-response wrappers (not part of the public types)
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SearchDirectoryPeopleResponse {
    #[serde(default)]
    people: Vec<PersonProfile>,
    #[serde(default)]
    next_page_token: Option<String>,
}

// ---------------------------------------------------------------------------
// Service
// ---------------------------------------------------------------------------

/// Async client for the Google People API.
#[derive(Debug, Clone)]
pub struct PeopleService {
    http: HttpClient,
    token: String,
}

impl PeopleService {
    /// Create a new `PeopleService`.
    pub fn new(http: HttpClient, token: String) -> Self {
        Self { http, token }
    }

    fn parse<T: serde::de::DeserializeOwned>(&self, bytes: &[u8]) -> Result<T> {
        serde_json::from_slice(bytes).map_err(|e| Error::Other(format!("JSON parse error: {e}")))
    }

    // -- profiles -----------------------------------------------------------

    /// Get the authenticated user's profile.
    pub async fn me(&self) -> Result<PersonProfile> {
        let url = format!("{BASE}/people/me?personFields=names,emailAddresses,photos");
        let resp = self.http.get(&url, &self.token).await?;
        self.parse(&resp)
    }

    /// Get a person's profile by resource name.
    pub async fn get(&self, resource_name: &str) -> Result<PersonProfile> {
        let url = format!(
            "{BASE}/{}?personFields=names,emailAddresses,photos",
            urlencoding(resource_name)
        );
        let resp = self.http.get(&url, &self.token).await?;
        self.parse(&resp)
    }

    /// Search people in the directory by query string.
    pub async fn search(&self, query: &str) -> Result<Vec<PersonProfile>> {
        let url = format!(
            "{BASE}/people:searchDirectoryPeople?query={}&readMask=names,emailAddresses&sources=DIRECTORY_SOURCE_TYPE_DOMAIN_PROFILE",
            urlencoding(query)
        );
        let resp = self.http.get(&url, &self.token).await?;
        let list: SearchDirectoryPeopleResponse = self.parse(&resp)?;
        Ok(list.people)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn urlencoding(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}
