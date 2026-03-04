//! Google Calendar API service client.
//!
//! Wraps the Calendar REST API v3.
//! <https://developers.google.com/calendar/api/v3/reference>

use crate::error::{Error, Result};
use crate::http::HttpClient;
use crate::types::calendar::*;
use serde::Deserialize;

const BASE: &str = "https://www.googleapis.com/calendar/v3";

// ---------------------------------------------------------------------------
// API list-response wrappers (not part of the public types)
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListEventsResponse {
    #[serde(default)]
    items: Vec<Event>,
    #[serde(default)]
    next_page_token: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListCalendarsResponse {
    #[serde(default)]
    items: Vec<CalendarListEntry>,
    #[serde(default)]
    next_page_token: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListAclResponse {
    #[serde(default)]
    items: Vec<AclRule>,
    #[serde(default)]
    next_page_token: Option<String>,
}

// ---------------------------------------------------------------------------
// Service
// ---------------------------------------------------------------------------

/// Async client for the Google Calendar API.
#[derive(Debug, Clone)]
pub struct CalendarService {
    http: HttpClient,
    token: String,
    calendar_id: String,
}

impl CalendarService {
    /// Create a new `CalendarService` with the default calendar ("primary").
    pub fn new(http: HttpClient, token: String) -> Self {
        Self {
            http,
            token,
            calendar_id: "primary".to_string(),
        }
    }

    /// Create a `CalendarService` for a specific calendar.
    pub fn with_calendar(http: HttpClient, token: String, calendar_id: String) -> Self {
        Self {
            http,
            token,
            calendar_id,
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{BASE}{path}")
    }

    fn parse<T: serde::de::DeserializeOwned>(&self, bytes: &[u8]) -> Result<T> {
        serde_json::from_slice(bytes).map_err(|e| Error::Other(format!("JSON parse error: {e}")))
    }

    /// Resolve a calendar id argument, falling back to the service default.
    fn resolve_calendar_id<'a>(&'a self, calendar_id: &'a str) -> &'a str {
        if calendar_id.is_empty() {
            &self.calendar_id
        } else {
            calendar_id
        }
    }

    // -- calendars ----------------------------------------------------------

    /// List calendars the authenticated user has access to.
    pub async fn calendars(
        &self,
        page_token: Option<&str>,
    ) -> Result<(Vec<CalendarListEntry>, Option<String>)> {
        let mut url = self.url("/users/me/calendarList");
        let mut sep = '?';
        if let Some(pt) = page_token {
            url.push_str(&format!("{sep}pageToken={}", urlencoding(pt)));
            sep = '&';
        }
        let _ = sep;
        let resp = self.http.get(&url, &self.token).await?;
        let list: ListCalendarsResponse = self.parse(&resp)?;
        Ok((list.items, list.next_page_token))
    }

    // -- events -------------------------------------------------------------

    /// List events on a calendar.
    pub async fn events(
        &self,
        calendar_id: &str,
        time_min: Option<&str>,
        time_max: Option<&str>,
        max_results: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<(Vec<Event>, Option<String>)> {
        let cal_id = self.resolve_calendar_id(calendar_id);
        let encoded_cal = urlencoding(cal_id);
        let mut url = self.url(&format!("/calendars/{encoded_cal}/events"));
        let mut sep = '?';
        if let Some(tmin) = time_min {
            url.push_str(&format!("{sep}timeMin={}", urlencoding(tmin)));
            sep = '&';
        }
        if let Some(tmax) = time_max {
            url.push_str(&format!("{sep}timeMax={}", urlencoding(tmax)));
            sep = '&';
        }
        if let Some(n) = max_results {
            url.push_str(&format!("{sep}maxResults={n}"));
            sep = '&';
        }
        if let Some(pt) = page_token {
            url.push_str(&format!("{sep}pageToken={}", urlencoding(pt)));
            sep = '&';
        }
        // Default to ordering by start time with single events expanded
        url.push_str(&format!("{sep}singleEvents=true&orderBy=startTime"));

        let resp = self.http.get(&url, &self.token).await?;
        let list: ListEventsResponse = self.parse(&resp)?;
        Ok((list.items, list.next_page_token))
    }

    /// Get a single event by ID.
    pub async fn event_get(&self, calendar_id: &str, event_id: &str) -> Result<Event> {
        let cal_id = self.resolve_calendar_id(calendar_id);
        let url = self.url(&format!(
            "/calendars/{}/events/{}",
            urlencoding(cal_id),
            urlencoding(event_id)
        ));
        let resp = self.http.get(&url, &self.token).await?;
        self.parse(&resp)
    }

    /// Search events on a calendar by free-text query.
    pub async fn search(
        &self,
        calendar_id: &str,
        query: &str,
        time_min: Option<&str>,
        time_max: Option<&str>,
        max_results: Option<u32>,
    ) -> Result<Vec<Event>> {
        let cal_id = self.resolve_calendar_id(calendar_id);
        let encoded_cal = urlencoding(cal_id);
        let mut url = format!(
            "{}?q={}",
            self.url(&format!("/calendars/{encoded_cal}/events")),
            urlencoding(query)
        );
        if let Some(tmin) = time_min {
            url.push_str(&format!("&timeMin={}", urlencoding(tmin)));
        }
        if let Some(tmax) = time_max {
            url.push_str(&format!("&timeMax={}", urlencoding(tmax)));
        }
        if let Some(n) = max_results {
            url.push_str(&format!("&maxResults={n}"));
        }
        url.push_str("&singleEvents=true&orderBy=startTime");

        let resp = self.http.get(&url, &self.token).await?;
        let list: ListEventsResponse = self.parse(&resp)?;
        Ok(list.items)
    }

    /// Create an event on a calendar.
    pub async fn create(&self, calendar_id: &str, event: &serde_json::Value) -> Result<Event> {
        let cal_id = self.resolve_calendar_id(calendar_id);
        let url = self.url(&format!(
            "/calendars/{}/events",
            urlencoding(cal_id)
        ));
        let body = serde_json::to_vec(event)
            .map_err(|e| Error::Other(e.to_string()))?;
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Update an existing event (full replacement via PUT).
    pub async fn update(
        &self,
        calendar_id: &str,
        event_id: &str,
        event: &serde_json::Value,
    ) -> Result<Event> {
        let cal_id = self.resolve_calendar_id(calendar_id);
        let url = self.url(&format!(
            "/calendars/{}/events/{}",
            urlencoding(cal_id),
            urlencoding(event_id)
        ));
        let body = serde_json::to_vec(event)
            .map_err(|e| Error::Other(e.to_string()))?;
        let resp = self.http.put(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Permanently delete an event.
    #[cfg(feature = "destructive-permanent")]
    pub async fn delete(&self, calendar_id: &str, event_id: &str) -> Result<()> {
        let cal_id = self.resolve_calendar_id(calendar_id);
        let url = self.url(&format!(
            "/calendars/{}/events/{}",
            urlencoding(cal_id),
            urlencoding(event_id)
        ));
        self.http.delete(&url, &self.token).await?;
        Ok(())
    }

    /// Quick-add an event using natural language text.
    pub async fn quick_add(&self, calendar_id: &str, text: &str) -> Result<Event> {
        let cal_id = self.resolve_calendar_id(calendar_id);
        let url = format!(
            "{}/calendars/{}/events/quickAdd?text={}",
            BASE,
            urlencoding(cal_id),
            urlencoding(text)
        );
        let resp = self.http.post(&url, &self.token, &[]).await?;
        self.parse(&resp)
    }

    /// Respond to an event (accept, decline, tentative).
    ///
    /// Patches the event's attendees list to set the current user's
    /// response status.
    pub async fn respond(
        &self,
        calendar_id: &str,
        event_id: &str,
        response_status: &str,
    ) -> Result<Event> {
        // Fetch the event to find the current user's attendee entry
        let existing = self.event_get(calendar_id, event_id).await?;

        if existing.attendees.is_empty() {
            return Err(Error::Other(
                "no attendees on event to update response status".into(),
            ));
        }

        let attendees: Vec<serde_json::Value> = existing
            .attendees
            .iter()
            .map(|a| {
                let mut val = serde_json::to_value(a)
                    .unwrap_or_else(|_| serde_json::Value::Null);
                if a.is_self {
                    val["responseStatus"] =
                        serde_json::Value::String(response_status.to_string());
                }
                val
            })
            .collect();

        let cal_id = self.resolve_calendar_id(calendar_id);
        let url = self.url(&format!(
            "/calendars/{}/events/{}",
            urlencoding(cal_id),
            urlencoding(event_id)
        ));
        let patch_body = serde_json::json!({
            "attendees": attendees,
        });
        let body = serde_json::to_vec(&patch_body)
            .map_err(|e| Error::Other(e.to_string()))?;
        let resp = self.http.patch(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    // -- free/busy ----------------------------------------------------------

    /// Query free/busy information for a set of calendars.
    pub async fn freebusy(
        &self,
        time_min: &str,
        time_max: &str,
        calendar_ids: &[String],
    ) -> Result<FreeBusyResponse> {
        let url = self.url("/freeBusy");
        let items: Vec<serde_json::Value> = calendar_ids
            .iter()
            .map(|id| serde_json::json!({ "id": id }))
            .collect();
        let req_body = serde_json::json!({
            "timeMin": time_min,
            "timeMax": time_max,
            "items": items,
        });
        let body = serde_json::to_vec(&req_body)
            .map_err(|e| Error::Other(e.to_string()))?;
        let resp = self.http.post(&url, &self.token, &body).await?;
        self.parse(&resp)
    }

    /// Find time slots when all specified team members are free.
    ///
    /// Queries free/busy data for all emails and computes open windows
    /// within the given time range. The `duration_minutes` parameter
    /// is provided for future filtering once a date/time library is
    /// available; currently all free gaps are returned.
    pub async fn team(
        &self,
        emails: &[String],
        time_min: &str,
        time_max: &str,
        duration_minutes: u32,
    ) -> Result<Vec<serde_json::Value>> {
        let freebusy_resp = self.freebusy(time_min, time_max, emails).await?;

        // Parse the busy intervals from the response
        let calendars = freebusy_resp
            .calendars
            .as_ref()
            .and_then(|c| c.as_object())
            .cloned()
            .unwrap_or_default();

        // Collect all busy intervals across all calendars
        let mut busy_intervals: Vec<(String, String)> = Vec::new();
        for (_email, cal_data) in &calendars {
            if let Some(busy_list) = cal_data.get("busy").and_then(|b| b.as_array()) {
                for interval in busy_list {
                    if let (Some(start), Some(end)) = (
                        interval.get("start").and_then(|s| s.as_str()),
                        interval.get("end").and_then(|e| e.as_str()),
                    ) {
                        busy_intervals.push((start.to_string(), end.to_string()));
                    }
                }
            }
        }

        // Sort busy intervals by start time
        busy_intervals.sort_by(|a, b| a.0.cmp(&b.0));

        // Merge overlapping intervals
        let mut merged: Vec<(String, String)> = Vec::new();
        for interval in busy_intervals {
            if let Some(last) = merged.last_mut() {
                if interval.0 <= last.1 {
                    if interval.1 > last.1 {
                        last.1 = interval.1.clone();
                    }
                    continue;
                }
            }
            merged.push(interval);
        }

        // Build free slots between the merged busy intervals
        let mut free_slots: Vec<serde_json::Value> = Vec::new();
        let mut current_start = time_min.to_string();

        for (busy_start, busy_end) in &merged {
            if *busy_start > current_start {
                free_slots.push(serde_json::json!({
                    "start": current_start,
                    "end": busy_start,
                }));
            }
            if *busy_end > current_start {
                current_start = busy_end.clone();
            }
        }

        // Final slot from last busy end to time_max
        if current_start < time_max.to_string() {
            free_slots.push(serde_json::json!({
                "start": current_start,
                "end": time_max,
            }));
        }

        let _ = duration_minutes;
        Ok(free_slots)
    }

    // -- colors -------------------------------------------------------------

    /// Get calendar color definitions.
    pub async fn colors(&self) -> Result<CalendarColor> {
        let url = self.url("/colors");
        let resp = self.http.get(&url, &self.token).await?;
        self.parse(&resp)
    }

    // -- ACL ----------------------------------------------------------------

    /// List access control list entries for a calendar.
    pub async fn acl(
        &self,
        calendar_id: &str,
        page_token: Option<&str>,
    ) -> Result<(Vec<AclRule>, Option<String>)> {
        let cal_id = self.resolve_calendar_id(calendar_id);
        let mut url = self.url(&format!(
            "/calendars/{}/acl",
            urlencoding(cal_id)
        ));
        let mut sep = '?';
        if let Some(pt) = page_token {
            url.push_str(&format!("{sep}pageToken={}", urlencoding(pt)));
            sep = '&';
        }
        let _ = sep;
        let resp = self.http.get(&url, &self.token).await?;
        let list: ListAclResponse = self.parse(&resp)?;
        Ok((list.items, list.next_page_token))
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn urlencoding(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}
