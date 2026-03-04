use clap::{Args, Subcommand};

/// Google Tasks operations.
#[derive(Args, Debug)]
pub struct TasksArgs {
    #[command(subcommand)]
    pub command: TasksCommands,
}

#[derive(Subcommand, Debug)]
pub enum TasksCommands {
    /// Task list management
    Lists {
        #[command(subcommand)]
        sub: ListsCommands,
    },

    /// List tasks in a task list
    List {
        /// Task list ID (default: "@default")
        #[arg(long)]
        list: Option<String>,

        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,

        /// Show completed tasks
        #[arg(long)]
        show_completed: bool,

        /// Show hidden tasks
        #[arg(long)]
        show_hidden: bool,

        /// Due date minimum (RFC 3339)
        #[arg(long)]
        due_min: Option<String>,

        /// Due date maximum (RFC 3339)
        #[arg(long)]
        due_max: Option<String>,
    },

    /// Get a single task
    Get {
        /// Task ID
        id: String,

        /// Task list ID (default: "@default")
        #[arg(long)]
        list: Option<String>,
    },

    /// Add a new task
    Add {
        /// Task title
        #[arg(long)]
        title: String,

        /// Task notes/description
        #[arg(long)]
        notes: Option<String>,

        /// Due date (RFC 3339 or YYYY-MM-DD)
        #[arg(long)]
        due: Option<String>,

        /// Task list ID (default: "@default")
        #[arg(long)]
        list: Option<String>,

        /// Parent task ID (for subtasks)
        #[arg(long)]
        parent: Option<String>,

        /// Insert after this task ID
        #[arg(long)]
        previous: Option<String>,
    },

    /// Update an existing task
    Update {
        /// Task ID
        id: String,

        /// New title
        #[arg(long)]
        title: Option<String>,

        /// New notes/description
        #[arg(long)]
        notes: Option<String>,

        /// New due date (RFC 3339 or YYYY-MM-DD)
        #[arg(long)]
        due: Option<String>,

        /// Task list ID (default: "@default")
        #[arg(long)]
        list: Option<String>,
    },

    /// Mark a task as complete
    Done {
        /// Task ID
        id: String,

        /// Task list ID (default: "@default")
        #[arg(long)]
        list: Option<String>,
    },

    /// Mark a task as incomplete
    Undo {
        /// Task ID
        id: String,

        /// Task list ID (default: "@default")
        #[arg(long)]
        list: Option<String>,
    },

    /// Delete a task
    #[cfg(feature = "destructive-permanent")]
    Delete {
        /// Task ID
        id: String,

        /// Task list ID (default: "@default")
        #[arg(long)]
        list: Option<String>,
    },

    /// Clear completed tasks from a list
    #[cfg(feature = "destructive-permanent")]
    Clear {
        /// Task list ID (default: "@default")
        #[arg(long)]
        list: Option<String>,
    },
}

/// Subcommands under `tasks lists`.
#[derive(Subcommand, Debug)]
pub enum ListsCommands {
    /// List all task lists
    List {
        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,
    },

    /// Create a new task list
    Create {
        /// Task list title
        #[arg(long)]
        title: String,
    },

    /// Update a task list
    Update {
        /// Task list ID
        id: String,

        /// New title
        #[arg(long)]
        title: String,
    },

    /// Delete a task list
    Delete {
        /// Task list ID
        id: String,
    },

    /// Get a single task list
    Get {
        /// Task list ID
        id: String,
    },
}
