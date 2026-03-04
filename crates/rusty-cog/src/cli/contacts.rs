use clap::{Args, Subcommand};

/// Google Contacts operations.
#[derive(Args, Debug)]
pub struct ContactsArgs {
    #[command(subcommand)]
    pub command: ContactsCommands,
}

#[derive(Subcommand, Debug)]
pub enum ContactsCommands {
    /// List contacts
    List {
        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,

        /// Sort order (LAST_MODIFIED_ASCENDING, LAST_MODIFIED_DESCENDING, FIRST_NAME_ASCENDING, LAST_NAME_ASCENDING)
        #[arg(long)]
        sort: Option<String>,
    },

    /// Search contacts
    Search {
        /// Search query
        query: String,

        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,
    },

    /// Get a contact by resource name
    Get {
        /// Contact resource name (e.g. "people/c123456")
        resource_name: String,
    },

    /// Create a new contact
    Create {
        /// First/given name
        #[arg(long)]
        first_name: Option<String>,

        /// Last/family name
        #[arg(long)]
        last_name: Option<String>,

        /// Email address (repeatable)
        #[arg(long, num_args = 1..)]
        email: Vec<String>,

        /// Phone number (repeatable)
        #[arg(long, num_args = 1..)]
        phone: Vec<String>,

        /// Organization/company
        #[arg(long)]
        organization: Option<String>,

        /// Job title
        #[arg(long)]
        title: Option<String>,

        /// Notes
        #[arg(long)]
        notes: Option<String>,
    },

    /// Update an existing contact
    Update {
        /// Contact resource name
        resource_name: String,

        /// First/given name
        #[arg(long)]
        first_name: Option<String>,

        /// Last/family name
        #[arg(long)]
        last_name: Option<String>,

        /// Email address (repeatable, replaces all)
        #[arg(long, num_args = 1..)]
        email: Vec<String>,

        /// Phone number (repeatable, replaces all)
        #[arg(long, num_args = 1..)]
        phone: Vec<String>,

        /// Organization/company
        #[arg(long)]
        organization: Option<String>,

        /// Job title
        #[arg(long)]
        title: Option<String>,

        /// Notes
        #[arg(long)]
        notes: Option<String>,
    },

    /// Delete a contact
    #[cfg(feature = "destructive-permanent")]
    Delete {
        /// Contact resource name
        resource_name: String,
    },

    /// Other contacts (read-only, auto-collected)
    Other {
        #[command(subcommand)]
        sub: OtherContactsCommands,
    },

    /// Directory contacts (domain-wide)
    Directory {
        #[command(subcommand)]
        sub: DirectoryCommands,
    },
}

/// Subcommands under `contacts other`.
#[derive(Subcommand, Debug)]
pub enum OtherContactsCommands {
    /// List other contacts
    List {
        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,
    },

    /// Search other contacts
    Search {
        /// Search query
        query: String,

        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,
    },
}

/// Subcommands under `contacts directory`.
#[derive(Subcommand, Debug)]
pub enum DirectoryCommands {
    /// List directory contacts
    List {
        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,
    },

    /// Search directory contacts
    Search {
        /// Search query
        query: String,

        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,
    },
}
