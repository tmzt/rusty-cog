//! Google Tasks API service client.
//!
//! Wraps the Tasks REST API v1.
//! <https://developers.google.com/tasks/reference/rest>

use crate::error::{Error, Result};
use crate::http::HttpClient;
use crate::types::tasks::*;
use serde::Deserialize;

const BASE: &str = "https://tasks.googleapis.com/tasks/v1";

// ---------------------------------------------------------------------------
// API list-response wrappers (not part of the public types)
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListTaskListsResponse {
    #[serde(default)]
    items: Vec<TaskList>,
    #[serde(default)]
    next_page_token: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListTasksResponse {
    #[serde(default)]
    items: Vec<Task>,
    #[serde(default)]
    next_page_token: Option<String>,
}

// ---------------------------------------------------------------------------
// Service
// ---------------------------------------------------------------------------

/// Async client for the Google Tasks API.
#[derive(Debug, Clone)]
pub struct TasksService {
    http: HttpClient,
    token: String,
}

impl TasksService {
    /// Create a new `TasksService`.
    pub fn new(http: HttpClient, token: String) -> Self {
        Self { http, token }
    }

    fn parse<T: serde::de::DeserializeOwned>(&self, bytes: &[u8]) -> Result<T> {
        serde_json::from_slice(bytes).map_err(|e| Error::Other(format!("JSON parse error: {e}")))
    }

    // -- task lists ---------------------------------------------------------

    /// List all task lists.
    pub async fn tasklists(
        &self,
        page_token: Option<&str>,
    ) -> Result<(Vec<TaskList>, Option<String>)> {
        let mut url = format!("{BASE}/users/@me/lists");
        if let Some(pt) = page_token {
            url.push_str(&format!("?pageToken={}", urlencoding(pt)));
        }
        let resp = self.http.get(&url, &self.token).await?;
        let list: ListTaskListsResponse = self.parse(&resp)?;
        Ok((list.items, list.next_page_token))
    }

    // -- tasks --------------------------------------------------------------

    /// List tasks in a task list.
    pub async fn list(
        &self,
        tasklist_id: &str,
        page_token: Option<&str>,
    ) -> Result<(Vec<Task>, Option<String>)> {
        let mut url = format!(
            "{BASE}/lists/{}/tasks",
            urlencoding(tasklist_id)
        );
        if let Some(pt) = page_token {
            url.push_str(&format!("?pageToken={}", urlencoding(pt)));
        }
        let resp = self.http.get(&url, &self.token).await?;
        let list: ListTasksResponse = self.parse(&resp)?;
        Ok((list.items, list.next_page_token))
    }

    /// Get a single task by ID.
    pub async fn get(&self, tasklist_id: &str, task_id: &str) -> Result<Task> {
        let url = format!(
            "{BASE}/lists/{}/tasks/{}",
            urlencoding(tasklist_id),
            urlencoding(task_id)
        );
        let resp = self.http.get(&url, &self.token).await?;
        self.parse(&resp)
    }

    /// Create a new task in a task list.
    pub async fn create(
        &self,
        tasklist_id: &str,
        title: &str,
        notes: Option<&str>,
    ) -> Result<Task> {
        let mut task = serde_json::json!({
            "title": title,
        });
        if let Some(n) = notes {
            task["notes"] = serde_json::Value::String(n.to_string());
        }
        let body = serde_json::to_vec(&task)
            .map_err(|e| Error::Other(e.to_string()))?;
        let url = format!(
            "{BASE}/lists/{}/tasks",
            urlencoding(tasklist_id)
        );
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Mark a task as completed.
    pub async fn complete(&self, tasklist_id: &str, task_id: &str) -> Result<Task> {
        let body = serde_json::to_vec(&serde_json::json!({
            "status": "completed",
        }))
        .map_err(|e| Error::Other(e.to_string()))?;
        let url = format!(
            "{BASE}/lists/{}/tasks/{}",
            urlencoding(tasklist_id),
            urlencoding(task_id)
        );
        let resp = self.http.patch(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Permanently delete a task.
    #[cfg(feature = "destructive-permanent")]
    pub async fn delete(&self, tasklist_id: &str, task_id: &str) -> Result<()> {
        let url = format!(
            "{BASE}/lists/{}/tasks/{}",
            urlencoding(tasklist_id),
            urlencoding(task_id)
        );
        self.http.delete(&url, &self.token).await?;
        Ok(())
    }

    /// Clear all completed tasks from a task list.
    #[cfg(feature = "destructive-permanent")]
    pub async fn clear(&self, tasklist_id: &str) -> Result<()> {
        let url = format!(
            "{BASE}/lists/{}/clear",
            urlencoding(tasklist_id)
        );
        self.http.post(&url, &self.token, &[]).await?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn urlencoding(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}
