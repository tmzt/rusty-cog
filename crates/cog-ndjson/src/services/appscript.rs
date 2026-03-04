use serde::{Deserialize, Serialize};

/// Apps Script service protocol requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum AppScriptRequest {
    Get {
        script_id: String,
    },
    Content {
        script_id: String,
    },
    Create {
        title: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        parent_id: Option<String>,
    },
    Run {
        script_id: String,
        function: String,
        #[serde(default)]
        params: Vec<serde_json::Value>,
        #[serde(default)]
        dev_mode: bool,
    },
}
