use clap::{Args, Subcommand};

/// NotebookLM integration (experimental).
#[derive(Args, Debug)]
pub struct NotebookLmArgs {
    #[command(subcommand)]
    pub command: NotebookLmCommands,
}

#[derive(Subcommand, Debug)]
pub enum NotebookLmCommands {
    /// List notebooks
    List {
        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,
    },

    /// Get a notebook by ID
    Get {
        /// Notebook ID
        notebook_id: String,
    },
}
