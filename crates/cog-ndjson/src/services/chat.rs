use serde::{Deserialize, Serialize};

/// Chat service protocol requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum ChatRequest {
    SpacesList,
    SpacesFind {
        name: String,
    },
    SpacesCreate {
        name: String,
        #[serde(default)]
        members: Vec<String>,
    },
    MessagesList {
        space_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max: Option<u32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        thread_id: Option<String>,
        #[serde(default)]
        unread: bool,
    },
    MessagesSend {
        space_id: String,
        text: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        thread_id: Option<String>,
    },
    ThreadsList {
        space_id: String,
    },
    DmSpace {
        email: String,
    },
    DmSend {
        email: String,
        text: String,
    },
}
