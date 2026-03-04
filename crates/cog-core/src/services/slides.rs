//! Google Slides API service client.
//!
//! Wraps the Slides REST API v1.
//! <https://developers.google.com/slides/api/reference/rest>

use crate::error::{Error, Result};
use crate::http::HttpClient;
use crate::types::slides::*;
use serde::Deserialize;

const BASE: &str = "https://slides.googleapis.com/v1";

// ---------------------------------------------------------------------------
// API response wrappers (not part of the public types)
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct BatchUpdateResponse {
    #[serde(default)]
    presentation_id: Option<String>,
    #[serde(default)]
    replies: Vec<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Service
// ---------------------------------------------------------------------------

/// Async client for the Google Slides API.
#[derive(Debug, Clone)]
pub struct SlidesService {
    http: HttpClient,
    token: String,
}

impl SlidesService {
    /// Create a new `SlidesService`.
    pub fn new(http: HttpClient, token: String) -> Self {
        Self { http, token }
    }

    fn url(&self, path: &str) -> String {
        format!("{BASE}{path}")
    }

    fn parse<T: serde::de::DeserializeOwned>(&self, bytes: &[u8]) -> Result<T> {
        serde_json::from_slice(bytes).map_err(|e| Error::Other(format!("JSON parse error: {e}")))
    }

    // -- presentation -------------------------------------------------------

    /// Get presentation metadata and structure.
    pub async fn get(&self, presentation_id: &str) -> Result<Presentation> {
        let url = self.url(&format!("/presentations/{presentation_id}"));
        let resp = self.http.get(&url, &self.token).await?;
        self.parse(&resp)
    }

    /// Get presentation metadata and structure.
    /// Alias for backwards compatibility.
    pub async fn info(&self, presentation_id: &str) -> Result<Presentation> {
        self.get(presentation_id).await
    }

    /// Create a new blank presentation with a title.
    pub async fn create(&self, title: &str) -> Result<Presentation> {
        let body = serde_json::to_vec(&serde_json::json!({
            "title": title
        }))
        .map_err(|e| Error::Other(e.to_string()))?;
        let url = self.url("/presentations");
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Create a presentation from Markdown content.
    /// Creates a blank presentation, then inserts text content via batchUpdate.
    pub async fn create_from_markdown(
        &self,
        title: &str,
        markdown: &str,
    ) -> Result<Presentation> {
        // First create a blank presentation
        let pres = self.create(title).await?;
        let pres_id = pres
            .presentation_id
            .as_deref()
            .ok_or_else(|| Error::Other("no presentation_id returned".into()))?;

        // Get the first slide's body placeholder to insert text
        if let Some(slide) = pres.slides.first() {
            if let Some(obj_id) = &slide.object_id {
                // Find a body placeholder to insert text into
                let placeholder_id = slide
                    .page_elements
                    .iter()
                    .find_map(|el| {
                        if let Some(shape) = &el.shape {
                            if let Some(ph) = &shape.placeholder {
                                if ph.placeholder_type.as_deref() == Some("BODY")
                                    || ph.placeholder_type.as_deref() == Some("SUBTITLE")
                                {
                                    return el.object_id.clone();
                                }
                            }
                        }
                        None
                    })
                    .unwrap_or_else(|| obj_id.clone());

                let body = serde_json::to_vec(&serde_json::json!({
                    "requests": [{
                        "insertText": {
                            "objectId": placeholder_id,
                            "text": markdown
                        }
                    }]
                }))
                .map_err(|e| Error::Other(e.to_string()))?;
                let url = self.url(&format!("/presentations/{}:batchUpdate", pres_id));
                self.http.post(&url, &self.token, &body).await?;
            }
        }

        // Return the updated presentation
        self.get(pres_id).await
    }

    /// Copy a presentation (via Drive copy endpoint).
    pub async fn copy(
        &self,
        presentation_id: &str,
        new_title: Option<&str>,
    ) -> Result<serde_json::Value> {
        let mut obj = serde_json::json!({});
        if let Some(title) = new_title {
            obj["name"] = serde_json::Value::String(title.to_string());
        }
        let body = serde_json::to_vec(&obj).map_err(|e| Error::Other(e.to_string()))?;
        let url = format!(
            "https://www.googleapis.com/drive/v3/files/{}:copy",
            presentation_id
        );
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Export the presentation in a given MIME type (e.g. `application/pdf`).
    /// Uses the Drive export endpoint.
    pub async fn export(&self, presentation_id: &str, mime_type: &str) -> Result<Vec<u8>> {
        let url = format!(
            "https://www.googleapis.com/drive/v3/files/{}/export?mimeType={}",
            presentation_id,
            urlencoding(mime_type)
        );
        self.http.get(&url, &self.token).await
    }

    /// List all slides in a presentation (page object IDs and titles).
    pub async fn list_slides(&self, presentation_id: &str) -> Result<Vec<Slide>> {
        let pres = self.get(presentation_id).await?;
        Ok(pres.slides)
    }

    /// Add a blank slide to a presentation.
    pub async fn add_slide(
        &self,
        presentation_id: &str,
        insertion_index: Option<i32>,
        layout: Option<&str>,
    ) -> Result<serde_json::Value> {
        let mut create_slide = serde_json::json!({});
        if let Some(idx) = insertion_index {
            create_slide["insertionIndex"] = serde_json::json!(idx);
        }
        if let Some(layout_id) = layout {
            create_slide["slideLayoutReference"] = serde_json::json!({
                "predefinedLayout": layout_id
            });
        }

        let body = serde_json::to_vec(&serde_json::json!({
            "requests": [{
                "createSlide": create_slide
            }]
        }))
        .map_err(|e| Error::Other(e.to_string()))?;
        let url = self.url(&format!("/presentations/{}:batchUpdate", presentation_id));
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Update the speaker notes of a slide.
    pub async fn update_notes(
        &self,
        presentation_id: &str,
        slide_object_id: &str,
        notes_text: &str,
    ) -> Result<serde_json::Value> {
        // First get the presentation to find the notes page speaker notes object ID
        let pres = self.get(presentation_id).await?;
        let notes_obj_id = pres
            .slides
            .iter()
            .find(|s| s.object_id.as_deref() == Some(slide_object_id))
            .and_then(|s| s.slide_properties.as_ref())
            .and_then(|sp| sp.notes_page.as_ref())
            .and_then(|np| np.notes_properties.as_ref())
            .and_then(|nprops| nprops.speaker_notes_object_id.clone())
            .ok_or_else(|| {
                Error::Other(format!(
                    "could not find speaker notes for slide {slide_object_id}"
                ))
            })?;

        // Clear existing text and insert new notes
        let body = serde_json::to_vec(&serde_json::json!({
            "requests": [
                {
                    "deleteText": {
                        "objectId": notes_obj_id,
                        "textRange": {
                            "type": "ALL"
                        }
                    }
                },
                {
                    "insertText": {
                        "objectId": notes_obj_id,
                        "text": notes_text
                    }
                }
            ]
        }))
        .map_err(|e| Error::Other(e.to_string()))?;
        let url = self.url(&format!("/presentations/{}:batchUpdate", presentation_id));
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Replace a slide's content via batchUpdate requests.
    pub async fn replace_slide(
        &self,
        presentation_id: &str,
        requests: &[serde_json::Value],
    ) -> Result<serde_json::Value> {
        let body = serde_json::to_vec(&serde_json::json!({
            "requests": requests
        }))
        .map_err(|e| Error::Other(e.to_string()))?;
        let url = self.url(&format!("/presentations/{}:batchUpdate", presentation_id));
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Permanently delete a slide from a presentation.
    #[cfg(feature = "destructive-permanent")]
    pub async fn delete_slide(
        &self,
        presentation_id: &str,
        slide_object_id: &str,
    ) -> Result<serde_json::Value> {
        let body = serde_json::to_vec(&serde_json::json!({
            "requests": [{
                "deleteObject": {
                    "objectId": slide_object_id
                }
            }]
        }))
        .map_err(|e| Error::Other(e.to_string()))?;
        let url = self.url(&format!("/presentations/{}:batchUpdate", presentation_id));
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn urlencoding(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}
