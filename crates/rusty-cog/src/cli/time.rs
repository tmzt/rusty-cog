use clap::{Args, Subcommand};

/// Timezone utilities.
#[derive(Args, Debug)]
pub struct TimeArgs {
    #[command(subcommand)]
    pub command: TimeCommands,
}

#[derive(Subcommand, Debug)]
pub enum TimeCommands {
    /// Show current time in a timezone
    Now {
        /// Timezone (e.g. "America/New_York", "UTC", "EST")
        timezone: Option<String>,
    },
}
