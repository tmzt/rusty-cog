pub mod auth;
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
pub mod config;
pub mod time;

#[cfg(feature = "gemini-web")]
pub mod gemini;

#[cfg(feature = "notebooklm")]
pub mod notebooklm;

use clap::{Parser, Subcommand};

/// Google Workspace CLI -- Rust port of gogcli
#[derive(Parser, Debug)]
#[command(name = "cog", version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Account email or alias
    #[arg(short = 'a', long, global = true, env = "COG_ACCOUNT")]
    pub account: Option<String>,

    /// OAuth client name
    #[arg(long, global = true, env = "COG_CLIENT")]
    pub client: Option<String>,

    /// JSON output
    #[arg(short = 'j', long, global = true, env = "COG_JSON")]
    pub json: bool,

    /// TSV output (no colors)
    #[arg(short = 'p', long, global = true, env = "COG_PLAIN")]
    pub plain: bool,

    /// In JSON mode, only primary result
    #[arg(long, global = true)]
    pub results_only: bool,

    /// Project JSON fields (comma-separated)
    #[arg(long, global = true)]
    pub select: Option<String>,

    /// Color output mode
    #[arg(long, global = true, default_value = "auto", env = "COG_COLOR")]
    pub color: String,

    /// Print intended actions without executing
    #[arg(short = 'n', long, global = true)]
    pub dry_run: bool,

    /// Skip destructive confirmations
    #[arg(short = 'y', long, global = true)]
    pub force: bool,

    /// Never prompt (CI mode)
    #[arg(long, global = true)]
    pub no_input: bool,

    /// Debug logging
    #[arg(short = 'v', long, global = true)]
    pub verbose: bool,

    /// Restrict available commands (comma-separated)
    #[arg(long, global = true, env = "COG_ENABLE_COMMANDS")]
    pub enable_commands: Option<String>,

    /// Output file path
    #[arg(long = "out", alias = "output", global = true)]
    pub output: Option<String>,

    /// Output directory path
    #[arg(long = "out-dir", alias = "output-dir", global = true)]
    pub output_dir: Option<String>,

    /// Enable monitor mode
    #[arg(long, global = true)]
    pub monitor: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Authentication and credential management
    Auth(auth::AuthArgs),

    /// Email operations
    Gmail(gmail::GmailArgs),

    /// Calendar management
    Calendar(calendar::CalendarArgs),

    /// Cloud storage operations
    Drive(drive::DriveArgs),

    /// Document editing and export
    Docs(docs::DocsArgs),

    /// Spreadsheet operations
    Sheets(sheets::SheetsArgs),

    /// Presentation management
    Slides(slides::SlidesArgs),

    /// Form creation and responses
    Forms(forms::FormsArgs),

    /// Contact management
    Contacts(contacts::ContactsArgs),

    /// Task list operations
    Tasks(tasks::TasksArgs),

    /// Profile and directory access
    People(people::PeopleArgs),

    /// Workspace chat messaging
    Chat(chat::ChatArgs),

    /// Education platform management
    Classroom(classroom::ClassroomArgs),

    /// Workspace group management
    Groups(groups::GroupsArgs),

    /// Note-taking (Workspace only)
    Keep(keep::KeepArgs),

    /// Apps Script project management
    #[command(name = "appscript")]
    AppScript(appscript::AppScriptArgs),

    /// Configuration management
    Config(config::ConfigArgs),

    /// Timezone utilities
    Time(time::TimeArgs),

    /// Shell completion generation
    Completion {
        /// Shell type
        shell: String,
    },

    /// MCP server mode (JSON-RPC over stdio)
    Mcp,

    /// Print version
    Version,

    /// Show available Google services and scopes
    Schema,

    /// Show all exit codes
    #[command(name = "agent")]
    Agent {
        #[command(subcommand)]
        command: AgentCommands,
    },

    #[cfg(feature = "gemini-web")]
    /// Gemini web access (experimental, read-only)
    Gemini(gemini::GeminiArgs),

    #[cfg(feature = "notebooklm")]
    /// NotebookLM integration (experimental)
    #[command(name = "notebooklm")]
    NotebookLm(notebooklm::NotebookLmArgs),

    // -- Desire-path shortcuts --

    /// Send an email (shortcut for gmail send)
    Send(gmail::SendArgs),

    /// List Drive files (shortcut for drive ls)
    Ls(drive::LsArgs),

    /// Search Drive (shortcut for drive search)
    Search(drive::SearchArgs),

    /// Open URL for a resource
    Open {
        /// Resource ID
        id: String,
    },

    /// Download from Drive (shortcut for drive download)
    Download(drive::DownloadArgs),

    /// Upload to Drive (shortcut for drive upload)
    Upload(drive::UploadArgs),

    /// Add an account (shortcut for auth add)
    Login(auth::LoginArgs),

    /// Remove an account (shortcut for auth remove)
    Logout(auth::LogoutArgs),

    /// Show auth status (shortcut for auth status)
    Status,

    /// Show current user profile (shortcut for people me)
    Me,

    /// Show current user profile (alias for me)
    #[command(name = "whoami")]
    WhoAmI,
}

#[derive(Subcommand, Debug)]
pub enum AgentCommands {
    /// Print all exit codes
    ExitCodes,
}
