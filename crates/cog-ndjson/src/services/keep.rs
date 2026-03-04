use serde::{Deserialize, Serialize};

/// Keep service protocol requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum KeepRequest {
    List {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        account: Option<String>,
    },
    Get {
        note_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        account: Option<String>,
    },
    Search {
        query: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        account: Option<String>,
    },
    Attachment {
        attachment_name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        account: Option<String>,
    },
}
