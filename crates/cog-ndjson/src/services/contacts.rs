use serde::{Deserialize, Serialize};

/// Contacts service protocol requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum ContactsRequest {
    List {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max: Option<u32>,
    },
    Search {
        query: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max: Option<u32>,
    },
    Get {
        resource_name: String,
    },
    Create {
        given: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        family: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        email: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        phone: Option<String>,
    },
    Update {
        resource_name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        given: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        family: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        email: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        phone: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        birthday: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        notes: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        from_file: Option<String>,
    },
    #[cfg(feature = "destructive-permanent")]
    Delete {
        resource_name: String,
    },
    OtherList {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max: Option<u32>,
    },
    OtherSearch {
        query: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max: Option<u32>,
    },
    DirectoryList {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max: Option<u32>,
    },
    DirectorySearch {
        query: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max: Option<u32>,
    },
}
