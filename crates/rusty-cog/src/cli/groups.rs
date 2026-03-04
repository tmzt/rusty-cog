use clap::{Args, Subcommand};

/// Google Groups operations.
#[derive(Args, Debug)]
pub struct GroupsArgs {
    #[command(subcommand)]
    pub command: GroupsCommands,
}

#[derive(Subcommand, Debug)]
pub enum GroupsCommands {
    /// List groups
    List {
        /// Domain to filter by
        #[arg(long)]
        domain: Option<String>,

        /// User email to list groups for
        #[arg(long)]
        user: Option<String>,

        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,
    },

    /// List group members
    Members {
        /// Group email address
        group_email: String,

        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,

        /// Filter by role (OWNER, MANAGER, MEMBER)
        #[arg(long)]
        role: Option<String>,
    },
}
