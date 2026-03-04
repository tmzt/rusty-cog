use serde::{Deserialize, Serialize};

/// Docs service protocol requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum DocsRequest {
    Info {
        doc_id: String,
    },
    Cat {
        doc_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_bytes: Option<usize>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        tab: Option<String>,
        #[serde(default)]
        all_tabs: bool,
    },
    Create {
        title: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        file: Option<String>,
    },
    Copy {
        doc_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    Export {
        doc_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        format: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        out: Option<String>,
    },
    Write {
        doc_id: String,
        #[serde(default)]
        replace: bool,
        #[serde(default)]
        markdown: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        file: Option<String>,
    },
    FindReplace {
        doc_id: String,
        find: String,
        replace_with: String,
    },
    Sed {
        doc_id: String,
        expression: String,
    },
    ListTabs {
        doc_id: String,
    },
}
