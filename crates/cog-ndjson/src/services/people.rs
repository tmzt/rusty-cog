use serde::{Deserialize, Serialize};

/// People service protocol requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum PeopleRequest {
    Me,
    Get {
        resource_name: String,
    },
    Search {
        query: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max: Option<u32>,
    },
    Relations {
        resource_name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        relation_type: Option<String>,
    },
}
