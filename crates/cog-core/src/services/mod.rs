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
