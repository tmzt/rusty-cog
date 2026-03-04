//! Google Drive API service client.
//!
//! Wraps the Drive REST API v3.
//! <https://developers.google.com/drive/api/reference/rest/v3>

use crate::error::{Error, Result};
use crate::http::HttpClient;
use crate::indexable::Indexable;
use crate::types::drive::*;
use serde::{Deserialize, Serialize};

const BASE: &str = "https://www.googleapis.com/drive/v3";
const UPLOAD_BASE: &str = "https://www.googleapis.com/upload/drive/v3";

/// Default fields to request for file metadata.
const FILE_FIELDS: &str = "id,name,mimeType,description,starred,trashed,parents,\
    webViewLink,webContentLink,iconLink,thumbnailLink,createdTime,modifiedTime,\
    size,owners,shared,permissions,version,md5Checksum,sha1Checksum,sha256Checksum,\
    originalFilename,fullFileExtension,fileExtension,quotaBytesUsed,driveId,\
    capabilities,headRevisionId";

/// Default fields for file list responses.
const LIST_FIELDS: &str = "nextPageToken,incompleteSearch,files(id,name,mimeType,\
    description,starred,trashed,parents,webViewLink,webContentLink,createdTime,\
    modifiedTime,size,owners,shared,driveId)";

// ---------------------------------------------------------------------------
// API response wrappers (not part of the public types)
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListFilesResponse {
    #[serde(default)]
    files: Vec<File>,
    #[serde(default)]
    next_page_token: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListDrivesResponse {
    #[serde(default)]
    drives: Vec<SharedDrive>,
    #[serde(default)]
    next_page_token: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListChangesResponse {
    #[serde(default)]
    changes: Vec<Change>,
    #[serde(default)]
    next_page_token: Option<String>,
    #[serde(default)]
    new_start_page_token: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct StartPageTokenResponse {
    #[serde(default)]
    start_page_token: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListPermissionsResponse {
    #[serde(default)]
    permissions: Vec<Permission>,
}

// ---------------------------------------------------------------------------
// Index document
// ---------------------------------------------------------------------------

/// Document yielded by the Drive indexing implementation.
#[derive(Debug, Clone, Serialize)]
pub struct DriveIndexDocument {
    pub file_id: String,
    pub name: String,
    pub mime_type: String,
    pub modified_time: Option<String>,
    pub owners: Vec<String>,
    pub parents: Vec<String>,
    pub web_view_link: Option<String>,
    pub size: Option<i64>,
}

// ---------------------------------------------------------------------------
// Service
// ---------------------------------------------------------------------------

/// Async client for the Google Drive API.
#[derive(Debug, Clone)]
pub struct DriveService {
    http: HttpClient,
    token: String,
}

impl DriveService {
    /// Create a new `DriveService`.
    pub fn new(http: HttpClient, token: String) -> Self {
        Self { http, token }
    }

    fn url(&self, path: &str) -> String {
        format!("{BASE}{path}")
    }

    fn upload_url(&self, path: &str) -> String {
        format!("{UPLOAD_BASE}{path}")
    }

    fn parse<T: serde::de::DeserializeOwned>(&self, bytes: &[u8]) -> Result<T> {
        serde_json::from_slice(bytes).map_err(|e| Error::Other(format!("JSON parse error: {e}")))
    }

    // -- files --------------------------------------------------------------

    /// List files in the user's Drive (optionally filtered by parent folder).
    pub async fn list(
        &self,
        parent_id: Option<&str>,
        page_size: Option<u32>,
        page_token: Option<&str>,
        order_by: Option<&str>,
    ) -> Result<(Vec<serde_json::Value>, Option<String>)> {
        let mut url = format!("{}?fields={}", self.url("/files"), urlencoding(LIST_FIELDS));
        if let Some(pid) = parent_id {
            let q = format!("'{}' in parents and trashed = false", pid);
            url.push_str(&format!("&q={}", urlencoding(&q)));
        } else {
            url.push_str(&format!("&q={}", urlencoding("trashed = false")));
        }
        if let Some(n) = page_size {
            url.push_str(&format!("&pageSize={n}"));
        }
        if let Some(pt) = page_token {
            url.push_str(&format!("&pageToken={}", urlencoding(pt)));
        }
        if let Some(ob) = order_by {
            url.push_str(&format!("&orderBy={}", urlencoding(ob)));
        }
        let resp = self.http.get(&url, &self.token).await?;
        let list: ListFilesResponse = self.parse(&resp)?;
        let values: Vec<serde_json::Value> = list
            .files
            .into_iter()
            .map(|f| serde_json::to_value(f).unwrap_or_default())
            .collect();
        Ok((values, list.next_page_token))
    }

    /// Search files by query string (Drive query syntax).
    pub async fn search(
        &self,
        query: &str,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<(Vec<serde_json::Value>, Option<String>)> {
        let mut url = format!(
            "{}?q={}&fields={}",
            self.url("/files"),
            urlencoding(query),
            urlencoding(LIST_FIELDS),
        );
        if let Some(n) = page_size {
            url.push_str(&format!("&pageSize={n}"));
        }
        if let Some(pt) = page_token {
            url.push_str(&format!("&pageToken={}", urlencoding(pt)));
        }
        let resp = self.http.get(&url, &self.token).await?;
        let list: ListFilesResponse = self.parse(&resp)?;
        let values: Vec<serde_json::Value> = list
            .files
            .into_iter()
            .map(|f| serde_json::to_value(f).unwrap_or_default())
            .collect();
        Ok((values, list.next_page_token))
    }

    /// Get file metadata by ID.
    pub async fn get(&self, file_id: &str) -> Result<serde_json::Value> {
        let url = format!(
            "{}?fields={}",
            self.url(&format!("/files/{file_id}")),
            urlencoding(FILE_FIELDS),
        );
        let resp = self.http.get(&url, &self.token).await?;
        let file: File = self.parse(&resp)?;
        serde_json::to_value(file).map_err(|e| Error::Other(format!("JSON serialize error: {e}")))
    }

    /// Upload a file to Drive.
    ///
    /// Uses multipart upload to send metadata and content in a single request.
    pub async fn upload(
        &self,
        name: &str,
        parent_id: Option<&str>,
        mime_type: &str,
        content: &[u8],
    ) -> Result<serde_json::Value> {
        let boundary = uuid::Uuid::new_v4().to_string();

        // Build metadata JSON
        let mut metadata = serde_json::json!({
            "name": name,
            "mimeType": mime_type,
        });
        if let Some(pid) = parent_id {
            metadata["parents"] = serde_json::json!([pid]);
        }
        let metadata_json = serde_json::to_string(&metadata)
            .map_err(|e| Error::Other(e.to_string()))?;

        // Build multipart/related body
        let mut body = Vec::new();
        body.extend_from_slice(format!("--{boundary}\r\n").as_bytes());
        body.extend_from_slice(b"Content-Type: application/json; charset=UTF-8\r\n\r\n");
        body.extend_from_slice(metadata_json.as_bytes());
        body.extend_from_slice(format!("\r\n--{boundary}\r\n").as_bytes());
        body.extend_from_slice(format!("Content-Type: {mime_type}\r\n\r\n").as_bytes());
        body.extend_from_slice(content);
        body.extend_from_slice(format!("\r\n--{boundary}--").as_bytes());

        let url = format!(
            "{}?uploadType=multipart&fields={}",
            self.upload_url("/files"),
            urlencoding(FILE_FIELDS),
        );
        let content_type = format!("multipart/related; boundary={boundary}");
        let resp = self
            .http
            .post_multipart(&url, &self.token, &body, &content_type)
            .await?;
        let file: File = self.parse(&resp)?;
        serde_json::to_value(file).map_err(|e| Error::Other(format!("JSON serialize error: {e}")))
    }

    /// Download file content by ID.
    pub async fn download(&self, file_id: &str) -> Result<Vec<u8>> {
        let url = self.url(&format!("/files/{file_id}?alt=media"));
        self.http.get(&url, &self.token).await
    }

    /// Copy a file.
    pub async fn copy(
        &self,
        file_id: &str,
        new_name: Option<&str>,
    ) -> Result<serde_json::Value> {
        let mut metadata = serde_json::json!({});
        if let Some(name) = new_name {
            metadata["name"] = serde_json::Value::String(name.to_string());
        }
        let body = serde_json::to_vec(&metadata)
            .map_err(|e| Error::Other(e.to_string()))?;
        let url = format!(
            "{}?fields={}",
            self.url(&format!("/files/{file_id}/copy")),
            urlencoding(FILE_FIELDS),
        );
        let resp = self.http.post(&url, &self.token, &body).await?;
        let file: File = self.parse(&resp)?;
        serde_json::to_value(file).map_err(|e| Error::Other(format!("JSON serialize error: {e}")))
    }

    /// Create a folder.
    pub async fn mkdir(
        &self,
        name: &str,
        parent_id: Option<&str>,
    ) -> Result<serde_json::Value> {
        let mut metadata = serde_json::json!({
            "name": name,
            "mimeType": "application/vnd.google-apps.folder",
        });
        if let Some(pid) = parent_id {
            metadata["parents"] = serde_json::json!([pid]);
        }
        let body = serde_json::to_vec(&metadata)
            .map_err(|e| Error::Other(e.to_string()))?;
        let url = format!(
            "{}?fields={}",
            self.url("/files"),
            urlencoding(FILE_FIELDS),
        );
        let resp = self.http.post(&url, &self.token, &body).await?;
        let file: File = self.parse(&resp)?;
        serde_json::to_value(file).map_err(|e| Error::Other(format!("JSON serialize error: {e}")))
    }

    /// Rename a file or folder.
    pub async fn rename(
        &self,
        file_id: &str,
        new_name: &str,
    ) -> Result<serde_json::Value> {
        let body = serde_json::to_vec(&serde_json::json!({ "name": new_name }))
            .map_err(|e| Error::Other(e.to_string()))?;
        let url = format!(
            "{}?fields={}",
            self.url(&format!("/files/{file_id}")),
            urlencoding(FILE_FIELDS),
        );
        let resp = self.http.patch(&url, &self.token, &body).await?;
        let file: File = self.parse(&resp)?;
        serde_json::to_value(file).map_err(|e| Error::Other(format!("JSON serialize error: {e}")))
    }

    /// Move a file to a different parent folder.
    pub async fn move_file(
        &self,
        file_id: &str,
        new_parent_id: &str,
        old_parent_id: Option<&str>,
    ) -> Result<serde_json::Value> {
        let mut url = format!(
            "{}?addParents={}&fields={}",
            self.url(&format!("/files/{file_id}")),
            urlencoding(new_parent_id),
            urlencoding(FILE_FIELDS),
        );
        if let Some(old_pid) = old_parent_id {
            url.push_str(&format!("&removeParents={}", urlencoding(old_pid)));
        }
        let resp = self.http.patch(&url, &self.token, &[]).await?;
        let file: File = self.parse(&resp)?;
        serde_json::to_value(file).map_err(|e| Error::Other(format!("JSON serialize error: {e}")))
    }

    /// Move a file to Trash.
    ///
    /// In Drive API v3, trashing is done via PATCH with `trashed: true`.
    pub async fn trash(&self, file_id: &str) -> Result<serde_json::Value> {
        let body = serde_json::to_vec(&serde_json::json!({ "trashed": true }))
            .map_err(|e| Error::Other(e.to_string()))?;
        let url = format!(
            "{}?fields={}",
            self.url(&format!("/files/{file_id}")),
            urlencoding(FILE_FIELDS),
        );
        let resp = self.http.patch(&url, &self.token, &body).await?;
        let file: File = self.parse(&resp)?;
        serde_json::to_value(file).map_err(|e| Error::Other(format!("JSON serialize error: {e}")))
    }

    /// Permanently delete a file (bypasses Trash).
    #[cfg(feature = "destructive-permanent")]
    pub async fn permanent_delete(&self, file_id: &str) -> Result<()> {
        let url = self.url(&format!("/files/{file_id}"));
        self.http.delete(&url, &self.token).await?;
        Ok(())
    }

    /// Permanently empty the Trash.
    #[cfg(feature = "destructive-permanent")]
    pub async fn empty_trash(&self) -> Result<()> {
        let url = self.url("/files/trash");
        self.http.delete(&url, &self.token).await?;
        Ok(())
    }

    /// Share a file with a user or group.
    pub async fn share(
        &self,
        file_id: &str,
        email: &str,
        role: &str,
        send_notification: bool,
    ) -> Result<serde_json::Value> {
        let body = serde_json::to_vec(&serde_json::json!({
            "type": "user",
            "role": role,
            "emailAddress": email,
        }))
        .map_err(|e| Error::Other(e.to_string()))?;
        let url = format!(
            "{}?sendNotificationEmail={}",
            self.url(&format!("/files/{file_id}/permissions")),
            send_notification,
        );
        let resp = self.http.post(&url, &self.token, &body).await?;
        let perm: Permission = self.parse(&resp)?;
        serde_json::to_value(perm)
            .map_err(|e| Error::Other(format!("JSON serialize error: {e}")))
    }

    /// Remove sharing permission.
    pub async fn unshare(&self, file_id: &str, permission_id: &str) -> Result<()> {
        let url = self.url(&format!("/files/{file_id}/permissions/{permission_id}"));
        self.http.delete(&url, &self.token).await?;
        Ok(())
    }

    /// List permissions on a file.
    pub async fn permissions(&self, file_id: &str) -> Result<Vec<serde_json::Value>> {
        let url = format!(
            "{}?fields={}",
            self.url(&format!("/files/{file_id}/permissions")),
            urlencoding("permissions(id,type,role,emailAddress,displayName,domain,expirationTime,deleted)"),
        );
        let resp = self.http.get(&url, &self.token).await?;
        let list: ListPermissionsResponse = self.parse(&resp)?;
        let values: Vec<serde_json::Value> = list
            .permissions
            .into_iter()
            .map(|p| serde_json::to_value(p).unwrap_or_default())
            .collect();
        Ok(values)
    }

    /// List shared drives the user has access to.
    pub async fn drives(
        &self,
        page_token: Option<&str>,
    ) -> Result<(Vec<serde_json::Value>, Option<String>)> {
        let mut url = self.url("/drives");
        let mut sep = '?';
        if let Some(pt) = page_token {
            url.push_str(&format!("{sep}pageToken={}", urlencoding(pt)));
            sep = '&';
        }
        let _ = sep; // suppress unused warning
        let resp = self.http.get(&url, &self.token).await?;
        let list: ListDrivesResponse = self.parse(&resp)?;
        let values: Vec<serde_json::Value> = list
            .drives
            .into_iter()
            .map(|d| serde_json::to_value(d).unwrap_or_default())
            .collect();
        Ok((values, list.next_page_token))
    }

    /// Batch-trash multiple files (with bulk check).
    pub async fn batch_trash(&self, file_ids: &[String]) -> Result<Vec<serde_json::Value>> {
        crate::destructive::check_bulk_trash(file_ids.len())?;
        let mut results = Vec::with_capacity(file_ids.len());
        for file_id in file_ids {
            let result = self.trash(file_id).await?;
            results.push(result);
        }
        Ok(results)
    }

    // -- changes (used by Indexable) ----------------------------------------

    /// Get a start page token for tracking changes.
    async fn get_start_page_token(&self) -> Result<String> {
        let url = self.url("/changes/startPageToken");
        let resp = self.http.get(&url, &self.token).await?;
        let token_resp: StartPageTokenResponse = self.parse(&resp)?;
        token_resp
            .start_page_token
            .ok_or_else(|| Error::Other("missing startPageToken".into()))
    }

    /// List changes since a page token.
    async fn list_changes(
        &self,
        page_token: &str,
        page_size: Option<u32>,
    ) -> Result<(Vec<Change>, Option<String>, Option<String>)> {
        let mut url = format!(
            "{}?pageToken={}&fields={}",
            self.url("/changes"),
            urlencoding(page_token),
            urlencoding("nextPageToken,newStartPageToken,changes(kind,type,time,removed,fileId,file(id,name,mimeType,modifiedTime,owners,parents,webViewLink,size,trashed))"),
        );
        if let Some(n) = page_size {
            url.push_str(&format!("&pageSize={n}"));
        }
        // Include items from shared drives
        url.push_str("&includeItemsFromAllDrives=true&supportsAllDrives=true");
        let resp = self.http.get(&url, &self.token).await?;
        let list: ListChangesResponse = self.parse(&resp)?;
        Ok((list.changes, list.next_page_token, list.new_start_page_token))
    }
}

// ---------------------------------------------------------------------------
// Indexable
// ---------------------------------------------------------------------------

impl Indexable for DriveService {
    type Document = DriveIndexDocument;

    async fn fetch_indexable(
        &self,
        since: Option<&str>,
        limit: usize,
    ) -> Result<(Vec<DriveIndexDocument>, Option<String>)> {
        // Get the page token: use provided cursor or fetch a fresh start token
        let page_token = if let Some(cursor) = since {
            cursor.to_string()
        } else {
            self.get_start_page_token().await?
        };

        // If this is an initial sync (no cursor), list recent files instead of
        // changes, since there won't be any changes from the start token.
        if since.is_none() {
            let (files, _) = self
                .search(
                    "trashed = false",
                    Some(limit as u32),
                    None,
                )
                .await?;
            let docs: Vec<DriveIndexDocument> = files
                .into_iter()
                .filter_map(|v| file_value_to_index_doc(&v))
                .collect();
            return Ok((docs, Some(page_token)));
        }

        // Incremental sync via changes API
        let (changes, next_page_token, new_start_page_token) =
            self.list_changes(&page_token, Some(limit as u32)).await?;

        let docs: Vec<DriveIndexDocument> = changes
            .into_iter()
            .filter(|c| !c.removed)
            .filter_map(|c| {
                c.file.as_ref().and_then(|f| file_to_index_doc(f))
            })
            .collect();

        // Prefer the new start page token (indicates all pages consumed),
        // otherwise use nextPageToken to continue paging.
        let cursor = new_start_page_token.or(next_page_token);
        Ok((docs, cursor))
    }

    fn index_namespace(&self) -> &'static str {
        "drive"
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn urlencoding(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}

/// Convert a `File` struct to a `DriveIndexDocument`.
fn file_to_index_doc(file: &File) -> Option<DriveIndexDocument> {
    let file_id = file.id.as_ref()?.clone();
    let name = file.name.clone().unwrap_or_default();
    let mime_type = file.mime_type.clone().unwrap_or_default();

    // Skip trashed files
    if file.trashed {
        return None;
    }

    Some(DriveIndexDocument {
        file_id,
        name,
        mime_type,
        modified_time: file.modified_time.clone(),
        owners: file
            .owners
            .iter()
            .filter_map(|u| u.email_address.clone())
            .collect(),
        parents: file.parents.clone(),
        web_view_link: file.web_view_link.clone(),
        size: file.size.as_ref().and_then(|s| s.parse().ok()),
    })
}

/// Convert a `serde_json::Value` (from search results) to a `DriveIndexDocument`.
fn file_value_to_index_doc(val: &serde_json::Value) -> Option<DriveIndexDocument> {
    let file: File = serde_json::from_value(val.clone()).ok()?;
    file_to_index_doc(&file)
}
