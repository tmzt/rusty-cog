use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Calendar {
    pub id: String,
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub location: Option<String>,
    #[serde(default)]
    pub time_zone: Option<String>,
    #[serde(default)]
    pub etag: Option<String>,
    #[serde(default)]
    pub conference_properties: Option<ConferenceProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConferenceProperties {
    #[serde(default)]
    pub allowed_conference_solution_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarListEntry {
    pub id: String,
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub location: Option<String>,
    #[serde(default)]
    pub time_zone: Option<String>,
    #[serde(default)]
    pub color_id: Option<String>,
    #[serde(default)]
    pub background_color: Option<String>,
    #[serde(default)]
    pub foreground_color: Option<String>,
    #[serde(default)]
    pub hidden: bool,
    #[serde(default)]
    pub selected: bool,
    #[serde(default)]
    pub primary: bool,
    #[serde(default)]
    pub deleted: bool,
    #[serde(default)]
    pub access_role: Option<String>,
    #[serde(default)]
    pub summary_override: Option<String>,
    #[serde(default)]
    pub default_reminders: Vec<EventReminder>,
    #[serde(default)]
    pub notification_settings: Option<NotificationSettings>,
    #[serde(default)]
    pub etag: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationSettings {
    #[serde(default)]
    pub notifications: Vec<CalendarNotification>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarNotification {
    #[serde(default)]
    pub method: Option<String>,
    #[serde(rename = "type", default)]
    pub notification_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub html_link: Option<String>,
    #[serde(default)]
    pub created: Option<String>,
    #[serde(default)]
    pub updated: Option<String>,
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub location: Option<String>,
    #[serde(default)]
    pub color_id: Option<String>,
    #[serde(default)]
    pub creator: Option<EventPerson>,
    #[serde(default)]
    pub organizer: Option<EventPerson>,
    #[serde(default)]
    pub start: Option<EventDateTime>,
    #[serde(default)]
    pub end: Option<EventDateTime>,
    #[serde(default)]
    pub original_start_time: Option<EventDateTime>,
    #[serde(default)]
    pub recurring_event_id: Option<String>,
    #[serde(default)]
    pub recurrence: Vec<String>,
    #[serde(default)]
    pub transparency: Option<String>,
    #[serde(default)]
    pub visibility: Option<String>,
    #[serde(default)]
    pub ical_uid: Option<String>,
    #[serde(default)]
    pub sequence: Option<i64>,
    #[serde(default)]
    pub attendees: Vec<EventAttendee>,
    #[serde(default)]
    pub hangout_link: Option<String>,
    #[serde(default)]
    pub conference_data: Option<ConferenceData>,
    #[serde(default)]
    pub reminders: Option<EventReminders>,
    #[serde(default)]
    pub event_type: Option<String>,
    #[serde(default)]
    pub etag: Option<String>,
    #[serde(rename = "kind", default)]
    pub kind: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventPerson {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default, rename = "self")]
    pub is_self: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventDateTime {
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub date_time: Option<String>,
    #[serde(default)]
    pub time_zone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventAttendee {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub organizer: bool,
    #[serde(default, rename = "self")]
    pub is_self: bool,
    #[serde(default)]
    pub resource: bool,
    #[serde(default)]
    pub optional: bool,
    #[serde(default)]
    pub response_status: Option<String>,
    #[serde(default)]
    pub comment: Option<String>,
    #[serde(default)]
    pub additional_guests: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventReminder {
    #[serde(default)]
    pub method: Option<String>,
    #[serde(default)]
    pub minutes: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventReminders {
    #[serde(default)]
    pub use_default: bool,
    #[serde(default)]
    pub overrides: Vec<EventReminder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventRecurrence {
    #[serde(default)]
    pub rrule: Option<String>,
    #[serde(default)]
    pub exrule: Option<String>,
    #[serde(default)]
    pub rdate: Option<String>,
    #[serde(default)]
    pub exdate: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConferenceData {
    #[serde(default)]
    pub conference_id: Option<String>,
    #[serde(default)]
    pub conference_solution: Option<ConferenceSolution>,
    #[serde(default)]
    pub entry_points: Vec<EntryPoint>,
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConferenceSolution {
    #[serde(default)]
    pub key: Option<ConferenceSolutionKey>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub icon_uri: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConferenceSolutionKey {
    #[serde(rename = "type", default)]
    pub solution_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryPoint {
    #[serde(default)]
    pub entry_point_type: Option<String>,
    #[serde(default)]
    pub uri: Option<String>,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub pin: Option<String>,
    #[serde(default)]
    pub region_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FreeBusyResponse {
    pub kind: String,
    #[serde(default)]
    pub time_min: Option<String>,
    #[serde(default)]
    pub time_max: Option<String>,
    #[serde(default)]
    pub calendars: Option<serde_json::Value>,
    #[serde(default)]
    pub groups: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarColor {
    #[serde(default)]
    pub calendar: Option<serde_json::Value>,
    #[serde(default)]
    pub event: Option<serde_json::Value>,
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub updated: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Acl {
    pub kind: String,
    #[serde(default)]
    pub etag: Option<String>,
    #[serde(default)]
    pub next_page_token: Option<String>,
    #[serde(default)]
    pub next_sync_token: Option<String>,
    #[serde(default)]
    pub items: Vec<AclRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AclRule {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub etag: Option<String>,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub scope: Option<AclScope>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AclScope {
    #[serde(rename = "type", default)]
    pub scope_type: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
}
