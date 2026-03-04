use serde::{Deserialize, Serialize};

/// Forms service protocol requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum FormsRequest {
    Get {
        form_id: String,
    },
    Create {
        title: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
    ResponsesList {
        form_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max: Option<u32>,
    },
    ResponsesGet {
        form_id: String,
        response_id: String,
    },
}
