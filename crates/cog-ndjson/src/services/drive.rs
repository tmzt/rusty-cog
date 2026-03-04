use serde::{Deserialize, Serialize};

/// Drive service protocol requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum DriveRequest {
    List {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max: Option<u32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        parent: Option<String>,
        #[serde(default)]
        no_all_drives: bool,
    },
    Search {
        query: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max: Option<u32>,
        #[serde(default)]
        no_all_drives: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        raw_query: Option<String>,
    },
    Get {
        file_id: String,
    },
    Upload {
        file_path: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        parent: Option<String>,
        #[serde(default)]
        replace: bool,
        #[serde(default)]
        convert: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        convert_to: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    Download {
        file_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        format: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        out: Option<String>,
    },
    Copy {
        file_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    Mkdir {
        name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        parent: Option<String>,
    },
    Rename {
        file_id: String,
        name: String,
    },
    Move {
        file_id: String,
        parent: String,
    },
    Trash {
        file_id: String,
    },
    #[cfg(feature = "destructive-permanent")]
    PermanentDelete {
        file_id: String,
    },
    #[cfg(feature = "destructive-permanent")]
    EmptyTrash,
    Share {
        file_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        share_to: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        email: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        domain: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        role: Option<String>,
    },
    Unshare {
        file_id: String,
        permission_id: String,
    },
    Permissions {
        file_id: String,
    },
    Drives {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max: Option<u32>,
    },
    BatchTrash {
        #[serde(default)]
        file_ids: Vec<String>,
    },
    Url {
        file_id: String,
    },
}
