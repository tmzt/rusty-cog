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
        Ok(content) => json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "content": [{
                    "type": "text",
                    "text": serde_json::to_string_pretty(&content).unwrap_or_default()
                }]
            }
        }),
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

async fn dispatch_tool(cli: &Cli, name: &str, args: &Value) -> cog_core::Result<Value> {
    match name {
        // Gmail
        "gmail_search" => {
            let query = arg_str(args, "query")?;
            let max = arg_opt_u32(args, "max");
            let gmail = crate::load_gmail_service(cli).await?;
            let (results, _) = gmail.search(&query, max, None).await?;
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
