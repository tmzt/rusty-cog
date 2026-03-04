// #![cfg(feature = "gemini-web")]

use serde::{Deserialize, Serialize};

/// Gemini web service protocol requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum GeminiRequest {
    ListConversations {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max: Option<u32>,
    },
    GetConversation {
        conversation_id: String,
    },
    SearchConversations {
        query: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max: Option<u32>,
    },
}
