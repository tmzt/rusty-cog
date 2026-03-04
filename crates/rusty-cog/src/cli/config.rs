use clap::{Args, Subcommand};

/// Configuration management.
#[derive(Args, Debug)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommands,
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    /// Show configuration file path
    Path,

    /// List all configuration values
    List,

    /// List all known configuration keys
    Keys,

    /// Get a configuration value
    Get {
        /// Configuration key
        key: String,
    },

    /// Set a configuration value
    Set {
        /// Configuration key
        key: String,

        /// Configuration value
        value: String,
    },

    /// Remove a configuration value
    Unset {
        /// Configuration key
        key: String,
    },
}
