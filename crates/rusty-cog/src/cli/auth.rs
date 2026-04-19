use clap::{Args, Subcommand};

/// Authentication and credential management.
#[derive(Args, Debug)]
pub struct AuthArgs {
    #[command(subcommand)]
    pub command: AuthCommands,
}

#[derive(Subcommand, Debug)]
pub enum AuthCommands {
    /// Store OAuth client credentials
    Credentials {
        /// Path to credentials file
        path: Option<String>,

        #[command(subcommand)]
        sub: Option<CredentialsSub>,
    },

    /// Authorize an account
    Add(LoginArgs),

    /// Configure service account
    ServiceAccount {
        #[command(subcommand)]
        command: ServiceAccountCommands,
    },

    /// Show or set keyring backend
    Keyring {
        /// Backend name (e.g. "file", "os")
        backend: Option<String>,
    },

    /// Show current authentication state
    Status,

    /// List available Google services and scopes
    Services,

    /// List stored accounts
    List {
        /// Check token validity for each account
        #[arg(long)]
        check: bool,
    },

    /// Remove an account
    Remove(LogoutArgs),

    /// Open Google account manager in browser
    Manage,

    /// Manage refresh tokens
    Tokens,

    /// Account alias management
    Alias {
        #[command(subcommand)]
        command: AliasCommands,
    },
}

/// Arguments for adding/authorizing an account.
#[derive(Args, Debug, Clone)]
pub struct LoginArgs {
    /// Account email
    pub email: Option<String>,

    /// Services to authorize (repeatable)
    #[arg(short, long, num_args = 1..)]
    pub services: Vec<String>,

    /// Request read-only scopes
    #[arg(long)]
    pub readonly: bool,

    /// Manual authorization (paste code instead of localhost redirect)
    #[arg(long)]
    pub manual: bool,

    /// Listen address for the OAuth callback (e.g. "127.0.0.1:44760",
    /// "100.64.0.5:44760", or ":44760" to use the default host).
    ///
    /// Overrides the default loopback binding. The URI http://<addr>/
    /// must be registered as a valid redirect URI for the OAuth client
    /// in Google Cloud Console.
    #[arg(long, value_name = "ADDR")]
    pub listen: Option<String>,

    /// Authorize for remote/headless machine
    #[arg(long)]
    pub remote: bool,

    /// Show step-by-step authorization flow
    #[arg(long)]
    pub step: bool,

    /// Force consent screen even if already authorized
    #[arg(long)]
    pub force_consent: bool,

    /// Drive scope level (e.g. "full", "appdata", "file")
    #[arg(long)]
    pub drive_scope: Option<String>,
}

/// Arguments for removing an account.
#[derive(Args, Debug, Clone)]
pub struct LogoutArgs {
    /// Account email to remove
    pub email: String,
}

/// Subcommands under `auth credentials`.
#[derive(Subcommand, Debug)]
pub enum CredentialsSub {
    /// Show current credentials path
    Path,

    /// Reset credentials to default
    Reset,

    /// Import credentials from file
    Import {
        /// Path to credentials file
        path: String,
    },

    /// Export credentials to file
    Export {
        /// Output path
        path: String,
    },
}

/// Subcommands under `auth service-account`.
#[derive(Subcommand, Debug)]
pub enum ServiceAccountCommands {
    /// Add a service account key
    Add {
        /// Path to service account JSON key file
        path: String,
    },

    /// Remove a service account
    Remove {
        /// Service account email
        email: String,
    },

    /// List configured service accounts
    List,
}

/// Subcommands under `auth alias`.
#[derive(Subcommand, Debug)]
pub enum AliasCommands {
    /// Set an alias for an account
    Set {
        /// Alias name
        alias: String,

        /// Account email
        email: String,
    },

    /// Remove an alias
    Remove {
        /// Alias name
        alias: String,
    },

    /// List all aliases
    List,
}
