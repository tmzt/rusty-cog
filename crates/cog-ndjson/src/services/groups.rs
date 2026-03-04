use serde::{Deserialize, Serialize};

/// Groups service protocol requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum GroupsRequest {
    List,
    Members {
        group_email: String,
    },
}
