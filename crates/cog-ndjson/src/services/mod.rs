mod gmail;
mod calendar;
mod drive;
mod docs;
mod sheets;
mod slides;
mod forms;
mod contacts;
mod tasks;
mod people;
mod chat;
mod classroom;
mod groups;
mod keep;
mod appscript;

#[cfg(feature = "gemini-web")]
mod gemini;

#[cfg(feature = "notebooklm")]
mod notebooklm;

pub use gmail::GmailRequest;
pub use calendar::CalendarRequest;
pub use drive::DriveRequest;
pub use docs::DocsRequest;
pub use sheets::SheetsRequest;
pub use slides::SlidesRequest;
pub use forms::FormsRequest;
pub use contacts::ContactsRequest;
pub use tasks::TasksRequest;
pub use people::PeopleRequest;
pub use chat::ChatRequest;
pub use classroom::ClassroomRequest;
pub use groups::GroupsRequest;
pub use keep::KeepRequest;
pub use appscript::AppScriptRequest;

#[cfg(feature = "gemini-web")]
pub use gemini::GeminiRequest;

#[cfg(feature = "notebooklm")]
pub use notebooklm::NotebookLmRequest;
