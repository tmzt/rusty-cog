use clap::{Args, Subcommand};

/// Google Docs operations.
#[derive(Args, Debug)]
pub struct DocsArgs {
    #[command(subcommand)]
    pub command: DocsCommands,
}

#[derive(Subcommand, Debug)]
pub enum DocsCommands {
    /// Show document metadata and info
    Info {
        /// Document ID
        id: String,
    },

    /// Print document content as plain text
    Cat {
        /// Document ID
        id: String,

        /// Tab name or index to read
        #[arg(long)]
        tab: Option<String>,
    },

    /// Create a new document
    Create {
        /// Document title
        #[arg(long)]
        title: String,

        /// Initial body text
        #[arg(long)]
        body: Option<String>,

        /// Parent folder ID in Drive
        #[arg(long)]
        parent: Option<String>,
    },

    /// Copy an existing document
    Copy {
        /// Source document ID
        id: String,

        /// Title for the copy
        #[arg(long)]
        title: Option<String>,

        /// Destination folder ID
        #[arg(long)]
        parent: Option<String>,
    },

    /// Export document to a file format
    Export {
        /// Document ID
        id: String,

        /// Export format (pdf, docx, txt, html, epub, odt, rtf, md)
        #[arg(long)]
        format: Option<String>,

        /// Output file path
        #[arg(long)]
        out: Option<String>,
    },

    /// Write/insert text into a document
    Write {
        /// Document ID
        id: String,

        /// Text to insert
        #[arg(long)]
        text: Option<String>,

        /// Read text from file
        #[arg(long)]
        file: Option<String>,

        /// Insert at index position
        #[arg(long)]
        index: Option<i64>,

        /// Insert at end of document
        #[arg(long)]
        append: bool,

        /// Tab name or index
        #[arg(long)]
        tab: Option<String>,
    },

    /// Find and replace text in a document
    FindReplace {
        /// Document ID
        id: String,

        /// Text to find
        #[arg(long)]
        find: String,

        /// Replacement text
        #[arg(long)]
        replace: String,

        /// Case-sensitive match
        #[arg(long)]
        match_case: bool,
    },

    /// Sed-like find and replace (s/pattern/replacement/)
    Sed {
        /// Document ID
        id: String,

        /// Sed expression (e.g. "s/old/new/g")
        expression: String,
    },

    /// List document tabs
    ListTabs {
        /// Document ID
        id: String,
    },

    /// Apply batch updates to a document
    Update {
        /// Document ID
        id: String,

        /// JSON file with batch update requests
        #[arg(long)]
        requests_file: Option<String>,

        /// Inline JSON batch update requests
        #[arg(long)]
        requests: Option<String>,
    },
}
