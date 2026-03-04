use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub id: String,
    #[serde(default)]
    pub thread_id: Option<String>,
    #[serde(default)]
    pub label_ids: Vec<String>,
    #[serde(default)]
    pub snippet: Option<String>,
    #[serde(default)]
    pub history_id: Option<String>,
    #[serde(default)]
    pub internal_date: Option<String>,
    #[serde(default)]
    pub size_estimate: Option<i64>,
    #[serde(default)]
    pub payload: Option<MessagePart>,
    #[serde(default)]
    pub raw: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessagePart {
    #[serde(default)]
    pub part_id: Option<String>,
    #[serde(default)]
    pub mime_type: Option<String>,
    #[serde(default)]
    pub filename: Option<String>,
    #[serde(default)]
    pub headers: Vec<Header>,
    #[serde(default)]
    pub body: Option<MessagePartBody>,
    #[serde(default)]
    pub parts: Vec<MessagePart>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessagePartBody {
    #[serde(default)]
    pub attachment_id: Option<String>,
    #[serde(default)]
    pub size: Option<i64>,
    #[serde(default)]
    pub data: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thread {
    pub id: String,
    #[serde(default)]
    pub history_id: Option<String>,
    #[serde(default)]
    pub snippet: Option<String>,
    #[serde(default)]
    pub messages: Vec<Message>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub label_list_visibility: Option<String>,
    #[serde(default)]
    pub message_list_visibility: Option<String>,
    #[serde(rename = "type", default)]
    pub label_type: Option<String>,
    #[serde(default)]
    pub messages_total: Option<i64>,
    #[serde(default)]
    pub messages_unread: Option<i64>,
    #[serde(default)]
    pub threads_total: Option<i64>,
    #[serde(default)]
    pub threads_unread: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Draft {
    pub id: String,
    #[serde(default)]
    pub message: Option<Message>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    pub id: String,
    #[serde(default)]
    pub criteria: Option<FilterCriteria>,
    #[serde(default)]
    pub action: Option<FilterAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterCriteria {
    #[serde(default)]
    pub from: Option<String>,
    #[serde(default)]
    pub to: Option<String>,
    #[serde(default)]
    pub subject: Option<String>,
    #[serde(default)]
    pub query: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterAction {
    #[serde(default)]
    pub add_label_ids: Vec<String>,
    #[serde(default)]
    pub remove_label_ids: Vec<String>,
    #[serde(default)]
    pub forward: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoForwardingSettings {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub email_address: Option<String>,
    #[serde(default)]
    pub disposition: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VacationSettings {
    #[serde(default)]
    pub enable_auto_reply: bool,
    #[serde(default)]
    pub response_subject: Option<String>,
    #[serde(default)]
    pub response_body_plain_text: Option<String>,
    #[serde(default)]
    pub response_body_html: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Delegate {
    pub delegate_email: String,
    #[serde(default)]
    pub verification_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendAs {
    pub send_as_email: String,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub is_primary: bool,
    #[serde(default)]
    pub is_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryRecord {
    pub id: String,
    #[serde(default)]
    pub messages: Vec<Message>,
    #[serde(default)]
    pub messages_added: Vec<HistoryMessageAdded>,
    #[serde(default)]
    pub messages_deleted: Vec<HistoryMessageDeleted>,
    #[serde(default)]
    pub labels_added: Vec<HistoryLabelAdded>,
    #[serde(default)]
    pub labels_removed: Vec<HistoryLabelRemoved>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryMessageAdded {
    pub message: Message,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryMessageDeleted {
    pub message: Message,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryLabelAdded {
    pub message: Message,
    pub label_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryLabelRemoved {
    pub message: Message,
    pub label_ids: Vec<String>,
}

/// Parameters for composing/sending a message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComposeParams {
    pub to: Vec<String>,
    #[serde(default)]
    pub cc: Vec<String>,
    #[serde(default)]
    pub bcc: Vec<String>,
    pub subject: String,
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub body_html: Option<String>,
    #[serde(default)]
    pub body_file: Option<String>,
    #[serde(default)]
    pub reply_to_message_id: Option<String>,
    #[serde(default)]
    pub quote: bool,
    #[serde(default)]
    pub track: bool,
    #[serde(default)]
    pub track_split: bool,
}
