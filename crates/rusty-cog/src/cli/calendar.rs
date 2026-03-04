use clap::{Args, Subcommand};

/// Calendar management.
#[derive(Args, Debug)]
pub struct CalendarArgs {
    #[command(subcommand)]
    pub command: CalendarCommands,
}

#[derive(Subcommand, Debug)]
pub enum CalendarCommands {
    /// List calendars
    Calendars {
        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,
    },

    /// List events
    Events(EventsArgs),

    /// Get a single event
    Event(EventGetArgs),

    /// Get a single event by ID
    Get(EventGetArgs),

    /// Search events
    Search {
        /// Search query
        query: String,

        /// Calendar ID (default: primary)
        #[arg(long)]
        calendar_id: Option<String>,

        /// Maximum number of results
        #[arg(long)]
        max: Option<u32>,
    },

    /// Create an event
    Create {
        /// Event title/summary
        #[arg(long)]
        title: Option<String>,

        /// Start time (RFC 3339 or natural language)
        #[arg(long)]
        start: Option<String>,

        /// End time (RFC 3339 or natural language)
        #[arg(long)]
        end: Option<String>,

        /// All-day event date (YYYY-MM-DD)
        #[arg(long)]
        date: Option<String>,

        /// Event description
        #[arg(long)]
        description: Option<String>,

        /// Event location
        #[arg(long)]
        location: Option<String>,

        /// Calendar ID (default: primary)
        #[arg(long)]
        calendar_id: Option<String>,

        /// Attendees (repeatable)
        #[arg(long, num_args = 1..)]
        attendees: Vec<String>,

        /// Timezone
        #[arg(long)]
        timezone: Option<String>,

        /// Color ID
        #[arg(long)]
        color: Option<String>,

        /// Recurrence rule (RRULE format)
        #[arg(long)]
        recurrence: Option<String>,

        /// Send notifications to attendees
        #[arg(long)]
        send_updates: Option<String>,

        /// Conference/meeting link type
        #[arg(long)]
        conference: Option<String>,
    },

    /// Update an existing event
    Update {
        /// Event ID
        id: String,

        /// Calendar ID (default: primary)
        #[arg(long)]
        calendar_id: Option<String>,

        /// New title/summary
        #[arg(long)]
        title: Option<String>,

        /// New start time
        #[arg(long)]
        start: Option<String>,

        /// New end time
        #[arg(long)]
        end: Option<String>,

        /// New description
        #[arg(long)]
        description: Option<String>,

        /// New location
        #[arg(long)]
        location: Option<String>,

        /// New color ID
        #[arg(long)]
        color: Option<String>,

        /// Send notifications to attendees
        #[arg(long)]
        send_updates: Option<String>,
    },

    /// Delete an event
    #[cfg(feature = "destructive-permanent")]
    Delete {
        /// Event ID
        id: String,

        /// Calendar ID (default: primary)
        #[arg(long)]
        calendar_id: Option<String>,

        /// Send notifications to attendees
        #[arg(long)]
        send_updates: Option<String>,
    },

    /// Respond to an event (accept/decline/tentative)
    Respond {
        /// Event ID
        id: String,

        /// Response status (accepted, declined, tentative)
        #[arg(long)]
        status: String,

        /// Calendar ID (default: primary)
        #[arg(long)]
        calendar_id: Option<String>,

        /// Send notifications to attendees
        #[arg(long)]
        send_updates: Option<String>,
    },

    /// Suggest alternative times for an event
    ProposeTimes {
        /// Event ID
        id: String,

        /// Calendar ID (default: primary)
        #[arg(long)]
        calendar_id: Option<String>,
    },

    /// Query free/busy information
    FreeBusy {
        /// Start time (RFC 3339)
        #[arg(long)]
        start: String,

        /// End time (RFC 3339)
        #[arg(long)]
        end: String,

        /// Calendar IDs or emails to check (repeatable)
        #[arg(long, num_args = 1..)]
        calendars: Vec<String>,
    },

    /// Find scheduling conflicts
    Conflicts {
        /// Start time (RFC 3339)
        #[arg(long)]
        start: Option<String>,

        /// End time (RFC 3339)
        #[arg(long)]
        end: Option<String>,

        /// Number of days to check
        #[arg(long)]
        days: Option<u32>,
    },

    /// Show team availability
    Team {
        /// Team member emails (repeatable)
        #[arg(long, num_args = 1..)]
        members: Vec<String>,

        /// Start time
        #[arg(long)]
        start: Option<String>,

        /// End time
        #[arg(long)]
        end: Option<String>,

        /// Number of days to check
        #[arg(long)]
        days: Option<u32>,
    },

    /// List available calendar colors
    Colors,

    /// Calendar access control management
    Acl {
        /// Calendar ID (default: primary)
        #[arg(long)]
        calendar_id: Option<String>,

        #[command(subcommand)]
        sub: Option<AclCommands>,
    },

    /// List users/attendees for an event
    Users {
        /// Event ID
        id: String,

        /// Calendar ID (default: primary)
        #[arg(long)]
        calendar_id: Option<String>,
    },

    /// Show current time in timezone
    Time {
        /// Timezone (e.g. "America/New_York")
        timezone: Option<String>,
    },

    /// Manage focus time events
    FocusTime {
        /// Start time
        #[arg(long)]
        start: Option<String>,

        /// End time
        #[arg(long)]
        end: Option<String>,

        /// Duration in minutes
        #[arg(long)]
        duration: Option<u32>,

        /// Calendar ID (default: primary)
        #[arg(long)]
        calendar_id: Option<String>,
    },

    /// Manage out-of-office events
    OutOfOffice {
        /// Start time
        #[arg(long)]
        start: Option<String>,

        /// End time
        #[arg(long)]
        end: Option<String>,

        /// Auto-decline message
        #[arg(long)]
        message: Option<String>,

        /// Calendar ID (default: primary)
        #[arg(long)]
        calendar_id: Option<String>,
    },

    /// Manage working location events
    WorkingLocation {
        /// Start time
        #[arg(long)]
        start: Option<String>,

        /// End time
        #[arg(long)]
        end: Option<String>,

        /// Location type (homeOffice, officeLocation, customLocation)
        #[arg(long)]
        location_type: Option<String>,

        /// Office/building name
        #[arg(long)]
        office: Option<String>,

        /// Calendar ID (default: primary)
        #[arg(long)]
        calendar_id: Option<String>,
    },
}

/// Arguments for listing events.
#[derive(Args, Debug, Clone)]
pub struct EventsArgs {
    /// Calendar ID (default: primary)
    #[arg(long)]
    pub calendar_id: Option<String>,

    /// Show today's events
    #[arg(long)]
    pub today: bool,

    /// Show tomorrow's events
    #[arg(long)]
    pub tomorrow: bool,

    /// Show this week's events
    #[arg(long)]
    pub week: bool,

    /// Number of days to show from today
    #[arg(long)]
    pub days: Option<u32>,

    /// Start date/time (RFC 3339 or YYYY-MM-DD)
    #[arg(long)]
    pub from: Option<String>,

    /// End date/time (RFC 3339 or YYYY-MM-DD)
    #[arg(long)]
    pub to: Option<String>,

    /// Show all events (no time filter)
    #[arg(long)]
    pub all: bool,

    /// Calendar IDs or names to include (repeatable)
    #[arg(long, num_args = 1..)]
    pub calendars: Vec<String>,

    /// Short calendar name/alias filter
    #[arg(long)]
    pub cal: Option<String>,

    /// Week start day (sunday, monday)
    #[arg(long)]
    pub week_start: Option<String>,

    /// Maximum number of results
    #[arg(long)]
    pub max: Option<u32>,
}

/// Arguments for getting a single event.
#[derive(Args, Debug, Clone)]
pub struct EventGetArgs {
    /// Event ID
    pub id: String,

    /// Calendar ID (default: primary)
    #[arg(long)]
    pub calendar_id: Option<String>,
}

/// Subcommands for ACL management.
#[derive(Subcommand, Debug)]
pub enum AclCommands {
    /// List ACL rules
    List,

    /// Get an ACL rule
    Get {
        /// ACL rule ID
        rule_id: String,
    },

    /// Insert an ACL rule
    Insert {
        /// Role (owner, writer, reader, freeBusyReader)
        #[arg(long)]
        role: String,

        /// Scope type (user, group, domain, default)
        #[arg(long)]
        scope_type: String,

        /// Scope value (email or domain)
        #[arg(long)]
        scope_value: Option<String>,
    },

    /// Delete an ACL rule
    Delete {
        /// ACL rule ID
        rule_id: String,
    },
}
