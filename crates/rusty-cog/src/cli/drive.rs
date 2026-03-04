use clap::{Args, Subcommand};

/// Cloud storage operations.
#[derive(Args, Debug)]
pub struct DriveArgs {
    #[command(subcommand)]
    pub command: DriveCommands,
}

#[derive(Subcommand, Debug)]
pub enum DriveCommands {
    /// List files and folders
    Ls(LsArgs),

    /// Search for files
    Search(SearchArgs),

    /// Get file metadata
    Get {
        /// File ID
        id: String,
    },

    /// Upload a file
    Upload(UploadArgs),

    /// Download a file
    Download(DownloadArgs),

    /// Copy a file
    Copy {
        /// Source file ID
        id: String,

        /// New file name
        #[arg(long)]
        name: Option<String>,

        /// Destination parent folder ID
        #[arg(long)]
        parent: Option<String>,
    },

    /// Create a folder
    Mkdir {
        /// Folder name
        name: String,

        /// Parent folder ID
        #[arg(long)]
        parent: Option<String>,
    },

    /// Rename a file or folder
    Rename {
        /// File ID
        id: String,

        /// New name
        name: String,
    },

    /// Move a file or folder
    Move {
        /// File ID
        id: String,

        /// Destination parent folder ID
        #[arg(long)]
        parent: String,
    },

    /// Delete (trash) a file or folder
    Delete {
        /// File ID
        id: String,

        /// Permanently delete (skip trash)
        #[cfg(feature = "destructive-permanent")]
        #[arg(long)]
        permanent: bool,
    },

    /// Share a file or folder
    Share {
        /// File ID
        id: String,

        /// Email address or domain to share with
        #[arg(long)]
        email: Option<String>,

        /// Share with domain
        #[arg(long)]
        domain: Option<String>,

        /// Role (owner, organizer, writer, commenter, reader)
        #[arg(long)]
        role: String,

        /// Permission type (user, group, domain, anyone)
        #[arg(long, name = "type")]
        permission_type: Option<String>,

        /// Send notification email
        #[arg(long)]
        send_notification: bool,

        /// Notification message
        #[arg(long)]
        message: Option<String>,
    },

    /// Remove sharing permission
    Unshare {
        /// File ID
        id: String,

        /// Permission ID
        permission_id: String,
    },

    /// List file permissions
    Permissions {
        /// File ID
        id: String,
    },

    /// List shared drives
    Drives {
        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,
    },

    /// Get a Drive URL for a file
    Url {
        /// File ID
        id: String,
    },
}

/// Arguments for listing files.
#[derive(Args, Debug, Clone)]
pub struct LsArgs {
    /// Maximum number of results
    #[arg(long)]
    pub max: Option<u32>,

    /// Parent folder ID
    #[arg(long)]
    pub parent: Option<String>,

    /// Disable searching across all shared drives
    #[arg(long)]
    pub no_all_drives: bool,
}

/// Arguments for searching files.
#[derive(Args, Debug, Clone)]
pub struct SearchArgs {
    /// Search query
    pub query: String,

    /// Maximum number of results
    #[arg(long)]
    pub max: Option<u32>,

    /// Disable searching across all shared drives
    #[arg(long)]
    pub no_all_drives: bool,

    /// Use raw Drive API query syntax
    #[arg(long)]
    pub raw_query: bool,
}

/// Arguments for downloading a file.
#[derive(Args, Debug, Clone)]
pub struct DownloadArgs {
    /// File ID
    pub file_id: String,

    /// Export format (e.g. "pdf", "docx", "xlsx")
    #[arg(long)]
    pub format: Option<String>,

    /// Output file path
    #[arg(long)]
    pub out: Option<String>,
}

/// Arguments for uploading a file.
#[derive(Args, Debug, Clone)]
pub struct UploadArgs {
    /// Local file path to upload
    pub file_path: String,

    /// Parent folder ID
    #[arg(long)]
    pub parent: Option<String>,

    /// Replace an existing file (file ID)
    #[arg(long)]
    pub replace: Option<String>,

    /// Convert to Google Docs format
    #[arg(long)]
    pub convert: bool,

    /// Convert to specific Google format
    #[arg(long)]
    pub convert_to: Option<String>,

    /// Set file name (overrides local filename)
    #[arg(long)]
    pub name: Option<String>,
}
