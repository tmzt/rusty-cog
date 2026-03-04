use clap::{Args, Subcommand};

/// Gemini web access (experimental, read-only).
#[derive(Args, Debug)]
pub struct GeminiArgs {
    #[command(subcommand)]
    pub command: GeminiCommands,
}

#[derive(Subcommand, Debug)]
pub enum GeminiCommands {
    /// List recent conversations
    List {
        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,
    },

    /// Get a conversation by ID
    Get {
        /// Conversation ID
        conversation_id: String,
    },

    /// Search conversations
    Search {
        /// Search query
        query: String,

        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,
    },
}
