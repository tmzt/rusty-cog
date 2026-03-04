// #![cfg(feature = "notebooklm")]

use serde::{Deserialize, Serialize};

/// NotebookLM service protocol requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum NotebookLmRequest {
    List {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max: Option<u32>,
    },
    Get {
        notebook_id: String,
    },
}
