//! Google Contacts (People API) service client.
//!
//! Wraps the People REST API v1 for contact management.
//! <https://developers.google.com/people/api/rest>

use crate::error::{Error, Result};
use crate::http::HttpClient;
use crate::types::contacts::*;
use serde::Deserialize;

const BASE: &str = "https://people.googleapis.com/v1";

// ---------------------------------------------------------------------------
// API list-response wrappers (not part of the public types)
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListConnectionsResponse {
    #[serde(default)]
    connections: Vec<Person>,
    #[serde(default)]
    next_page_token: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SearchContactsResponse {
    #[serde(default)]
    results: Vec<SearchResult>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SearchResult {
    #[serde(default)]
    person: Option<Person>,
}

// ---------------------------------------------------------------------------
// Service
// ---------------------------------------------------------------------------

/// Async client for the Google Contacts API (via People API).
#[derive(Debug, Clone)]
pub struct ContactsService {
    http: HttpClient,
    token: String,
}

impl ContactsService {
    /// Create a new `ContactsService`.
    pub fn new(http: HttpClient, token: String) -> Self {
        Self { http, token }
    }

    fn parse<T: serde::de::DeserializeOwned>(&self, bytes: &[u8]) -> Result<T> {
        serde_json::from_slice(bytes).map_err(|e| Error::Other(format!("JSON parse error: {e}")))
    }

    // -- contacts -----------------------------------------------------------

    /// List the authenticated user's contacts.
    pub async fn list(
        &self,
        page_token: Option<&str>,
        page_size: Option<u32>,
    ) -> Result<(Vec<Person>, Option<String>)> {
        let mut url = format!(
            "{BASE}/people/me/connections?personFields=names,emailAddresses,phoneNumbers"
        );
        if let Some(n) = page_size {
            url.push_str(&format!("&pageSize={n}"));
        }
        if let Some(pt) = page_token {
            url.push_str(&format!("&pageToken={}", urlencoding(pt)));
        }
        let resp = self.http.get(&url, &self.token).await?;
        let list: ListConnectionsResponse = self.parse(&resp)?;
        Ok((list.connections, list.next_page_token))
    }

    /// Search contacts by name or email.
    pub async fn search(&self, query: &str) -> Result<Vec<Person>> {
        let url = format!(
            "{BASE}/people:searchContacts?query={}&readMask=names,emailAddresses,phoneNumbers",
            urlencoding(query)
        );
        let resp = self.http.get(&url, &self.token).await?;
        let list: SearchContactsResponse = self.parse(&resp)?;
        Ok(list
            .results
            .into_iter()
            .filter_map(|r| r.person)
            .collect())
    }

    /// Create a new contact.
    pub async fn create(
        &self,
        given_name: &str,
        family_name: Option<&str>,
        email: Option<&str>,
        phone: Option<&str>,
    ) -> Result<Person> {
        let mut names = serde_json::json!([{
            "givenName": given_name,
        }]);
        if let Some(fam) = family_name {
            names[0]["familyName"] = serde_json::Value::String(fam.to_string());
        }

        let mut person = serde_json::json!({
            "names": names,
        });

        if let Some(e) = email {
            person["emailAddresses"] = serde_json::json!([{ "value": e }]);
        }
        if let Some(p) = phone {
            person["phoneNumbers"] = serde_json::json!([{ "value": p }]);
        }

        let body = serde_json::to_vec(&person)
            .map_err(|e| Error::Other(e.to_string()))?;
        let url = format!("{BASE}/people:createContact");
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Permanently delete a contact.
    #[cfg(feature = "destructive-permanent")]
    pub async fn delete(&self, resource_name: &str) -> Result<()> {
        let url = format!("{BASE}/{}:deleteContact", urlencoding(resource_name));
        self.http.delete(&url, &self.token).await?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn urlencoding(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}
