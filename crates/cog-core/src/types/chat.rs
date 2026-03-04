use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Space {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "type", default)]
    pub space_type: Option<String>,
    #[serde(default)]
    pub single_user_bot_dm: bool,
    #[serde(default)]
    pub threaded: bool,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub space_threading_state: Option<String>,
    #[serde(default)]
    pub space_details: Option<SpaceDetails>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpaceDetails {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub guidelines: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub sender: Option<ChatUser>,
    #[serde(default)]
    pub create_time: Option<String>,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub thread: Option<ChatThread>,
    #[serde(default)]
    pub space: Option<Space>,
    #[serde(default)]
    pub fallback_text: Option<String>,
    #[serde(default)]
    pub action_response: Option<serde_json::Value>,
    #[serde(default)]
    pub annotations: Vec<serde_json::Value>,
    #[serde(default)]
    pub attachment: Vec<ChatAttachment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatUser {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(rename = "type", default)]
    pub user_type: Option<String>,
    #[serde(default)]
    pub domain_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatThread {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub thread_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMember {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub member: Option<ChatUser>,
    #[serde(default)]
    pub create_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatAttachment {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub content_name: Option<String>,
    #[serde(default)]
    pub content_type: Option<String>,
    #[serde(default)]
    pub thumbnail_uri: Option<String>,
    #[serde(default)]
    pub download_uri: Option<String>,
    #[serde(default)]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reaction {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub user: Option<ChatUser>,
    #[serde(default)]
    pub emoji: Option<Emoji>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Emoji {
    #[serde(default)]
    pub unicode: Option<String>,
}
