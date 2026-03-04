pub mod gmail;
pub mod calendar;
pub mod drive;
pub mod docs;
pub mod sheets;
pub mod slides;
pub mod forms;
pub mod contacts;
pub mod tasks;
pub mod people;
pub mod chat;
pub mod classroom;
pub mod groups;
pub mod keep;
pub mod appscript;

#[cfg(feature = "gemini-web")]
pub mod gemini;

#[cfg(feature = "notebooklm")]
pub mod notebooklm;

/// Common pagination response wrapper.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PagedResponse<T> {
    pub items: Vec<T>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(default)]
    pub total: Option<u64>,
}
