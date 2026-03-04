//! Gmail API service client.
//!
//! Wraps the Gmail REST API v1.
//! <https://developers.google.com/gmail/api/reference/rest>

use crate::error::{Error, Result};
use crate::http::HttpClient;
use crate::indexable::Indexable;
use crate::types::gmail::*;
use serde::{Deserialize, Serialize};

const BASE: &str = "https://gmail.googleapis.com/gmail/v1";

// ---------------------------------------------------------------------------
// API list-response wrappers (not part of the public types)
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListMessagesResponse {
    #[serde(default)]
    messages: Vec<Message>,
    #[serde(default)]
    next_page_token: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListDraftsResponse {
    #[serde(default)]
    drafts: Vec<Draft>,
    #[serde(default)]
    next_page_token: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListLabelsResponse {
    #[serde(default)]
    labels: Vec<Label>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListFiltersResponse {
    #[serde(default)]
    filter: Vec<Filter>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListDelegatesResponse {
    #[serde(default)]
    delegates: Vec<Delegate>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListSendAsResponse {
    #[serde(default)]
    send_as: Vec<SendAs>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListHistoryResponse {
    #[serde(default)]
    history: Vec<HistoryRecord>,
    #[serde(default)]
    next_page_token: Option<String>,
    #[serde(default)]
    history_id: Option<String>,
}

// ---------------------------------------------------------------------------
// Index document
// ---------------------------------------------------------------------------

/// Document yielded by the Gmail indexing implementation.
#[derive(Debug, Clone, Serialize)]
pub struct GmailIndexDocument {
    pub message_id: String,
    pub thread_id: String,
    pub subject: String,
    pub snippet: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
    pub internal_date: Option<String>,
    pub label_ids: Vec<String>,
}

// ---------------------------------------------------------------------------
// Service
// ---------------------------------------------------------------------------

/// Async client for the Gmail API.
#[derive(Debug, Clone)]
pub struct GmailService {
    http: HttpClient,
    token: String,
    user_id: String,
}

impl GmailService {
    /// Create a new `GmailService`.
    pub fn new(http: HttpClient, token: String) -> Self {
        Self {
            http,
            token,
            user_id: "me".to_string(),
        }
    }

    /// Create a `GmailService` for a specific user (delegation).
    pub fn with_user(http: HttpClient, token: String, user_id: String) -> Self {
        Self {
            http,
            token,
            user_id,
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{BASE}/users/{}{path}", self.user_id)
    }

    fn parse<T: serde::de::DeserializeOwned>(&self, bytes: &[u8]) -> Result<T> {
        serde_json::from_slice(bytes).map_err(|e| Error::Other(format!("JSON parse error: {e}")))
    }

    // -- messages -----------------------------------------------------------

    /// Full-text search over messages (returns message stubs).
    pub async fn search(
        &self,
        query: &str,
        max_results: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<(Vec<Message>, Option<String>)> {
        let mut url = format!("{}?q={}", self.url("/messages"), urlencoding(query));
        if let Some(n) = max_results {
            url.push_str(&format!("&maxResults={n}"));
        }
        if let Some(pt) = page_token {
            url.push_str(&format!("&pageToken={}", urlencoding(pt)));
        }
        let resp = self.http.get(&url, &self.token).await?;
        let list: ListMessagesResponse = self.parse(&resp)?;
        Ok((list.messages, list.next_page_token))
    }

    /// Full-text search returning full message payloads.
    pub async fn messages_search(
        &self,
        query: &str,
        max_results: Option<u32>,
    ) -> Result<Vec<Message>> {
        let (stubs, _) = self.search(query, max_results, None).await?;
        let mut full = Vec::with_capacity(stubs.len());
        for stub in &stubs {
            full.push(self.get(&stub.id).await?);
        }
        Ok(full)
    }

    /// Retrieve a full thread by ID.
    pub async fn thread_get(&self, thread_id: &str) -> Result<Thread> {
        let url = format!("{}?format=full", self.url(&format!("/threads/{thread_id}")));
        let resp = self.http.get(&url, &self.token).await?;
        self.parse(&resp)
    }

    /// Get a single message by ID.
    pub async fn get(&self, message_id: &str) -> Result<Message> {
        let url = format!(
            "{}?format=full",
            self.url(&format!("/messages/{message_id}"))
        );
        let resp = self.http.get(&url, &self.token).await?;
        self.parse(&resp)
    }

    /// Send a message.
    pub async fn send(&self, params: &ComposeParams) -> Result<Message> {
        let raw = build_raw_message(params);
        let body = serde_json::to_vec(&serde_json::json!({ "raw": raw }))
            .map_err(|e| Error::Other(e.to_string()))?;
        let url = self.url("/messages/send");
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    // -- drafts -------------------------------------------------------------

    /// List drafts.
    pub async fn drafts_list(
        &self,
        max_results: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<(Vec<Draft>, Option<String>)> {
        let mut url = self.url("/drafts");
        let mut sep = '?';
        if let Some(n) = max_results {
            url.push_str(&format!("{sep}maxResults={n}"));
            sep = '&';
        }
        if let Some(pt) = page_token {
            url.push_str(&format!("{sep}pageToken={}", urlencoding(pt)));
        }
        let resp = self.http.get(&url, &self.token).await?;
        let list: ListDraftsResponse = self.parse(&resp)?;
        Ok((list.drafts, list.next_page_token))
    }

    /// Create a draft.
    pub async fn drafts_create(&self, params: &ComposeParams) -> Result<Draft> {
        let raw = build_raw_message(params);
        let body = serde_json::to_vec(&serde_json::json!({
            "message": { "raw": raw }
        }))
        .map_err(|e| Error::Other(e.to_string()))?;
        let url = self.url("/drafts");
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Send an existing draft.
    pub async fn drafts_send(&self, draft_id: &str) -> Result<Message> {
        let body = serde_json::to_vec(&serde_json::json!({ "id": draft_id }))
            .map_err(|e| Error::Other(e.to_string()))?;
        let url = self.url("/drafts/send");
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Permanently delete a message by ID.
    #[cfg(feature = "destructive-permanent")]
    pub async fn delete(&self, message_id: &str) -> Result<()> {
        let url = self.url(&format!("/messages/{message_id}"));
        self.http.delete(&url, &self.token).await?;
        Ok(())
    }

    // -- labels -------------------------------------------------------------

    /// List all labels.
    pub async fn labels_list(&self) -> Result<Vec<Label>> {
        let url = self.url("/labels");
        let resp = self.http.get(&url, &self.token).await?;
        let list: ListLabelsResponse = self.parse(&resp)?;
        Ok(list.labels)
    }

    /// Create a label.
    pub async fn labels_create(&self, name: &str) -> Result<Label> {
        let body = serde_json::to_vec(&serde_json::json!({ "name": name }))
            .map_err(|e| Error::Other(e.to_string()))?;
        let url = self.url("/labels");
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Permanently delete a label.
    #[cfg(feature = "destructive-permanent")]
    pub async fn labels_delete(&self, label_id: &str) -> Result<()> {
        let url = self.url(&format!("/labels/{label_id}"));
        self.http.delete(&url, &self.token).await?;
        Ok(())
    }

    // -- batch operations ---------------------------------------------------

    /// Permanently delete messages by ID.
    #[cfg(feature = "destructive-permanent")]
    pub async fn batch_delete(&self, message_ids: &[String]) -> Result<()> {
        let body = serde_json::to_vec(&serde_json::json!({ "ids": message_ids }))
            .map_err(|e| Error::Other(e.to_string()))?;
        let url = self.url("/messages/batchDelete");
        self.http.post(&url, &self.token, &body).await?;
        Ok(())
    }

    /// Batch modify labels on messages (with bulk check).
    pub async fn batch_modify(
        &self,
        message_ids: &[String],
        add_label_ids: &[String],
        remove_label_ids: &[String],
    ) -> Result<()> {
        crate::destructive::check_bulk_trash(message_ids.len())?;
        let body = serde_json::to_vec(&serde_json::json!({
            "ids": message_ids,
            "addLabelIds": add_label_ids,
            "removeLabelIds": remove_label_ids,
        }))
        .map_err(|e| Error::Other(e.to_string()))?;
        let url = self.url("/messages/batchModify");
        self.http.post(&url, &self.token, &body).await?;
        Ok(())
    }

    // -- filters ------------------------------------------------------------

    /// List all filters.
    pub async fn filters_list(&self) -> Result<Vec<Filter>> {
        let url = self.url("/settings/filters");
        let resp = self.http.get(&url, &self.token).await?;
        let list: ListFiltersResponse = self.parse(&resp)?;
        Ok(list.filter)
    }

    /// Create a filter.
    pub async fn filters_create(
        &self,
        criteria: &FilterCriteria,
        action: &FilterAction,
    ) -> Result<Filter> {
        let body = serde_json::to_vec(&serde_json::json!({
            "criteria": criteria,
            "action": action,
        }))
        .map_err(|e| Error::Other(e.to_string()))?;
        let url = self.url("/settings/filters");
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Permanently delete a filter.
    #[cfg(feature = "destructive-permanent")]
    pub async fn filters_delete(&self, filter_id: &str) -> Result<()> {
        let url = self.url(&format!("/settings/filters/{filter_id}"));
        self.http.delete(&url, &self.token).await?;
        Ok(())
    }

    // -- auto-forwarding ----------------------------------------------------

    /// Get auto-forwarding settings.
    pub async fn autoforward_get(&self) -> Result<AutoForwardingSettings> {
        let url = self.url("/settings/autoForwarding");
        let resp = self.http.get(&url, &self.token).await?;
        self.parse(&resp)
    }

    /// Enable auto-forwarding to an address.
    pub async fn autoforward_enable(
        &self,
        email: &str,
        disposition: &str,
    ) -> Result<AutoForwardingSettings> {
        let body = serde_json::to_vec(&serde_json::json!({
            "enabled": true,
            "emailAddress": email,
            "disposition": disposition,
        }))
        .map_err(|e| Error::Other(e.to_string()))?;
        let url = self.url("/settings/autoForwarding");
        let resp = self.http.put(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Disable auto-forwarding.
    pub async fn autoforward_disable(&self) -> Result<AutoForwardingSettings> {
        let body = serde_json::to_vec(&serde_json::json!({
            "enabled": false,
        }))
        .map_err(|e| Error::Other(e.to_string()))?;
        let url = self.url("/settings/autoForwarding");
        let resp = self.http.put(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    // -- vacation -----------------------------------------------------------

    /// Get vacation responder settings.
    pub async fn vacation_get(&self) -> Result<VacationSettings> {
        let url = self.url("/settings/vacation");
        let resp = self.http.get(&url, &self.token).await?;
        self.parse(&resp)
    }

    /// Enable vacation responder.
    pub async fn vacation_enable(
        &self,
        settings: &VacationSettings,
    ) -> Result<VacationSettings> {
        let mut val = serde_json::to_value(settings)
            .map_err(|e| Error::Other(e.to_string()))?;
        val["enableAutoReply"] = serde_json::Value::Bool(true);
        let body = serde_json::to_vec(&val)
            .map_err(|e| Error::Other(e.to_string()))?;
        let url = self.url("/settings/vacation");
        let resp = self.http.put(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Disable vacation responder.
    pub async fn vacation_disable(&self) -> Result<VacationSettings> {
        let body = serde_json::to_vec(&serde_json::json!({
            "enableAutoReply": false,
        }))
        .map_err(|e| Error::Other(e.to_string()))?;
        let url = self.url("/settings/vacation");
        let resp = self.http.put(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    // -- delegates ----------------------------------------------------------

    /// List delegates.
    pub async fn delegates_list(&self) -> Result<Vec<Delegate>> {
        let url = self.url("/settings/delegates");
        let resp = self.http.get(&url, &self.token).await?;
        let list: ListDelegatesResponse = self.parse(&resp)?;
        Ok(list.delegates)
    }

    /// Add a delegate.
    pub async fn delegates_add(&self, delegate_email: &str) -> Result<Delegate> {
        let body = serde_json::to_vec(&serde_json::json!({
            "delegateEmail": delegate_email,
        }))
        .map_err(|e| Error::Other(e.to_string()))?;
        let url = self.url("/settings/delegates");
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Remove a delegate.
    pub async fn delegates_remove(&self, delegate_email: &str) -> Result<()> {
        let url = self.url(&format!("/settings/delegates/{delegate_email}"));
        self.http.delete(&url, &self.token).await?;
        Ok(())
    }

    // -- send-as ------------------------------------------------------------

    /// List send-as aliases.
    pub async fn sendas_list(&self) -> Result<Vec<SendAs>> {
        let url = self.url("/settings/sendAs");
        let resp = self.http.get(&url, &self.token).await?;
        let list: ListSendAsResponse = self.parse(&resp)?;
        Ok(list.send_as)
    }

    /// Create a send-as alias.
    pub async fn sendas_create(
        &self,
        email: &str,
        display_name: Option<&str>,
    ) -> Result<SendAs> {
        let mut obj = serde_json::json!({ "sendAsEmail": email });
        if let Some(name) = display_name {
            obj["displayName"] = serde_json::Value::String(name.to_string());
        }
        let body = serde_json::to_vec(&obj)
            .map_err(|e| Error::Other(e.to_string()))?;
        let url = self.url("/settings/sendAs");
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    // -- push notifications -------------------------------------------------

    /// Start a push notification watch on the mailbox.
    pub async fn watch_start(
        &self,
        topic: &str,
        label_ids: Option<&[String]>,
    ) -> Result<serde_json::Value> {
        let mut obj = serde_json::json!({
            "topicName": topic,
        });
        if let Some(ids) = label_ids {
            obj["labelIds"] = serde_json::json!(ids);
        }
        let body = serde_json::to_vec(&obj)
            .map_err(|e| Error::Other(e.to_string()))?;
        let url = self.url("/watch");
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    // -- history ------------------------------------------------------------

    /// List history records starting from a history ID.
    pub async fn history(
        &self,
        start_history_id: &str,
        page_token: Option<&str>,
    ) -> Result<(Vec<HistoryRecord>, Option<String>, Option<String>)> {
        let mut url = format!(
            "{}?startHistoryId={}",
            self.url("/history"),
            urlencoding(start_history_id)
        );
        if let Some(pt) = page_token {
            url.push_str(&format!("&pageToken={}", urlencoding(pt)));
        }
        let resp = self.http.get(&url, &self.token).await?;
        let list: ListHistoryResponse = self.parse(&resp)?;
        Ok((list.history, list.next_page_token, list.history_id))
    }

    // -- single-message operations ------------------------------------------

    /// Move a message to Trash.
    pub async fn trash(&self, message_id: &str) -> Result<Message> {
        let url = self.url(&format!("/messages/{message_id}/trash"));
        let resp = self.http.post(&url, &self.token, &[]).await?;
        self.parse(&resp)
    }

    /// Modify labels on a single message.
    pub async fn modify(
        &self,
        message_id: &str,
        add_label_ids: &[String],
        remove_label_ids: &[String],
    ) -> Result<Message> {
        let body = serde_json::to_vec(&serde_json::json!({
            "addLabelIds": add_label_ids,
            "removeLabelIds": remove_label_ids,
        }))
        .map_err(|e| Error::Other(e.to_string()))?;
        let url = self.url(&format!("/messages/{message_id}/modify"));
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }
}

// ---------------------------------------------------------------------------
// Indexable
// ---------------------------------------------------------------------------

impl Indexable for GmailService {
    type Document = GmailIndexDocument;

    async fn fetch_indexable(
        &self,
        since: Option<&str>,
        limit: usize,
    ) -> Result<(Vec<GmailIndexDocument>, Option<String>)> {
        // Use history API if we have a cursor, otherwise search recent
        let messages = if let Some(history_id) = since {
            let (records, _, new_history_id) = self.history(history_id, None).await?;
            let msg_ids: Vec<String> = records
                .into_iter()
                .flat_map(|r| r.messages.into_iter().map(|m| m.id))
                .collect();
            let mut msgs = Vec::new();
            for id in msg_ids.iter().take(limit) {
                if let Ok(msg) = self.get(id).await {
                    msgs.push(msg);
                }
            }
            return Ok((
                msgs.iter().map(message_to_index_doc).collect(),
                new_history_id,
            ));
        } else {
            let (stubs, _) = self
                .search("in:anywhere", Some(limit as u32), None)
                .await?;
            let mut msgs = Vec::new();
            for stub in &stubs {
                if let Ok(msg) = self.get(&stub.id).await {
                    msgs.push(msg);
                }
            }
            msgs
        };

        let cursor = messages
            .first()
            .and_then(|m| m.history_id.clone());
        let docs: Vec<GmailIndexDocument> = messages.iter().map(message_to_index_doc).collect();
        Ok((docs, cursor))
    }

    fn index_namespace(&self) -> &'static str {
        "gmail"
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn urlencoding(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}

/// Build a base64url-encoded RFC 2822 message from ComposeParams.
fn build_raw_message(params: &ComposeParams) -> String {
    let mut headers = String::new();
    headers.push_str(&format!("To: {}\r\n", params.to.join(", ")));
    if !params.cc.is_empty() {
        headers.push_str(&format!("Cc: {}\r\n", params.cc.join(", ")));
    }
    if !params.bcc.is_empty() {
        headers.push_str(&format!("Bcc: {}\r\n", params.bcc.join(", ")));
    }
    headers.push_str(&format!("Subject: {}\r\n", params.subject));

    if let Some(reply_id) = &params.reply_to_message_id {
        headers.push_str(&format!("In-Reply-To: <{reply_id}>\r\n"));
        headers.push_str(&format!("References: <{reply_id}>\r\n"));
    }

    let body_text = params
        .body
        .as_deref()
        .unwrap_or("");

    if let Some(html) = &params.body_html {
        // MIME multipart for HTML + plain text
        let boundary = uuid::Uuid::new_v4().to_string();
        headers.push_str("MIME-Version: 1.0\r\n");
        headers.push_str(&format!(
            "Content-Type: multipart/alternative; boundary=\"{boundary}\"\r\n"
        ));
        let message = format!(
            "{headers}\r\n--{boundary}\r\nContent-Type: text/plain; charset=\"UTF-8\"\r\n\r\n{body_text}\r\n--{boundary}\r\nContent-Type: text/html; charset=\"UTF-8\"\r\n\r\n{html}\r\n--{boundary}--"
        );
        base64_url_encode(message.as_bytes())
    } else {
        headers.push_str("Content-Type: text/plain; charset=\"UTF-8\"\r\n");
        let message = format!("{headers}\r\n{body_text}");
        base64_url_encode(message.as_bytes())
    }
}

fn base64_url_encode(data: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(data)
}

fn message_to_index_doc(msg: &Message) -> GmailIndexDocument {
    let payload = msg.payload.as_ref();
    let headers: Vec<(String, String)> = payload
        .map(|p| {
            p.headers
                .iter()
                .map(|h| (h.name.clone(), h.value.clone()))
                .collect()
        })
        .unwrap_or_default();

    let subject = headers
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case("Subject"))
        .map(|(_, v)| v.clone())
        .unwrap_or_default();

    let body = extract_body_text(payload);

    GmailIndexDocument {
        message_id: msg.id.clone(),
        thread_id: msg.thread_id.clone().unwrap_or_default(),
        subject,
        snippet: msg.snippet.clone().unwrap_or_default(),
        headers,
        body,
        internal_date: msg.internal_date.clone(),
        label_ids: msg.label_ids.clone(),
    }
}

fn extract_body_text(payload: Option<&MessagePart>) -> String {
    let Some(part) = payload else {
        return String::new();
    };

    // Check for text/plain body data
    if let Some(mime) = &part.mime_type {
        if mime == "text/plain" {
            if let Some(body) = &part.body {
                if let Some(data) = &body.data {
                    return decode_base64url(data);
                }
            }
        }
    }

    // Recurse into sub-parts
    for sub in &part.parts {
        let text = extract_body_text(Some(sub));
        if !text.is_empty() {
            return text;
        }
    }

    String::new()
}

fn decode_base64url(data: &str) -> String {
    use base64::Engine;
    base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(data)
        .ok()
        .and_then(|bytes| String::from_utf8(bytes).ok())
        .unwrap_or_default()
}
