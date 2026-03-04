use clap::{Args, Subcommand};

/// Google Forms operations.
#[derive(Args, Debug)]
pub struct FormsArgs {
    #[command(subcommand)]
    pub command: FormsCommands,
}

#[derive(Subcommand, Debug)]
pub enum FormsCommands {
    /// Get form metadata and questions
    Get {
        /// Form ID
        id: String,
    },

    /// Create a new form
    Create {
        /// Form title
        #[arg(long)]
        title: String,

        /// Form description
        #[arg(long)]
        description: Option<String>,

        /// Document title (file name in Drive)
        #[arg(long)]
        document_title: Option<String>,
    },

    /// Form response management
    Responses {
        #[command(subcommand)]
        sub: ResponsesCommands,
    },
}

/// Subcommands under `forms responses`.
#[derive(Subcommand, Debug)]
pub enum ResponsesCommands {
    /// List form responses
    List {
        /// Form ID
        id: String,

        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,
    },

    /// Get a single form response
    Get {
        /// Form ID
        id: String,

        /// Response ID
        response_id: String,
    },
}
