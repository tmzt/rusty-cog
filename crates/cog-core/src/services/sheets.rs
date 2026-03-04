//! Google Sheets API service client.
//!
//! Wraps the Sheets REST API v4.
//! <https://developers.google.com/sheets/api/reference/rest>

use crate::error::{Error, Result};
use crate::http::HttpClient;
use crate::types::sheets::*;
use serde::Deserialize;

const BASE: &str = "https://sheets.googleapis.com/v4";

// ---------------------------------------------------------------------------
// API response wrappers (not part of the public types)
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateValuesResponse {
    #[serde(default)]
    spreadsheet_id: Option<String>,
    #[serde(default)]
    updated_range: Option<String>,
    #[serde(default)]
    updated_rows: Option<i64>,
    #[serde(default)]
    updated_columns: Option<i64>,
    #[serde(default)]
    updated_cells: Option<i64>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AppendValuesResponse {
    #[serde(default)]
    spreadsheet_id: Option<String>,
    #[serde(default)]
    table_range: Option<String>,
    #[serde(default)]
    updates: Option<UpdateValuesResponse>,
}

// ---------------------------------------------------------------------------
// Service
// ---------------------------------------------------------------------------

/// Async client for the Google Sheets API.
#[derive(Debug, Clone)]
pub struct SheetsService {
    http: HttpClient,
    token: String,
}

impl SheetsService {
    /// Create a new `SheetsService`.
    pub fn new(http: HttpClient, token: String) -> Self {
        Self { http, token }
    }

    fn url(&self, path: &str) -> String {
        format!("{BASE}{path}")
    }

    fn parse<T: serde::de::DeserializeOwned>(&self, bytes: &[u8]) -> Result<T> {
        serde_json::from_slice(bytes).map_err(|e| Error::Other(format!("JSON parse error: {e}")))
    }

    // -- spreadsheet --------------------------------------------------------

    /// Get spreadsheet metadata (sheets, named ranges, properties).
    pub async fn get(&self, spreadsheet_id: &str) -> Result<Spreadsheet> {
        let url = self.url(&format!("/spreadsheets/{spreadsheet_id}"));
        let resp = self.http.get(&url, &self.token).await?;
        self.parse(&resp)
    }

    /// Create a new spreadsheet with a title.
    pub async fn create(&self, title: &str) -> Result<Spreadsheet> {
        let body = serde_json::to_vec(&serde_json::json!({
            "properties": {
                "title": title
            }
        }))
        .map_err(|e| Error::Other(e.to_string()))?;
        let url = self.url("/spreadsheets");
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    // -- values -------------------------------------------------------------

    /// Read values from a range (e.g. `Sheet1!A1:C10`).
    pub async fn values_get(&self, spreadsheet_id: &str, range: &str) -> Result<ValueRange> {
        let url = self.url(&format!(
            "/spreadsheets/{}/values/{}",
            spreadsheet_id,
            urlencoding(range)
        ));
        let resp = self.http.get(&url, &self.token).await?;
        self.parse(&resp)
    }

    /// Update values in a range.
    pub async fn values_update(
        &self,
        spreadsheet_id: &str,
        range: &str,
        values: &[Vec<serde_json::Value>],
    ) -> Result<serde_json::Value> {
        let body = serde_json::to_vec(&serde_json::json!({
            "range": range,
            "majorDimension": "ROWS",
            "values": values
        }))
        .map_err(|e| Error::Other(e.to_string()))?;
        let url = format!(
            "{}/spreadsheets/{}/values/{}?valueInputOption=USER_ENTERED",
            BASE,
            spreadsheet_id,
            urlencoding(range)
        );
        let resp = self.http.put(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Append rows to a sheet.
    pub async fn values_append(
        &self,
        spreadsheet_id: &str,
        range: &str,
        values: &[Vec<serde_json::Value>],
    ) -> Result<serde_json::Value> {
        let body = serde_json::to_vec(&serde_json::json!({
            "range": range,
            "majorDimension": "ROWS",
            "values": values
        }))
        .map_err(|e| Error::Other(e.to_string()))?;
        let url = format!(
            "{}/spreadsheets/{}/values/{}:append?valueInputOption=USER_ENTERED&insertDataOption=INSERT_ROWS",
            BASE,
            spreadsheet_id,
            urlencoding(range)
        );
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Clear values in a range.
    pub async fn clear(&self, spreadsheet_id: &str, range: &str) -> Result<serde_json::Value> {
        let body = serde_json::to_vec(&serde_json::json!({}))
            .map_err(|e| Error::Other(e.to_string()))?;
        let url = self.url(&format!(
            "/spreadsheets/{}/values/{}:clear",
            spreadsheet_id,
            urlencoding(range)
        ));
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Apply formatting to a range via batchUpdate.
    pub async fn format(
        &self,
        spreadsheet_id: &str,
        requests: &[serde_json::Value],
    ) -> Result<serde_json::Value> {
        let body = serde_json::to_vec(&serde_json::json!({
            "requests": requests
        }))
        .map_err(|e| Error::Other(e.to_string()))?;
        let url = self.url(&format!("/spreadsheets/{}:batchUpdate", spreadsheet_id));
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Insert rows or columns into a sheet.
    pub async fn insert(
        &self,
        spreadsheet_id: &str,
        sheet_id: i64,
        dimension: &str,
        start_index: i64,
        end_index: i64,
    ) -> Result<serde_json::Value> {
        let body = serde_json::to_vec(&serde_json::json!({
            "requests": [{
                "insertDimension": {
                    "range": {
                        "sheetId": sheet_id,
                        "dimension": dimension,
                        "startIndex": start_index,
                        "endIndex": end_index
                    },
                    "inheritFromBefore": false
                }
            }]
        }))
        .map_err(|e| Error::Other(e.to_string()))?;
        let url = self.url(&format!("/spreadsheets/{}:batchUpdate", spreadsheet_id));
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Read or update notes on a range.
    pub async fn notes(
        &self,
        spreadsheet_id: &str,
        range: &str,
        notes: Option<&[Vec<Option<String>>]>,
    ) -> Result<serde_json::Value> {
        if let Some(note_values) = notes {
            // Update notes via batchUpdate with updateCells
            let rows: Vec<serde_json::Value> = note_values
                .iter()
                .map(|row| {
                    let cells: Vec<serde_json::Value> = row
                        .iter()
                        .map(|n| {
                            serde_json::json!({
                                "note": n.as_deref().unwrap_or("")
                            })
                        })
                        .collect();
                    serde_json::json!({ "values": cells })
                })
                .collect();

            let body = serde_json::to_vec(&serde_json::json!({
                "requests": [{
                    "updateCells": {
                        "range": {
                            "sheetId": 0
                        },
                        "rows": rows,
                        "fields": "note"
                    }
                }]
            }))
            .map_err(|e| Error::Other(e.to_string()))?;
            let url = self.url(&format!("/spreadsheets/{}:batchUpdate", spreadsheet_id));
            let resp = self.http.post(&url, &self.token, &body).await?;
            self.parse(&resp)
        } else {
            // Read notes: get spreadsheet with includeGridData
            let url = format!(
                "{}/spreadsheets/{}?ranges={}&includeGridData=true",
                BASE,
                spreadsheet_id,
                urlencoding(range)
            );
            let resp = self.http.get(&url, &self.token).await?;
            self.parse(&resp)
        }
    }

    /// Export the spreadsheet in a given MIME type (e.g. `text/csv`, `application/pdf`).
    /// Uses the Drive export endpoint.
    pub async fn export(&self, spreadsheet_id: &str, mime_type: &str) -> Result<Vec<u8>> {
        let url = format!(
            "https://www.googleapis.com/drive/v3/files/{}/export?mimeType={}",
            spreadsheet_id,
            urlencoding(mime_type)
        );
        self.http.get(&url, &self.token).await
    }

    /// Copy a sheet to another spreadsheet.
    pub async fn copy(
        &self,
        source_spreadsheet_id: &str,
        source_sheet_id: i64,
        destination_spreadsheet_id: &str,
    ) -> Result<serde_json::Value> {
        let body = serde_json::to_vec(&serde_json::json!({
            "destinationSpreadsheetId": destination_spreadsheet_id
        }))
        .map_err(|e| Error::Other(e.to_string()))?;
        let url = self.url(&format!(
            "/spreadsheets/{}/sheets/{}:copyTo",
            source_spreadsheet_id, source_sheet_id
        ));
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Get spreadsheet metadata (sheets, named ranges, properties).
    /// Alias for backwards compatibility.
    pub async fn metadata(&self, spreadsheet_id: &str) -> Result<Spreadsheet> {
        self.get(spreadsheet_id).await
    }

    /// Update values in a range.
    /// Alias for backwards compatibility with the old `update` name.
    pub async fn update(
        &self,
        spreadsheet_id: &str,
        range: &str,
        values: &[Vec<serde_json::Value>],
        _value_input_option: Option<&str>,
    ) -> Result<serde_json::Value> {
        self.values_update(spreadsheet_id, range, values).await
    }

    /// Append rows to a sheet.
    /// Alias for backwards compatibility with the old `append` name.
    pub async fn append(
        &self,
        spreadsheet_id: &str,
        range: &str,
        values: &[Vec<serde_json::Value>],
        _value_input_option: Option<&str>,
    ) -> Result<serde_json::Value> {
        self.values_append(spreadsheet_id, range, values).await
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn urlencoding(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}
