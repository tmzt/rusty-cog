use serde::{Deserialize, Serialize};

/// Tasks service protocol requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum TasksRequest {
    TaskLists {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max: Option<u32>,
    },
    TaskListCreate {
        title: String,
    },
    List {
        tasklist_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max: Option<u32>,
    },
    Get {
        tasklist_id: String,
        task_id: String,
    },
    Add {
        tasklist_id: String,
        title: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        due: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repeat: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repeat_count: Option<u32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repeat_until: Option<String>,
    },
    Update {
        tasklist_id: String,
        task_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
    },
    Done {
        tasklist_id: String,
        task_id: String,
    },
    Undo {
        tasklist_id: String,
        task_id: String,
    },
    #[cfg(feature = "destructive-permanent")]
    Delete {
        tasklist_id: String,
        task_id: String,
    },
    #[cfg(feature = "destructive-permanent")]
    Clear {
        tasklist_id: String,
    },
}
