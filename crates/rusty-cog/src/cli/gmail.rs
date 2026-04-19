use clap::{Args, Subcommand};

/// Gmail operations.
#[derive(Args, Debug)]
pub struct GmailArgs {
    #[command(subcommand)]
    pub command: GmailCommands,
}

#[derive(Subcommand, Debug)]
pub enum GmailCommands {
    /// Search for messages
    Search {
        /// Gmail search query
        query: String,

        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,

        /// Include spam and trash
        #[arg(long)]
        include_spam_trash: bool,

        /// Label IDs to filter by
        #[arg(long, num_args = 1..)]
        label_ids: Vec<String>,
    },

    /// List inbox messages (shorthand for `search "in:inbox"`)
    Inbox {
        /// Maximum number of results
        #[arg(long, default_value_t = 20)]
        max: u32,

        /// Show unread only (adds `is:unread`)
        #[arg(long)]
        unread: bool,

        /// Time window, e.g. "1d", "6h", "1w" (adds `newer_than:<val>`)
        #[arg(long)]
        newer_than: Option<String>,

        /// Extra raw query text appended to the generated query
        #[arg(long)]
        q: Option<String>,
    },

    /// Message operations
    Messages {
        #[command(subcommand)]
        sub: MessagesCommands,
    },

    /// Thread operations
    Thread {
        #[command(subcommand)]
        sub: ThreadCommands,
    },

    /// Get a message by ID
    Get {
        /// Message ID
        id: String,

        /// Response format (full, metadata, minimal, raw)
        #[arg(long)]
        format: Option<String>,
    },

    /// Send an email
    Send(SendArgs),

    /// Download an attachment
    Attachment {
        /// Message ID
        message_id: String,

        /// Attachment ID
        attachment_id: String,

        /// Output file path
        #[arg(long)]
        out: Option<String>,
    },

    /// Get Gmail URL for a message or thread
    Url {
        /// Message or thread ID
        id: String,
    },

    /// Draft management
    Drafts {
        #[command(subcommand)]
        sub: DraftsCommands,
    },

    /// Label management
    Labels {
        #[command(subcommand)]
        sub: LabelsCommands,
    },

    /// Batch operations on messages
    Batch {
        #[command(subcommand)]
        sub: BatchCommands,
    },

    /// Email filter management
    Filters {
        #[command(subcommand)]
        sub: FiltersCommands,
    },

    /// Auto-forwarding settings
    AutoForward {
        #[command(subcommand)]
        sub: AutoForwardCommands,
    },

    /// Forwarding address management
    Forwarding {
        #[command(subcommand)]
        sub: ForwardingCommands,
    },

    /// Send-as alias management
    SendAs {
        #[command(subcommand)]
        sub: SendAsCommands,
    },

    /// Vacation responder settings
    Vacation {
        #[command(subcommand)]
        sub: VacationCommands,
    },

    /// Mail delegation management
    Delegates {
        #[command(subcommand)]
        sub: DelegatesCommands,
    },

    /// Push notification watch management
    Watch {
        #[command(subcommand)]
        sub: WatchCommands,
    },

    /// List history changes
    History {
        /// Start history ID
        #[arg(long)]
        start_history_id: Option<String>,

        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,
    },

    /// Email tracking management
    Track {
        #[command(subcommand)]
        sub: TrackCommands,
    },
}

/// Arguments for sending an email.
#[derive(Args, Debug, Clone)]
pub struct SendArgs {
    /// Recipient email address(es) (repeatable)
    #[arg(long, num_args = 1..)]
    pub to: Vec<String>,

    /// CC recipient(s) (repeatable)
    #[arg(long, num_args = 1..)]
    pub cc: Vec<String>,

    /// BCC recipient(s) (repeatable)
    #[arg(long, num_args = 1..)]
    pub bcc: Vec<String>,

    /// Email subject
    #[arg(short, long)]
    pub subject: Option<String>,

    /// Plain text body
    #[arg(short, long)]
    pub body: Option<String>,

    /// HTML body
    #[arg(long)]
    pub body_html: Option<String>,

    /// Read body from file
    #[arg(long)]
    pub body_file: Option<String>,

    /// Reply to a message ID
    #[arg(long)]
    pub reply_to_message_id: Option<String>,

    /// Include quoted text in reply
    #[arg(long)]
    pub quote: bool,

    /// Enable read tracking
    #[arg(long)]
    pub track: bool,

    /// Split tracking pixel per recipient
    #[arg(long)]
    pub track_split: bool,
}

/// Subcommands under `gmail messages`.
#[derive(Subcommand, Debug)]
pub enum MessagesCommands {
    /// Search messages
    Search {
        /// Gmail search query
        query: String,

        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,
    },
}

/// Subcommands under `gmail thread`.
#[derive(Subcommand, Debug)]
pub enum ThreadCommands {
    /// Get a thread by ID
    Get {
        /// Thread ID
        id: String,

        /// Response format (full, metadata, minimal)
        #[arg(long)]
        format: Option<String>,
    },

    /// Modify thread labels
    Modify {
        /// Thread ID
        id: String,

        /// Label IDs to add (repeatable)
        #[arg(long, num_args = 1..)]
        add_labels: Vec<String>,

        /// Label IDs to remove (repeatable)
        #[arg(long, num_args = 1..)]
        remove_labels: Vec<String>,
    },
}

/// Subcommands under `gmail drafts`.
#[derive(Subcommand, Debug)]
pub enum DraftsCommands {
    /// List drafts
    List {
        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,
    },

    /// Get a draft by ID
    Get {
        /// Draft ID
        id: String,
    },

    /// Create a draft
    Create(SendArgs),

    /// Update an existing draft
    Update {
        /// Draft ID
        id: String,

        #[command(flatten)]
        args: SendArgs,
    },

    /// Send a draft
    Send {
        /// Draft ID
        id: String,
    },

    /// Delete a draft
    Delete {
        /// Draft ID
        id: String,
    },
}

/// Subcommands under `gmail labels`.
#[derive(Subcommand, Debug)]
pub enum LabelsCommands {
    /// List all labels
    List,

    /// Get a label by ID
    Get {
        /// Label ID
        id: String,
    },

    /// Create a label
    Create {
        /// Label name
        name: String,

        /// Label list visibility
        #[arg(long)]
        label_list_visibility: Option<String>,

        /// Message list visibility
        #[arg(long)]
        message_list_visibility: Option<String>,
    },

    /// Update a label
    Update {
        /// Label ID
        id: String,

        /// New label name
        #[arg(long)]
        name: Option<String>,
    },

    /// Delete a label
    #[cfg(feature = "destructive-permanent")]
    Delete {
        /// Label ID
        id: String,
    },
}

/// Subcommands under `gmail batch`.
#[derive(Subcommand, Debug)]
pub enum BatchCommands {
    /// Batch delete messages
    #[cfg(feature = "destructive-permanent")]
    Delete {
        /// Message IDs (repeatable)
        #[arg(long, num_args = 1..)]
        ids: Vec<String>,

        /// Gmail search query to select messages
        #[arg(long)]
        query: Option<String>,
    },

    /// Batch modify messages
    Modify {
        /// Message IDs (repeatable)
        #[arg(long, num_args = 1..)]
        ids: Vec<String>,

        /// Gmail search query to select messages
        #[arg(long)]
        query: Option<String>,

        /// Label IDs to add (repeatable)
        #[arg(long, num_args = 1..)]
        add_labels: Vec<String>,

        /// Label IDs to remove (repeatable)
        #[arg(long, num_args = 1..)]
        remove_labels: Vec<String>,
    },
}

/// Subcommands under `gmail filters`.
#[derive(Subcommand, Debug)]
pub enum FiltersCommands {
    /// List all filters
    List,

    /// Get a filter by ID
    Get {
        /// Filter ID
        id: String,
    },

    /// Create a filter
    Create {
        /// Match from address
        #[arg(long)]
        from: Option<String>,

        /// Match to address
        #[arg(long)]
        to: Option<String>,

        /// Match subject
        #[arg(long)]
        subject: Option<String>,

        /// Match query
        #[arg(long)]
        query: Option<String>,

        /// Add label (repeatable)
        #[arg(long, num_args = 1..)]
        add_labels: Vec<String>,

        /// Remove label (repeatable)
        #[arg(long, num_args = 1..)]
        remove_labels: Vec<String>,

        /// Forward to address
        #[arg(long)]
        forward_to: Option<String>,
    },

    /// Delete a filter
    #[cfg(feature = "destructive-permanent")]
    Delete {
        /// Filter ID
        id: String,
    },
}

/// Subcommands under `gmail auto-forward`.
#[derive(Subcommand, Debug)]
pub enum AutoForwardCommands {
    /// Get auto-forwarding settings
    Get,

    /// Update auto-forwarding settings
    Update {
        /// Enable auto-forwarding
        #[arg(long)]
        enabled: Option<bool>,

        /// Forwarding email address
        #[arg(long)]
        email: Option<String>,

        /// Disposition (leaveInInbox, markRead, archive, trash)
        #[arg(long)]
        disposition: Option<String>,
    },
}

/// Subcommands under `gmail forwarding`.
#[derive(Subcommand, Debug)]
pub enum ForwardingCommands {
    /// List forwarding addresses
    List,

    /// Get a forwarding address
    Get {
        /// Forwarding email address
        email: String,
    },

    /// Create a forwarding address
    Create {
        /// Forwarding email address
        email: String,
    },

    /// Delete a forwarding address
    Delete {
        /// Forwarding email address
        email: String,
    },
}

/// Subcommands under `gmail send-as`.
#[derive(Subcommand, Debug)]
pub enum SendAsCommands {
    /// List send-as aliases
    List,

    /// Get a send-as alias
    Get {
        /// Send-as email
        email: String,
    },

    /// Create a send-as alias
    Create {
        /// Send-as email
        email: String,

        /// Display name
        #[arg(long)]
        display_name: Option<String>,
    },

    /// Update a send-as alias
    Update {
        /// Send-as email
        email: String,

        /// Display name
        #[arg(long)]
        display_name: Option<String>,
    },

    /// Delete a send-as alias
    Delete {
        /// Send-as email
        email: String,
    },

    /// Verify a send-as alias
    Verify {
        /// Send-as email
        email: String,
    },
}

/// Subcommands under `gmail vacation`.
#[derive(Subcommand, Debug)]
pub enum VacationCommands {
    /// Get vacation responder settings
    Get,

    /// Enable or update vacation responder
    Set {
        /// Enable responder
        #[arg(long)]
        enable: bool,

        /// Subject line
        #[arg(long)]
        subject: Option<String>,

        /// Response body (plain text)
        #[arg(long)]
        body: Option<String>,

        /// Response body (HTML)
        #[arg(long)]
        body_html: Option<String>,

        /// Respond only to contacts
        #[arg(long)]
        contacts_only: bool,

        /// Respond only to domain
        #[arg(long)]
        domain_only: bool,

        /// Start time (RFC 3339)
        #[arg(long)]
        start: Option<String>,

        /// End time (RFC 3339)
        #[arg(long)]
        end: Option<String>,
    },

    /// Disable vacation responder
    Disable,
}

/// Subcommands under `gmail delegates`.
#[derive(Subcommand, Debug)]
pub enum DelegatesCommands {
    /// List mail delegates
    List,

    /// Get a delegate
    Get {
        /// Delegate email
        email: String,
    },

    /// Add a delegate
    Add {
        /// Delegate email
        email: String,
    },

    /// Remove a delegate
    Remove {
        /// Delegate email
        email: String,
    },
}

/// Subcommands under `gmail watch`.
#[derive(Subcommand, Debug)]
pub enum WatchCommands {
    /// Start push notifications
    Start {
        /// Cloud Pub/Sub topic
        #[arg(long)]
        topic: String,

        /// Label IDs to watch (repeatable)
        #[arg(long, num_args = 1..)]
        label_ids: Vec<String>,
    },

    /// Stop push notifications
    Stop,
}

/// Subcommands under `gmail track`.
#[derive(Subcommand, Debug)]
pub enum TrackCommands {
    /// List tracked messages
    List {
        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,
    },

    /// Get tracking status for a message
    Get {
        /// Message ID
        id: String,
    },
}
