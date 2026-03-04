use clap::{Args, Subcommand};

/// Google Keep operations.
#[derive(Args, Debug)]
pub struct KeepArgs {
    #[command(subcommand)]
    pub command: KeepCommands,
}

#[derive(Subcommand, Debug)]
pub enum KeepCommands {
    /// List notes
    List {
        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,

        /// Filter (OWNED, TRASHED)
        #[arg(long)]
        filter: Option<String>,
    },

    /// Get a note by ID
    Get {
        /// Note ID
        note_id: String,
    },

    /// Search notes
    Search {
        /// Search query
        query: String,

        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,
    },

    /// Download a note attachment
    Attachment {
        /// Attachment resource name
        attachment_name: String,

        /// Output file path
        #[arg(long)]
        out: Option<String>,
    },
}
