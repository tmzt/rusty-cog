use serde::{Deserialize, Serialize};

/// Compose parameters for sending a Gmail message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Compose {
    pub to: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bcc: Option<String>,
    pub subject: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body_file: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<String>,
    #[serde(default)]
    pub quote: bool,
    #[serde(default)]
    pub track: bool,
    #[serde(default)]
    pub track_split: bool,
}

/// Gmail service protocol requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum GmailRequest {
    Search {
        query: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max: Option<u32>,
    },
    MessagesSearch {
        query: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max: Option<u32>,
        #[serde(default)]
        include_body: bool,
    },
    ThreadGet {
        thread_id: String,
        #[serde(default)]
        download: bool,
    },
    Get {
        message_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        format: Option<String>,
    },
    Send {
        compose: Compose,
    },
    Attachment {
        message_id: String,
        attachment_id: String,
    },
    DraftsList,
    DraftsCreate {
        subject: String,
        body: String,
        to: String,
    },
    DraftsSend {
        draft_id: String,
    },
    DraftsUpdate {
        draft_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        subject: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        body: Option<String>,
    },
    LabelsList,
    LabelsGet {
        label_id: String,
    },
    LabelsCreate {
        name: String,
    },
    #[cfg(feature = "destructive-permanent")]
    LabelsDelete {
        label_id: String,
    },
    Trash {
        message_id: String,
    },
    ThreadModify {
        thread_id: String,
        #[serde(default)]
        add_labels: Vec<String>,
        #[serde(default)]
        remove_labels: Vec<String>,
    },
    BatchModify {
        #[serde(default)]
        message_ids: Vec<String>,
        #[serde(default)]
        add_labels: Vec<String>,
        #[serde(default)]
        remove_labels: Vec<String>,
    },
    #[cfg(feature = "destructive-permanent")]
    BatchDelete {
        #[serde(default)]
        message_ids: Vec<String>,
    },
    FiltersList,
    FiltersCreate {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        from: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        to: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        subject: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        query: Option<String>,
        #[serde(default)]
        add_label_ids: Vec<String>,
        #[serde(default)]
        remove_label_ids: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        forward: Option<String>,
    },
    #[cfg(feature = "destructive-permanent")]
    FiltersDelete {
        filter_id: String,
    },
    AutoForwardGet,
    AutoForwardEnable {
        email: String,
    },
    AutoForwardDisable,
    VacationGet,
    VacationEnable {
        subject: String,
        message: String,
    },
    VacationDisable,
    DelegatesList,
    DelegatesAdd {
        email: String,
    },
    DelegatesRemove {
        email: String,
    },
    SendAsList,
    SendAsCreate {
        email: String,
    },
    WatchStart {
        topic: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        label: Option<String>,
    },
    History {
        since_history_id: String,
    },
    Url {
        thread_id: String,
    },
}
