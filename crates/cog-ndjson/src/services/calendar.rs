use serde::{Deserialize, Serialize};

/// Calendar service protocol requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum CalendarRequest {
    Calendars,
    Events {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        calendar_id: Option<String>,
        #[serde(default)]
        today: bool,
        #[serde(default)]
        tomorrow: bool,
        #[serde(default)]
        week: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        days: Option<u32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        from: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        to: Option<String>,
        #[serde(default)]
        all: bool,
        #[serde(default)]
        calendars: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        week_start: Option<String>,
    },
    EventGet {
        calendar_id: String,
        event_id: String,
    },
    Search {
        query: String,
        #[serde(default)]
        today: bool,
        #[serde(default)]
        tomorrow: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        days: Option<u32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        from: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        to: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max: Option<u32>,
    },
    Create {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        calendar_id: Option<String>,
        summary: String,
        from: String,
        to: String,
        #[serde(default)]
        attendees: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        location: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        send_updates: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        rrule: Option<String>,
        #[serde(default)]
        reminders: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        event_type: Option<String>,
        #[serde(default)]
        all_day: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        working_location_type: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        working_office_label: Option<String>,
    },
    Update {
        calendar_id: String,
        event_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        summary: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        from: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        to: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        send_updates: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        add_attendee: Option<String>,
    },
    #[cfg(feature = "destructive-permanent")]
    Delete {
        calendar_id: String,
        event_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        send_updates: Option<String>,
    },
    Respond {
        calendar_id: String,
        event_id: String,
        status: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        send_updates: Option<String>,
    },
    ProposeTimes {
        calendar_id: String,
        event_id: String,
        #[serde(default)]
        open: bool,
        #[serde(default)]
        decline: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
    },
    FreeBusy {
        #[serde(default)]
        calendars: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        from: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        to: Option<String>,
    },
    Conflicts {
        #[serde(default)]
        calendars: Vec<String>,
        #[serde(default)]
        today: bool,
    },
    Team {
        group_email: String,
        #[serde(default)]
        today: bool,
        #[serde(default)]
        week: bool,
        #[serde(default)]
        freebusy: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        query: Option<String>,
    },
    Colors,
    Acl {
        calendar_id: String,
    },
    Users,
}
