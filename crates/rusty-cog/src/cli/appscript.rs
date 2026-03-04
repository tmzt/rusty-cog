use clap::{Args, Subcommand};

/// Google Apps Script operations.
#[derive(Args, Debug)]
pub struct AppScriptArgs {
    #[command(subcommand)]
    pub command: AppScriptCommands,
}

#[derive(Subcommand, Debug)]
pub enum AppScriptCommands {
    /// Get script project metadata
    Get {
        /// Script project ID
        script_id: String,
    },

    /// Get script project content (source files)
    Content {
        /// Script project ID
        script_id: String,
    },

    /// Create a new script project
    Create {
        /// Project title
        #[arg(long)]
        title: String,

        /// Parent Drive file ID (Docs, Sheets, etc. to bind to)
        #[arg(long)]
        parent_id: Option<String>,
    },

    /// Run a script function
    Run {
        /// Script project ID
        script_id: String,

        /// Function name to execute
        #[arg(long)]
        function: String,

        /// Function parameters as JSON array
        #[arg(long)]
        params: Option<String>,

        /// Run in development mode
        #[arg(long)]
        dev_mode: bool,
    },
}
