//! Google Docs API service client.
//!
//! Wraps the Docs REST API v1.
//! <https://developers.google.com/docs/api/reference/rest>

use crate::error::{Error, Result};
use crate::http::HttpClient;
use crate::indexable::Indexable;
use crate::types::docs::*;
use serde::Deserialize;
use serde::Serialize;

const BASE: &str = "https://docs.googleapis.com/v1";
const DRIVE_BASE: &str = "https://www.googleapis.com/drive/v3";

// ---------------------------------------------------------------------------
// API response wrappers (not part of the public types)
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DriveFileListResponse {
    #[serde(default)]
    files: Vec<DriveFileMetadata>,
    #[serde(default)]
    next_page_token: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DriveFileMetadata {
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    modified_time: Option<String>,
}

// ---------------------------------------------------------------------------
// Index document
// ---------------------------------------------------------------------------

/// Document yielded by the Docs indexing implementation.
#[derive(Debug, Clone, Serialize)]
pub struct DocsIndexDocument {
    pub document_id: String,
    pub title: String,
    pub body_text: String,
    pub modified_time: Option<String>,
    pub revision_id: Option<String>,
}

// ---------------------------------------------------------------------------
// Service
// ---------------------------------------------------------------------------

/// Async client for the Google Docs API.
#[derive(Debug, Clone)]
pub struct DocsService {
    http: HttpClient,
    token: String,
}

impl DocsService {
    /// Create a new `DocsService`.
    pub fn new(http: HttpClient, token: String) -> Self {
        Self { http, token }
    }

    fn url(&self, path: &str) -> String {
        format!("{BASE}{path}")
    }

    fn parse<T: serde::de::DeserializeOwned>(&self, bytes: &[u8]) -> Result<T> {
        serde_json::from_slice(bytes).map_err(|e| Error::Other(format!("JSON parse error: {e}")))
    }

    // -- documents ----------------------------------------------------------

    /// Get document metadata and full structure as a typed `Document`.
    pub async fn get(&self, document_id: &str) -> Result<Document> {
        let url = self.url(&format!("/documents/{document_id}"));
        let resp = self.http.get(&url, &self.token).await?;
        self.parse(&resp)
    }

    /// Get document metadata and structure as raw JSON.
    pub async fn info(&self, document_id: &str) -> Result<serde_json::Value> {
        let url = self.url(&format!("/documents/{document_id}"));
        let resp = self.http.get(&url, &self.token).await?;
        self.parse(&resp)
    }

    /// Extract the plain-text content of a document (concatenated body text).
    pub async fn cat(&self, document_id: &str) -> Result<String> {
        let doc = self.get(document_id).await?;
        Ok(extract_document_text(&doc))
    }

    /// Create a new blank document with a title.
    pub async fn create(&self, title: &str) -> Result<Document> {
        let body = serde_json::to_vec(&serde_json::json!({
            "title": title
        }))
        .map_err(|e| Error::Other(e.to_string()))?;
        let url = self.url("/documents");
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Copy a document (via Drive copy endpoint).
    pub async fn copy(
        &self,
        document_id: &str,
        new_title: Option<&str>,
    ) -> Result<serde_json::Value> {
        let mut obj = serde_json::json!({});
        if let Some(title) = new_title {
            obj["name"] = serde_json::Value::String(title.to_string());
        }
        let body =
            serde_json::to_vec(&obj).map_err(|e| Error::Other(e.to_string()))?;
        let url = format!("{DRIVE_BASE}/files/{document_id}/copy");
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Export a document in the given MIME type (e.g. `text/plain`, `application/pdf`).
    ///
    /// Uses the Drive export endpoint because the Docs API does not have a
    /// dedicated export method.
    pub async fn export(&self, document_id: &str, mime_type: &str) -> Result<Vec<u8>> {
        let encoded_mime = urlencoding(mime_type);
        let url = format!(
            "{DRIVE_BASE}/files/{document_id}/export?mimeType={encoded_mime}"
        );
        self.http.get(&url, &self.token).await
    }

    /// Insert text into a document at the given index.
    ///
    /// If `index` is `None`, text is inserted at index 1 (the beginning of
    /// the document body -- the first valid insertion point in the Docs API).
    pub async fn write(
        &self,
        document_id: &str,
        text: &str,
        index: Option<i64>,
    ) -> Result<serde_json::Value> {
        let insert_index = index.unwrap_or(1);
        let body = serde_json::to_vec(&serde_json::json!({
            "requests": [
                {
                    "insertText": {
                        "location": {
                            "index": insert_index
                        },
                        "text": text
                    }
                }
            ]
        }))
        .map_err(|e| Error::Other(e.to_string()))?;
        let url = self.url(&format!("/documents/{document_id}:batchUpdate"));
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Find and replace text in a document.
    ///
    /// Returns the batch update response which includes the number of
    /// occurrences replaced in the `replies` array.
    pub async fn find_replace(
        &self,
        document_id: &str,
        find: &str,
        replace: &str,
        match_case: bool,
    ) -> Result<serde_json::Value> {
        let body = serde_json::to_vec(&serde_json::json!({
            "requests": [
                {
                    "replaceAllText": {
                        "containsText": {
                            "text": find,
                            "matchCase": match_case
                        },
                        "replaceText": replace
                    }
                }
            ]
        }))
        .map_err(|e| Error::Other(e.to_string()))?;
        let url = self.url(&format!("/documents/{document_id}:batchUpdate"));
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Regex-based find and replace (sed-style).
    ///
    /// The Docs API `replaceAllText` does not support regex natively. This
    /// method fetches the document, performs a string-based find/replace on
    /// the plain text representation, then clears the document body and
    /// re-inserts the modified text via `batchUpdate`. The `pattern` is
    /// treated as a literal substring match. For true regex support, add the
    /// `regex` crate as a dependency.
    pub async fn sed(
        &self,
        document_id: &str,
        pattern: &str,
        replacement: &str,
    ) -> Result<serde_json::Value> {
        let doc = self.get(document_id).await?;
        let original = extract_document_text(&doc);

        let replaced = original.replace(pattern, replacement);

        if replaced == original {
            return Ok(serde_json::json!({
                "documentId": document_id,
                "replies": [],
                "matchCount": 0
            }));
        }

        // Delete all existing body text then re-insert.
        // Docs body indices are 1-based; the trailing newline occupies the
        // last index, so the deletable range is [1, body_len].
        let body_len = original.len() as i64;
        let mut requests = Vec::new();

        if body_len > 0 {
            requests.push(serde_json::json!({
                "deleteContentRange": {
                    "range": {
                        "startIndex": 1,
                        "endIndex": body_len + 1
                    }
                }
            }));
        }

        requests.push(serde_json::json!({
            "insertText": {
                "location": {
                    "index": 1
                },
                "text": replaced
            }
        }));

        let body =
            serde_json::to_vec(&serde_json::json!({ "requests": requests }))
                .map_err(|e| Error::Other(e.to_string()))?;
        let url = self.url(&format!("/documents/{document_id}:batchUpdate"));
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// List tabs in a document (Docs tab feature).
    pub async fn list_tabs(&self, document_id: &str) -> Result<Vec<serde_json::Value>> {
        let doc = self.get(document_id).await?;
        let tabs: Vec<serde_json::Value> = doc
            .tabs
            .iter()
            .map(|tab| {
                serde_json::json!({
                    "tabId": tab.tab_properties.as_ref().and_then(|p| p.tab_id.clone()),
                    "title": tab.tab_properties.as_ref().and_then(|p| p.title.clone()),
                    "parentTabId": tab.tab_properties.as_ref().and_then(|p| p.parent_tab_id.clone()),
                    "index": tab.tab_properties.as_ref().and_then(|p| p.index),
                    "childTabCount": tab.child_tabs.len(),
                })
            })
            .collect();
        Ok(tabs)
    }
}

// ---------------------------------------------------------------------------
// Indexable
// ---------------------------------------------------------------------------

impl Indexable for DocsService {
    type Document = DocsIndexDocument;

    async fn fetch_indexable(
        &self,
        since: Option<&str>,
        limit: usize,
    ) -> Result<(Vec<DocsIndexDocument>, Option<String>)> {
        // Use Drive API to find Google Docs files, optionally filtered by
        // modifiedTime. Then fetch each document via the Docs API to extract
        // body text.
        let mime_filter = "mimeType='application/vnd.google-apps.document'";
        let query = if let Some(cursor) = since {
            format!("{mime_filter} and modifiedTime > '{cursor}'")
        } else {
            mime_filter.to_string()
        };

        let encoded_query = urlencoding(&query);
        let url = format!(
            "{DRIVE_BASE}/files?q={encoded_query}&pageSize={limit}\
             &orderBy=modifiedTime%20desc\
             &fields=files(id,name,modifiedTime),nextPageToken"
        );

        let resp = self.http.get(&url, &self.token).await?;
        let list: DriveFileListResponse = self.parse(&resp)?;

        let mut docs = Vec::with_capacity(list.files.len());
        let mut latest_modified: Option<String> = None;

        for file in &list.files {
            let Some(file_id) = &file.id else {
                continue;
            };

            // Track the most recent modifiedTime as the new cursor
            if let Some(mt) = &file.modified_time {
                if latest_modified
                    .as_deref()
                    .map_or(true, |prev| mt.as_str() > prev)
                {
                    latest_modified = Some(mt.clone());
                }
            }

            // Fetch the full document to extract text
            if let Ok(doc) = self.get(file_id).await {
                let body_text = extract_document_text(&doc);
                docs.push(DocsIndexDocument {
                    document_id: file_id.clone(),
                    title: doc
                        .title
                        .clone()
                        .or_else(|| file.name.clone())
                        .unwrap_or_default(),
                    body_text,
                    modified_time: file.modified_time.clone(),
                    revision_id: doc.revision_id.clone(),
                });
            }
        }

        Ok((docs, latest_modified))
    }

    fn index_namespace(&self) -> &'static str {
        "docs"
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn urlencoding(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}

/// Extract plain text from a `Document` by walking the body's structural
/// elements and concatenating all `TextRun` content.
fn extract_document_text(doc: &Document) -> String {
    let mut text = String::new();
    if let Some(body) = &doc.body {
        extract_body_text(body, &mut text);
    }
    text
}

/// Walk a `Body` and append all text-run content to the buffer.
fn extract_body_text(body: &Body, out: &mut String) {
    for element in &body.content {
        extract_structural_element(element, out);
    }
}

/// Recursively extract text from a single structural element.
fn extract_structural_element(element: &StructuralElement, out: &mut String) {
    if let Some(paragraph) = &element.paragraph {
        for pe in &paragraph.elements {
            if let Some(text_run) = &pe.text_run {
                if let Some(content) = &text_run.content {
                    out.push_str(content);
                }
            }
        }
    }
    if let Some(table) = &element.table {
        // Table rows contain cells which contain structural elements.
        // Walk the JSON value to extract text from nested paragraphs.
        for row_val in &table.table_rows {
            if let Some(cells) = row_val.get("tableCells").and_then(|c| c.as_array()) {
                for cell in cells {
                    if let Some(content) = cell.get("content").and_then(|c| c.as_array()) {
                        for elem_val in content {
                            if let Ok(se) =
                                serde_json::from_value::<StructuralElement>(elem_val.clone())
                            {
                                extract_structural_element(&se, out);
                            }
                        }
                    }
                }
            }
        }
    }
    if let Some(toc) = &element.table_of_contents {
        for child in &toc.content {
            extract_structural_element(child, out);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_text_from_empty_document() {
        let doc = Document {
            document_id: Some("test".into()),
            title: Some("Test".into()),
            body: Some(Body { content: vec![] }),
            headers: None,
            footers: None,
            footnotes: None,
            document_style: None,
            named_styles: None,
            named_ranges: None,
            lists: None,
            inline_objects: None,
            positioned_objects: None,
            revision_id: None,
            suggestions_view_mode: None,
            tabs: vec![],
        };
        assert_eq!(extract_document_text(&doc), "");
    }

    #[test]
    fn extract_text_from_paragraph() {
        let doc = Document {
            document_id: Some("test".into()),
            title: Some("Test".into()),
            body: Some(Body {
                content: vec![StructuralElement {
                    start_index: Some(0),
                    end_index: Some(13),
                    paragraph: Some(Paragraph {
                        elements: vec![ParagraphElement {
                            start_index: Some(0),
                            end_index: Some(13),
                            text_run: Some(TextRun {
                                content: Some("Hello, world!".into()),
                                text_style: None,
                                suggested_insertion_ids: vec![],
                                suggested_deletion_ids: vec![],
                                suggested_text_style_changes: None,
                            }),
                            auto_text: None,
                            page_break: None,
                            column_break: None,
                            footnote_reference: None,
                            horizontal_rule: None,
                            inline_object_element: None,
                            person: None,
                            rich_link: None,
                        }],
                        paragraph_style: None,
                        bullet: None,
                        suggested_paragraph_style_changes: None,
                        suggested_bullet_changes: None,
                    }),
                    section_break: None,
                    table: None,
                    table_of_contents: None,
                }],
            }),
            headers: None,
            footers: None,
            footnotes: None,
            document_style: None,
            named_styles: None,
            named_ranges: None,
            lists: None,
            inline_objects: None,
            positioned_objects: None,
            revision_id: None,
            suggestions_view_mode: None,
            tabs: vec![],
        };
        assert_eq!(extract_document_text(&doc), "Hello, world!");
    }

    #[test]
    fn extract_text_skips_non_text_elements() {
        let doc = Document {
            document_id: Some("test".into()),
            title: Some("Test".into()),
            body: Some(Body {
                content: vec![
                    StructuralElement {
                        start_index: Some(0),
                        end_index: Some(1),
                        paragraph: None,
                        section_break: Some(SectionBreak {
                            section_style: None,
                        }),
                        table: None,
                        table_of_contents: None,
                    },
                    StructuralElement {
                        start_index: Some(1),
                        end_index: Some(6),
                        paragraph: Some(Paragraph {
                            elements: vec![ParagraphElement {
                                start_index: Some(1),
                                end_index: Some(6),
                                text_run: Some(TextRun {
                                    content: Some("text\n".into()),
                                    text_style: None,
                                    suggested_insertion_ids: vec![],
                                    suggested_deletion_ids: vec![],
                                    suggested_text_style_changes: None,
                                }),
                                auto_text: None,
                                page_break: None,
                                column_break: None,
                                footnote_reference: None,
                                horizontal_rule: None,
                                inline_object_element: None,
                                person: None,
                                rich_link: None,
                            }],
                            paragraph_style: None,
                            bullet: None,
                            suggested_paragraph_style_changes: None,
                            suggested_bullet_changes: None,
                        }),
                        section_break: None,
                        table: None,
                        table_of_contents: None,
                    },
                ],
            }),
            headers: None,
            footers: None,
            footnotes: None,
            document_style: None,
            named_styles: None,
            named_ranges: None,
            lists: None,
            inline_objects: None,
            positioned_objects: None,
            revision_id: None,
            suggestions_view_mode: None,
            tabs: vec![],
        };
        assert_eq!(extract_document_text(&doc), "text\n");
    }

    #[test]
    fn extract_text_no_body() {
        let doc = Document {
            document_id: Some("test".into()),
            title: Some("Test".into()),
            body: None,
            headers: None,
            footers: None,
            footnotes: None,
            document_style: None,
            named_styles: None,
            named_ranges: None,
            lists: None,
            inline_objects: None,
            positioned_objects: None,
            revision_id: None,
            suggestions_view_mode: None,
            tabs: vec![],
        };
        assert_eq!(extract_document_text(&doc), "");
    }

    #[test]
    fn extract_text_multiple_paragraphs() {
        let make_para = |text: &str| StructuralElement {
            start_index: None,
            end_index: None,
            paragraph: Some(Paragraph {
                elements: vec![ParagraphElement {
                    start_index: None,
                    end_index: None,
                    text_run: Some(TextRun {
                        content: Some(text.to_string()),
                        text_style: None,
                        suggested_insertion_ids: vec![],
                        suggested_deletion_ids: vec![],
                        suggested_text_style_changes: None,
                    }),
                    auto_text: None,
                    page_break: None,
                    column_break: None,
                    footnote_reference: None,
                    horizontal_rule: None,
                    inline_object_element: None,
                    person: None,
                    rich_link: None,
                }],
                paragraph_style: None,
                bullet: None,
                suggested_paragraph_style_changes: None,
                suggested_bullet_changes: None,
            }),
            section_break: None,
            table: None,
            table_of_contents: None,
        };

        let doc = Document {
            document_id: Some("test".into()),
            title: Some("Test".into()),
            body: Some(Body {
                content: vec![
                    make_para("First paragraph\n"),
                    make_para("Second paragraph\n"),
                ],
            }),
            headers: None,
            footers: None,
            footnotes: None,
            document_style: None,
            named_styles: None,
            named_ranges: None,
            lists: None,
            inline_objects: None,
            positioned_objects: None,
            revision_id: None,
            suggestions_view_mode: None,
            tabs: vec![],
        };
        assert_eq!(
            extract_document_text(&doc),
            "First paragraph\nSecond paragraph\n"
        );
    }

    #[test]
    fn index_namespace_is_docs() {
        let client = HttpClient::new().unwrap();
        let svc = DocsService::new(client, "token".into());
        assert_eq!(svc.index_namespace(), "docs");
    }
}
