use clap::{Args, Subcommand};

/// Google Chat operations.
#[derive(Args, Debug)]
pub struct ChatArgs {
    #[command(subcommand)]
    pub command: ChatCommands,
}

#[derive(Subcommand, Debug)]
pub enum ChatCommands {
    /// Space management
    Spaces {
        #[command(subcommand)]
        sub: SpacesCommands,
    },

    /// Message operations
    Messages {
        #[command(subcommand)]
        sub: ChatMessagesCommands,
    },

    /// Thread operations
    Threads {
        #[command(subcommand)]
        sub: ThreadsCommands,
    },

    /// Direct message operations
    Dm {
        #[command(subcommand)]
        sub: DmCommands,
    },
}

/// Subcommands under `chat spaces`.
#[derive(Subcommand, Debug)]
pub enum SpacesCommands {
    /// List spaces
    List {
        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,
    },

    /// Get a space by name
    Get {
        /// Space resource name (e.g. "spaces/AAAA")
        name: String,
    },

    /// Create a space
    Create {
        /// Display name for the space
        #[arg(long)]
        display_name: String,

        /// Space type (SPACE, GROUP_CHAT)
        #[arg(long, name = "type")]
        space_type: Option<String>,
    },

    /// List space members
    Members {
        /// Space resource name
        name: String,

        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,
    },
}

/// Subcommands under `chat messages`.
#[derive(Subcommand, Debug)]
pub enum ChatMessagesCommands {
    /// List messages in a space
    List {
        /// Space resource name
        #[arg(long)]
        space: String,

        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,
    },

    /// Get a message
    Get {
        /// Message resource name
        name: String,
    },

    /// Create/send a message
    Create {
        /// Space resource name
        #[arg(long)]
        space: String,

        /// Message text
        #[arg(long)]
        text: String,

        /// Thread key (to reply in a thread)
        #[arg(long)]
        thread: Option<String>,
    },

    /// Update a message
    Update {
        /// Message resource name
        name: String,

        /// New message text
        #[arg(long)]
        text: String,
    },

    /// Delete a message
    Delete {
        /// Message resource name
        name: String,
    },
}

/// Subcommands under `chat threads`.
#[derive(Subcommand, Debug)]
pub enum ThreadsCommands {
    /// List messages in a thread
    List {
        /// Space resource name
        #[arg(long)]
        space: String,

        /// Thread key
        #[arg(long)]
        thread: String,

        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,
    },

    /// Reply to a thread
    Reply {
        /// Space resource name
        #[arg(long)]
        space: String,

        /// Thread key
        #[arg(long)]
        thread: String,

        /// Message text
        #[arg(long)]
        text: String,
    },
}

/// Subcommands under `chat dm`.
#[derive(Subcommand, Debug)]
pub enum DmCommands {
    /// Send a direct message
    Send {
        /// Recipient user ID or email
        #[arg(long)]
        user: String,

        /// Message text
        #[arg(long)]
        text: String,
    },

    /// List direct message spaces
    List {
        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,
    },
}
