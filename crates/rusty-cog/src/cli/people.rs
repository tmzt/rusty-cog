use clap::{Args, Subcommand};

/// Google People API operations.
#[derive(Args, Debug)]
pub struct PeopleArgs {
    #[command(subcommand)]
    pub command: PeopleCommands,
}

#[derive(Subcommand, Debug)]
pub enum PeopleCommands {
    /// Show current user profile
    Me,

    /// Get a person by resource name
    Get {
        /// Person resource name (e.g. "people/123456")
        resource_name: String,
    },

    /// Search the directory for people
    Search {
        /// Search query
        query: String,

        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,
    },

    /// Show a person's relations/connections
    Relations {
        /// Person resource name (omit for current user)
        resource_name: Option<String>,

        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,
    },
}
