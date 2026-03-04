use serde::{Deserialize, Serialize};

/// Slides service protocol requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum SlidesRequest {
    Info {
        presentation_id: String,
    },
    Create {
        title: String,
    },
    CreateFromMarkdown {
        title: String,
        content_file: String,
    },
    Copy {
        presentation_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    Export {
        presentation_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        format: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        out: Option<String>,
    },
    ListSlides {
        presentation_id: String,
    },
    AddSlide {
        presentation_id: String,
        image_path: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        notes: Option<String>,
    },
    UpdateNotes {
        presentation_id: String,
        slide_id: String,
        notes: String,
    },
    ReplaceSlide {
        presentation_id: String,
        slide_id: String,
        image_path: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        notes: Option<String>,
    },
    #[cfg(feature = "destructive-permanent")]
    DeleteSlide {
        presentation_id: String,
        slide_id: String,
    },
}
