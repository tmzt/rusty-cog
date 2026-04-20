//! MCP (Model Context Protocol) server mode.
//!
//! Speaks JSON-RPC 2.0 over stdio. Exposes read-only Gmail, Calendar, and Drive
//! tools backed by the existing cog-core service clients.

use crate::cli::Cli;
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};

pub async fn run_mcp(cli: &Cli) -> cog_core::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = stdout.lock();

    for line in stdin.lock().lines() {
        let line = line.map_err(|e| cog_core::Error::Other(format!("stdin: {e}")))?;
        if line.trim().is_empty() {
            continue;
        }

        let req: Value = serde_json::from_str(&line)
            .map_err(|e| cog_core::Error::Other(format!("json parse: {e}")))?;

        let id = req.get("id").cloned();
        let method = req.get("method").and_then(|m| m.as_str()).unwrap_or("");
        let params = req.get("params").cloned().unwrap_or(json!({}));

        // Notifications (no id) — acknowledge silently
        if id.is_none() {
            continue;
        }

        let response = match method {
            "initialize" => handle_initialize(id.clone()),
            "tools/list" => handle_tools_list(id.clone()),
            "tools/call" => handle_tools_call(cli, id.clone(), &params).await,
            _ => jsonrpc_error(id.clone(), -32601, &format!("unknown method: {method}")),
        };

        let resp_str = serde_json::to_string(&response)
            .map_err(|e| cog_core::Error::Other(format!("json serialize: {e}")))?;
        writeln!(out, "{resp_str}")
            .map_err(|e| cog_core::Error::Other(format!("stdout: {e}")))?;
        out.flush()
            .map_err(|e| cog_core::Error::Other(format!("stdout flush: {e}")))?;
    }

    Ok(())
}

fn handle_initialize(id: Option<Value>) -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": {
            "protocolVersion": "2024-11-05",
            "capabilities": { "tools": {} },
            "serverInfo": {
                "name": "cog",
                "version": env!("CARGO_PKG_VERSION")
            }
        }
    })
}

fn handle_tools_list(id: Option<Value>) -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": { "tools": tool_definitions() }
    })
}

async fn handle_tools_call(cli: &Cli, id: Option<Value>, params: &Value) -> Value {
    let name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let args = params.get("arguments").cloned().unwrap_or(json!({}));

    let result = dispatch_tool(cli, name, &args).await;

    match result {
        Ok(result) => {
            let text = format_markdown(name, &result.data);
            json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "content": [{ "type": "text", "text": text }]
                }
            })
        }
        Err(e) => json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "content": [{ "type": "text", "text": format!("error: {e}") }],
                "isError": true
            }
        }),
    }
}

/// Result from a tool dispatch — carries both structured JSON and raw data.
pub struct ToolResult {
    /// Structured JSON (for TKV/VM consumption)
    pub data: Value,
}

pub async fn dispatch_tool(cli: &Cli, name: &str, args: &Value) -> cog_core::Result<ToolResult> {
    let data = dispatch_tool_json(cli, name, args).await?;
    Ok(ToolResult { data })
}

async fn dispatch_tool_json(cli: &Cli, name: &str, args: &Value) -> cog_core::Result<Value> {
    match name {
        // Gmail
        "gmail_search" => {
            let query = arg_str(args, "query")?;
            let max = arg_opt_u32(args, "max").or(Some(5));
            let gmail = crate::load_gmail_service(cli).await?;
            // Use messages_search for full payloads (from, date, snippet)
            let results = gmail.messages_search(&query, max).await?;
            Ok(serde_json::to_value(results).unwrap_or_default())
        }
        "gmail_get" => {
            let message_id = arg_str(args, "message_id")?;
            let gmail = crate::load_gmail_service(cli).await?;
            let msg = gmail.get(&message_id).await?;
            Ok(serde_json::to_value(msg).unwrap_or_default())
        }
        "gmail_thread_get" => {
            let thread_id = arg_str(args, "thread_id")?;
            let gmail = crate::load_gmail_service(cli).await?;
            let thread = gmail.thread_get(&thread_id).await?;
            Ok(serde_json::to_value(thread).unwrap_or_default())
        }
        "gmail_labels_list" => {
            let gmail = crate::load_gmail_service(cli).await?;
            let labels = gmail.labels_list().await?;
            Ok(serde_json::to_value(labels).unwrap_or_default())
        }

        // Calendar
        "calendar_list" => {
            let cal = crate::load_calendar_service(cli).await?;
            let (calendars, _) = cal.calendars(None).await?;
            Ok(serde_json::to_value(calendars).unwrap_or_default())
        }
        "calendar_events" => {
            let cal = crate::load_calendar_service(cli).await?;
            let calendar_id = args.get("calendar_id").and_then(|v| v.as_str()).unwrap_or("primary");
            let from = args.get("from").and_then(|v| v.as_str());
            let to = args.get("to").and_then(|v| v.as_str());
            let max = arg_opt_u32(args, "max");
            let (events, _) = cal.events(calendar_id, from, to, max, None).await?;
            Ok(serde_json::to_value(events).unwrap_or_default())
        }
        "calendar_event_get" => {
            let calendar_id = arg_str(args, "calendar_id")?;
            let event_id = arg_str(args, "event_id")?;
            let cal = crate::load_calendar_service(cli).await?;
            let event = cal.event_get(&calendar_id, &event_id).await?;
            Ok(serde_json::to_value(event).unwrap_or_default())
        }
        "calendar_search" => {
            let query = arg_str(args, "query")?;
            let max = arg_opt_u32(args, "max");
            let cal = crate::load_calendar_service(cli).await?;
            let results = cal.search("primary", &query, None, None, max).await?;
            Ok(serde_json::to_value(results).unwrap_or_default())
        }

        // Drive
        "drive_list" => {
            let drive = crate::load_drive_service(cli).await?;
            let max = arg_opt_u32(args, "max");
            let parent = args.get("parent").and_then(|v| v.as_str());
            let (files, _) = drive.list(parent, max, None, None).await?;
            Ok(serde_json::to_value(files).unwrap_or_default())
        }
        "drive_search" => {
            let query = arg_str(args, "query")?;
            let max = arg_opt_u32(args, "max");
            let drive = crate::load_drive_service(cli).await?;
            let (files, _) = drive.search(&query, max, None).await?;
            Ok(serde_json::to_value(files).unwrap_or_default())
        }
        "drive_get" => {
            let file_id = arg_str(args, "file_id")?;
            let drive = crate::load_drive_service(cli).await?;
            let file = drive.get(&file_id).await?;
            Ok(serde_json::to_value(file).unwrap_or_default())
        }

        _ => Err(cog_core::Error::Other(format!("unknown tool: {name}"))),
    }
}

fn tool_definitions() -> Value {
    json!([
        // Gmail
        {
            "name": "gmail_search",
            "description": "Search emails by query (Gmail search syntax)",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "query": { "type": "string", "description": "Gmail search query (e.g. 'is:unread', 'from:alice')" },
                    "max": { "type": "integer", "description": "Maximum results to return" }
                },
                "required": ["query"]
            }
        },
        {
            "name": "gmail_get",
            "description": "Get a single email message by ID",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "message_id": { "type": "string", "description": "Message ID" }
                },
                "required": ["message_id"]
            }
        },
        {
            "name": "gmail_thread_get",
            "description": "Get a full email thread (all messages in conversation)",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "thread_id": { "type": "string", "description": "Thread ID" }
                },
                "required": ["thread_id"]
            }
        },
        {
            "name": "gmail_labels_list",
            "description": "List all Gmail labels",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        },
        // Calendar
        {
            "name": "calendar_list",
            "description": "List all calendars",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        },
        {
            "name": "calendar_events",
            "description": "List calendar events",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "calendar_id": { "type": "string", "description": "Calendar ID (default: primary)" },
                    "days": { "type": "integer", "description": "Number of days ahead to list" },
                    "from": { "type": "string", "description": "Start date (ISO 8601)" },
                    "to": { "type": "string", "description": "End date (ISO 8601)" }
                }
            }
        },
        {
            "name": "calendar_event_get",
            "description": "Get a single calendar event",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "calendar_id": { "type": "string", "description": "Calendar ID" },
                    "event_id": { "type": "string", "description": "Event ID" }
                },
                "required": ["calendar_id", "event_id"]
            }
        },
        {
            "name": "calendar_search",
            "description": "Search calendar events by text query",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "query": { "type": "string", "description": "Search query" },
                    "max": { "type": "integer", "description": "Maximum results" }
                },
                "required": ["query"]
            }
        },
        // Drive
        {
            "name": "drive_list",
            "description": "List files in Google Drive",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "max": { "type": "integer", "description": "Maximum results" },
                    "parent": { "type": "string", "description": "Parent folder ID (default: root)" }
                }
            }
        },
        {
            "name": "drive_search",
            "description": "Search files in Google Drive",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "query": { "type": "string", "description": "Search query" },
                    "max": { "type": "integer", "description": "Maximum results" }
                },
                "required": ["query"]
            }
        },
        {
            "name": "drive_get",
            "description": "Get file metadata from Google Drive",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "file_id": { "type": "string", "description": "File ID" }
                },
                "required": ["file_id"]
            }
        }
    ])
}

// ── Markdown formatting ──────────────────────────────────────────────

/// Format a tool result as LLM-friendly markdown.
/// Falls back to pretty JSON for unknown tools.
pub fn format_markdown(tool_name: &str, data: &Value) -> String {
    match tool_name {
        "gmail_search" => format_gmail_messages(data),
        "gmail_get" => format_gmail_message(data),
        "gmail_thread_get" => format_gmail_thread(data),
        "gmail_labels_list" => format_gmail_labels(data),
        "calendar_list" => format_calendar_list(data),
        "calendar_events" => format_calendar_events(data),
        "calendar_event_get" => format_calendar_event(data),
        "calendar_search" => format_calendar_events(data),
        "drive_list" | "drive_search" => format_drive_files(data),
        "drive_get" => format_drive_file(data),
        _ => serde_json::to_string_pretty(data).unwrap_or_default(),
    }
}

fn header_value(headers: &Value, name: &str) -> String {
    headers.as_array()
        .and_then(|arr| arr.iter().find(|h| {
            h.get("name").and_then(|n| n.as_str()) == Some(name)
        }))
        .and_then(|h| h.get("value").and_then(|v| v.as_str()))
        .unwrap_or("")
        .to_string()
}

fn str_field(v: &Value, field: &str) -> String {
    v.get(field).and_then(|f| f.as_str()).unwrap_or("").to_string()
}

fn format_gmail_message(msg: &Value) -> String {
    let id = str_field(msg, "id");
    let snippet = str_field(msg, "snippet");
    let labels: Vec<&str> = msg.get("labelIds")
        .and_then(|l| l.as_array())
        .map(|a| a.iter().filter_map(|v| v.as_str()).collect())
        .unwrap_or_default();

    let headers = msg.pointer("/payload/headers").cloned().unwrap_or(json!([]));
    let from = header_value(&headers, "From");
    let subject = header_value(&headers, "Subject");
    let date = header_value(&headers, "Date");

    // Try to extract body text
    let body = extract_body_text(msg.get("payload"));

    let mut out = format!("## {}\n\n", if subject.is_empty() { "(no subject)" } else { &subject });
    if !from.is_empty() { out.push_str(&format!("**From:** {from}\n")); }
    if !date.is_empty() { out.push_str(&format!("**Date:** {date}\n")); }
    if !labels.is_empty() { out.push_str(&format!("**Labels:** {}\n", labels.join(", "))); }
    out.push_str(&format!("**ID:** {id}\n"));
    out.push('\n');
    if !body.is_empty() {
        out.push_str(&body);
        out.push('\n');
    } else if !snippet.is_empty() {
        out.push_str(&snippet);
        out.push('\n');
    }
    out
}

fn format_gmail_messages(data: &Value) -> String {
    let arr = match data.as_array() {
        Some(a) if !a.is_empty() => a,
        _ => return "No messages found.\n".to_string(),
    };
    let mut out = format!("# Inbox ({})\n\n", arr.len());
    for msg in arr {
        let headers = msg.pointer("/payload/headers").cloned().unwrap_or(json!([]));
        let from = header_value(&headers, "From");
        let subject = header_value(&headers, "Subject");
        let date = header_value(&headers, "Date");
        let snippet = str_field(msg, "snippet");

        // Clean up from: "Name <email>" → "Name"
        let from_display = if let Some(angle) = from.find('<') {
            from[..angle].trim().trim_matches('"').to_string()
        } else {
            from.clone()
        };

        // Clean up date: "Sun, 20 Apr 2025 10:30:00 -0400" → "Apr 20, 10:30"
        let date_short = shorten_date(&date);

        let subj = if subject.is_empty() { "(no subject)" } else { &subject };
        out.push_str(&format!("**{}**", subj));
        if !from_display.is_empty() {
            out.push_str(&format!(" — {}", from_display));
        }
        if !date_short.is_empty() {
            out.push_str(&format!("  *{}*", date_short));
        }
        out.push('\n');
        if !snippet.is_empty() {
            // Trim snippet to ~80 chars
            let s = if snippet.len() > 80 { &snippet[..80] } else { &snippet };
            out.push_str(&format!("  {}\n", s));
        }
        out.push('\n');
    }
    out
}

fn shorten_date(date: &str) -> String {
    // "Sun, 20 Apr 2025 10:30:00 -0400" → "Apr 20, 10:30"
    let parts: Vec<&str> = date.split_whitespace().collect();
    if parts.len() >= 5 {
        let day = parts.get(1).unwrap_or(&"");
        let month = parts.get(2).unwrap_or(&"");
        let time = parts.get(4).unwrap_or(&"");
        let time_short = if time.len() >= 5 { &time[..5] } else { time };
        format!("{} {}, {}", month, day, time_short)
    } else {
        date.to_string()
    }
}

fn format_gmail_thread(data: &Value) -> String {
    let id = str_field(data, "id");
    let snippet = str_field(data, "snippet");
    let messages = data.get("messages").and_then(|m| m.as_array());
    let count = messages.map(|m| m.len()).unwrap_or(0);

    let mut out = format!("# Thread {id} ({count} message(s))\n\n");
    if !snippet.is_empty() { out.push_str(&format!("> {snippet}\n\n")); }
    if let Some(msgs) = messages {
        for msg in msgs {
            out.push_str(&format_gmail_message(msg));
            out.push_str("---\n\n");
        }
    }
    out
}

fn format_gmail_labels(data: &Value) -> String {
    let arr = match data.as_array() {
        Some(a) => a,
        None => return "No labels found.\n".to_string(),
    };
    let mut out = format!("# Gmail Labels ({} total)\n\n", arr.len());
    out.push_str("| Label | Type | Messages | Unread |\n");
    out.push_str("|-------|------|----------|--------|\n");
    for label in arr {
        let name = str_field(label, "name");
        let ltype = str_field(label, "type");
        let total = label.get("messagesTotal").and_then(|v| v.as_i64()).unwrap_or(0);
        let unread = label.get("messagesUnread").and_then(|v| v.as_i64()).unwrap_or(0);
        out.push_str(&format!("| {name} | {ltype} | {total} | {unread} |\n"));
    }
    out
}

fn format_calendar_event(evt: &Value) -> String {
    let summary = str_field(evt, "summary");
    let status = str_field(evt, "status");
    let location = str_field(evt, "location");
    let description = str_field(evt, "description");

    let start = evt.get("start")
        .and_then(|s| s.get("dateTime").or(s.get("date")))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let end = evt.get("end")
        .and_then(|s| s.get("dateTime").or(s.get("date")))
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let mut out = format!("## {}\n\n", if summary.is_empty() { "(untitled)" } else { &summary });
    out.push_str(&format!("- **When:** {start}"));
    if !end.is_empty() { out.push_str(&format!(" to {end}")); }
    out.push('\n');
    if !location.is_empty() { out.push_str(&format!("- **Where:** {location}\n")); }
    if !status.is_empty() { out.push_str(&format!("- **Status:** {status}\n")); }

    if let Some(attendees) = evt.get("attendees").and_then(|a| a.as_array()) {
        let names: Vec<String> = attendees.iter().map(|a| {
            let email = str_field(a, "email");
            let name = str_field(a, "displayName");
            let resp = str_field(a, "responseStatus");
            if !name.is_empty() {
                format!("{name} <{email}> ({resp})")
            } else {
                format!("{email} ({resp})")
            }
        }).collect();
        out.push_str(&format!("- **Attendees:** {}\n", names.join(", ")));
    }

    if !description.is_empty() {
        out.push_str(&format!("\n{description}\n"));
    }
    out
}

fn format_calendar_events(data: &Value) -> String {
    let arr = match data.as_array() {
        Some(a) if !a.is_empty() => a,
        _ => return "No events found.\n".to_string(),
    };
    let mut out = format!("# Calendar: {} event(s)\n\n", arr.len());
    for evt in arr {
        out.push_str(&format_calendar_event(evt));
        out.push('\n');
    }
    out
}

fn format_calendar_list(data: &Value) -> String {
    let arr = match data.as_array() {
        Some(a) => a,
        None => return "No calendars found.\n".to_string(),
    };
    let mut out = format!("# Calendars ({} total)\n\n", arr.len());
    for cal in arr {
        let summary = str_field(cal, "summary");
        let id = str_field(cal, "id");
        let role = str_field(cal, "accessRole");
        let primary = cal.get("primary").and_then(|v| v.as_bool()).unwrap_or(false);
        let marker = if primary { " **(primary)**" } else { "" };
        out.push_str(&format!("- **{summary}**{marker} — `{id}` ({role})\n"));
    }
    out
}

fn mime_type_label(mime: &str) -> &str {
    match mime {
        "application/vnd.google-apps.document" => "Doc",
        "application/vnd.google-apps.spreadsheet" => "Sheet",
        "application/vnd.google-apps.presentation" => "Slides",
        "application/vnd.google-apps.folder" => "Folder",
        "application/vnd.google-apps.form" => "Form",
        "application/pdf" => "PDF",
        m if m.starts_with("image/") => "Image",
        m if m.starts_with("video/") => "Video",
        m if m.starts_with("audio/") => "Audio",
        _ => "File",
    }
}

fn format_drive_file(file: &Value) -> String {
    let name = str_field(file, "name");
    let id = str_field(file, "id");
    let mime = str_field(file, "mimeType");
    let modified = str_field(file, "modifiedTime");
    let size = str_field(file, "size");
    let link = str_field(file, "webViewLink");
    let desc = str_field(file, "description");
    let label = mime_type_label(&mime);

    let mut out = format!("## {} [{}]\n\n", if name.is_empty() { "(untitled)" } else { &name }, label);
    out.push_str(&format!("- **ID:** {id}\n"));
    out.push_str(&format!("- **Type:** {mime}\n"));
    if !modified.is_empty() { out.push_str(&format!("- **Modified:** {modified}\n")); }
    if !size.is_empty() { out.push_str(&format!("- **Size:** {} bytes\n", size)); }
    if !link.is_empty() { out.push_str(&format!("- **Link:** {link}\n")); }
    if !desc.is_empty() { out.push_str(&format!("\n{desc}\n")); }
    out
}

fn format_drive_files(data: &Value) -> String {
    let arr = match data.as_array() {
        Some(a) if !a.is_empty() => a,
        _ => return "No files found.\n".to_string(),
    };
    let mut out = format!("# Drive: {} file(s)\n\n", arr.len());
    out.push_str("| Name | Type | Modified | Size |\n");
    out.push_str("|------|------|----------|------|\n");
    for file in arr {
        let name = str_field(file, "name");
        let mime = str_field(file, "mimeType");
        let modified = str_field(file, "modifiedTime");
        let size = str_field(file, "size");
        let label = mime_type_label(&mime);
        let size_display = if size.is_empty() { "-".to_string() } else { format!("{} B", size) };
        let mod_display = if modified.is_empty() { "-".to_string() } else {
            modified.split('T').next().unwrap_or(&modified).to_string()
        };
        out.push_str(&format!("| {name} | {label} | {mod_display} | {size_display} |\n"));
    }
    out
}

/// Extract plain text body from a Gmail message payload (recursive multipart).
fn extract_body_text(payload: Option<&Value>) -> String {
    let payload = match payload {
        Some(p) => p,
        None => return String::new(),
    };

    // Check for direct text/plain body
    let mime = payload.get("mimeType").and_then(|m| m.as_str()).unwrap_or("");
    if mime == "text/plain" {
        if let Some(data) = payload.pointer("/body/data").and_then(|d| d.as_str()) {
            return decode_base64url(data);
        }
    }

    // Recurse into parts
    if let Some(parts) = payload.get("parts").and_then(|p| p.as_array()) {
        // Prefer text/plain over text/html
        for part in parts {
            let part_mime = part.get("mimeType").and_then(|m| m.as_str()).unwrap_or("");
            if part_mime == "text/plain" {
                if let Some(data) = part.pointer("/body/data").and_then(|d| d.as_str()) {
                    return decode_base64url(data);
                }
            }
        }
        // Fallback: recurse into first multipart
        for part in parts {
            let result = extract_body_text(Some(part));
            if !result.is_empty() { return result; }
        }
    }

    String::new()
}

fn decode_base64url(data: &str) -> String {
    // Gmail uses URL-safe base64 without padding
    let standard = data.replace('-', "+").replace('_', "/");
    match base64::Engine::decode(&base64::engine::general_purpose::STANDARD_NO_PAD, &standard) {
        Ok(bytes) => String::from_utf8_lossy(&bytes).to_string(),
        Err(_) => String::new(),
    }
}

fn jsonrpc_error(id: Option<Value>, code: i32, message: &str) -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": { "code": code, "message": message }
    })
}

fn arg_str(args: &Value, key: &str) -> cog_core::Result<String> {
    args.get(key)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| cog_core::Error::Other(format!("missing required argument: {key}")))
}

fn arg_opt_u32(args: &Value, key: &str) -> Option<u32> {
    args.get(key).and_then(|v| v.as_u64()).map(|n| n as u32)
}
