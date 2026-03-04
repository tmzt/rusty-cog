use serde::{Deserialize, Serialize};

/// Sheets service protocol requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum SheetsRequest {
    Metadata {
        spreadsheet_id: String,
    },
    Get {
        spreadsheet_id: String,
        range: String,
    },
    Create {
        title: String,
        #[serde(default)]
        sheets: Vec<String>,
    },
    Update {
        spreadsheet_id: String,
        range: String,
        #[serde(default)]
        values: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        values_json: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        copy_validation_from: Option<String>,
    },
    Append {
        spreadsheet_id: String,
        range: String,
        #[serde(default)]
        values: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        copy_validation_from: Option<String>,
    },
    Clear {
        spreadsheet_id: String,
        range: String,
    },
    Format {
        spreadsheet_id: String,
        range: String,
        format_json: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        format_fields: Option<String>,
    },
    Insert {
        spreadsheet_id: String,
        sheet_name: String,
        dimension: String,
        index: u32,
        count: u32,
        #[serde(default)]
        after: bool,
    },
    Notes {
        spreadsheet_id: String,
        range: String,
    },
    Export {
        spreadsheet_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        format: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        out: Option<String>,
    },
    Copy {
        spreadsheet_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
}
