//! Google Keep API service client.
//!
//! Wraps the Keep REST API v1.
//! <https://developers.google.com/keep/api/reference/rest>

use crate::error::{Error, Result};
use crate::http::HttpClient;
use crate::indexable::Indexable;
use crate::types::keep::*;
use serde::{Deserialize, Serialize};

const BASE: &str = "https://keep.googleapis.com/v1";

// ---------------------------------------------------------------------------
// API list-response wrappers (not part of the public types)
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListNotesResponse {
    #[serde(default)]
    notes: Vec<Note>,
    #[serde(default)]
    next_page_token: Option<String>,
}

// ---------------------------------------------------------------------------
// Index document
// ---------------------------------------------------------------------------

/// Document yielded by the Keep indexing implementation.
#[derive(Debug, Clone, Serialize)]
pub struct KeepIndexDocument {
    pub note_id: String,
    pub title: String,
    pub body: String,
    pub update_time: Option<String>,
    pub trashed: bool,
    pub labels: Vec<String>,
}

// ---------------------------------------------------------------------------
// Service
// ---------------------------------------------------------------------------

/// Async client for the Google Keep API.
#[derive(Debug, Clone)]
pub struct KeepService {
    http: HttpClient,
    token: String,
}

impl KeepService {
    /// Create a new `KeepService`.
    pub fn new(http: HttpClient, token: String) -> Self {
        Self { http, token }
    }

    fn parse<T: serde::de::DeserializeOwned>(&self, bytes: &[u8]) -> Result<T> {
        serde_json::from_slice(bytes).map_err(|e| Error::Other(format!("JSON parse error: {e}")))
    }

    /// List notes.
    pub async fn list(
        &self,
        page_token: Option<&str>,
    ) -> Result<(Vec<Note>, Option<String>)> {
        let mut url = format!("{BASE}/notes?pageSize=100");
        if let Some(pt) = page_token {
            url.push_str(&format!("&pageToken={}", urlencoding(pt)));
        }
        let resp = self.http.get(&url, &self.token).await?;
        let list: ListNotesResponse = self.parse(&resp)?;
        Ok((list.notes, list.next_page_token))
    }

    /// Get a single note by resource name (e.g. `notes/abc123`).
    pub async fn get(&self, note_name: &str) -> Result<Note> {
        let url = format!("{BASE}/{note_name}");
        let resp = self.http.get(&url, &self.token).await?;
        self.parse(&resp)
    }

    /// Search notes using a filter expression.
    pub async fn search(&self, query: &str) -> Result<Vec<Note>> {
        let mut all_notes = Vec::new();
        let mut page_token: Option<String> = None;

        loop {
            let mut url = format!(
                "{BASE}/notes?pageSize=100&filter={}",
                urlencoding(query)
            );
            if let Some(ref pt) = page_token {
                url.push_str(&format!("&pageToken={}", urlencoding(pt)));
            }
            let resp = self.http.get(&url, &self.token).await?;
            let list: ListNotesResponse = self.parse(&resp)?;
            all_notes.extend(list.notes);
            match list.next_page_token {
                Some(pt) => page_token = Some(pt),
                None => break,
            }
        }

        Ok(all_notes)
    }
}

// ---------------------------------------------------------------------------
// Indexable
// ---------------------------------------------------------------------------

impl Indexable for KeepService {
    type Document = KeepIndexDocument;

    async fn fetch_indexable(
        &self,
        since: Option<&str>,
        limit: usize,
    ) -> Result<(Vec<KeepIndexDocument>, Option<String>)> {
        let mut all_notes = Vec::new();
        let mut page_token: Option<String> = None;

        // Fetch notes, paginating until we have enough
        loop {
            let (notes, next) = self.list(page_token.as_deref()).await?;
            all_notes.extend(notes);
            if all_notes.len() >= limit || next.is_none() {
                break;
            }
            page_token = next;
        }

        all_notes.truncate(limit);

        // If we have a cursor (updateTime), filter to only notes updated after it
        if let Some(cursor) = since {
            all_notes.retain(|note| {
                note.update_time
                    .as_deref()
                    .map(|ut| ut > cursor)
                    .unwrap_or(false)
            });
        }

        // Determine new cursor: latest updateTime among fetched notes
        let new_cursor = all_notes
            .iter()
            .filter_map(|n| n.update_time.as_deref())
            .max()
            .map(String::from);

        let docs: Vec<KeepIndexDocument> = all_notes.iter().map(note_to_index_doc).collect();
        Ok((docs, new_cursor))
    }

    fn index_namespace(&self) -> &'static str {
        "keep"
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn urlencoding(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}

/// Convert a Keep `Note` into a `KeepIndexDocument`.
fn note_to_index_doc(note: &Note) -> KeepIndexDocument {
    let note_id = note
        .name
        .as_deref()
        .unwrap_or_default()
        .to_string();

    let title = note.title.as_deref().unwrap_or_default().to_string();

    let body = extract_note_body(note);

    let labels = Vec::new();

    KeepIndexDocument {
        note_id,
        title,
        body,
        update_time: note.update_time.clone(),
        trashed: note.trashed,
        labels,
    }
}

/// Extract text content from a note's body (text content or list items).
fn extract_note_body(note: &Note) -> String {
    let Some(body) = &note.body else {
        return String::new();
    };

    let mut parts = Vec::new();

    // Text content
    if let Some(text_content) = &body.text {
        if let Some(text) = &text_content.text {
            parts.push(text.clone());
        }
    }

    // List content
    if let Some(list_content) = &body.list {
        for item in &list_content.list_items {
            collect_list_item_text(item, &mut parts, 0);
        }
    }

    parts.join("\n")
}

/// Recursively collect text from list items and their children.
fn collect_list_item_text(item: &ListItem, parts: &mut Vec<String>, depth: usize) {
    let prefix = if item.checked { "[x] " } else { "[ ] " };
    let indent = "  ".repeat(depth);
    if let Some(text_content) = &item.text {
        if let Some(text) = &text_content.text {
            parts.push(format!("{indent}{prefix}{text}"));
        }
    }
    for child in &item.children_list_items {
        collect_list_item_text(child, parts, depth + 1);
    }
}
