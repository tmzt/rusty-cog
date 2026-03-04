# COG Protocol v1 Specification

| Field   | Value                          |
|---------|--------------------------------|
| Version | 1.0                            |
| Status  | Draft                          |
| Date    | 2026-02-28                     |
| Project | rusty-cog (Rust port of gogcli)|

---

## 1. Overview

The COG Protocol v1 defines a request/response protocol for programmatic access
to Google Workspace services. It is designed as the primary interface for agents,
automation tools, and local clients to communicate with a long-lived `cog-api`
daemon process.

Key properties:

- **Transport**: Unix Domain Sockets (UDS) only. No TCP, no HTTP.
- **Two wire formats**: NDJSON (text) and binary (postcard), negotiated at
  connection time via handshake.
- **Shared type system**: Both encodings serialize the same Rust enum hierarchy.
  A message valid in one encoding is valid in the other.
- **Request/response with server push**: Clients send requests and receive
  responses. The server may also push unsolicited monitor events to subscribed
  clients.
- **Feature-gated destructive operations**: Permanent-deletion and bulk-trash
  operations are compile-time gated and will be rejected at the protocol level
  when not enabled.

### 1.1 Terminology

| Term | Meaning |
|------|---------|
| Client | Process that connects to the UDS and sends `CogRequest` messages. |
| Server | The `cog-api` daemon listening on the UDS. |
| Frame | A single discrete message on the wire (one NDJSON line or one binary frame). |
| Envelope | The outer `CogRequest` or `CogResponse` wrapper around a payload. |
| Payload | The inner service-specific message carried by an envelope. |
| Monitor event | An unsolicited `CogEvent` pushed from server to client. |

---

## 2. Transport

### 2.1 Unix Domain Socket

All communication occurs over a stream-oriented Unix Domain Socket
(`AF_UNIX`, `SOCK_STREAM`). The socket is created by the server on startup
and removed on clean shutdown.

### 2.2 Socket Location

The server selects the socket path using the following precedence:

1. `$COG_HOME/cog.sock` -- if `$COG_HOME` is set.
2. `~/.config/cog/cog.sock` -- default `COG_HOME` location.
3. `$XDG_RUNTIME_DIR/cog.sock` -- if the above paths are not writable.
4. `/tmp/cog-{uid}.sock` -- final fallback, where `{uid}` is the numeric
   Unix user ID.

Clients MUST use the same resolution logic to locate the socket unless given
an explicit path.

### 2.3 Permissions

The socket file SHOULD be created with mode `0600` (owner read/write only).
The server MUST verify the peer credentials of connecting clients match the
server's UID using `SO_PEERCRED` (Linux) or equivalent.

---

## 3. Connection Lifecycle

```
Client                                  Server
  |                                       |
  |--- connect() ----------------------->|
  |                                       |
  |--- Handshake Request --------------->|
  |<-- Handshake Response ---------------|
  |                                       |
  |--- CogRequest {id:1} -------------->|
  |<-- CogResponse {id:1} --------------|
  |                                       |
  |--- CogRequest {id:2} -------------->|
  |<-- CogEvent (monitor push) ---------|  (if subscribed)
  |<-- CogResponse {id:2} --------------|
  |                                       |
  |--- Shutdown request ---------------->|
  |<-- Shutdown response ----------------|
  |<-- (server completes in-flight) -----|
  |--- (connection closed) ------------->|
```

### 3.1 Phases

1. **Connect**: Client opens a stream connection to the UDS.
2. **Handshake**: Client sends handshake, server validates and responds.
   Encoding is locked after this phase.
3. **Request/Response**: Client sends `CogRequest` messages, server replies
   with `CogResponse` messages. Server may interleave `CogEvent` pushes.
4. **Close**: Either side may close. Client SHOULD send a `Shutdown` request
   for clean teardown.

### 3.2 Multiple Connections

The server MUST accept multiple concurrent client connections. Each connection
has independent handshake state, encoding, and monitor subscriptions.

---

## 4. Handshake

The handshake is always performed in NDJSON (text) format, regardless of the
encoding that will be negotiated. This ensures both sides can bootstrap
without prior agreement.

### 4.1 Client Handshake Request

The client MUST send, as its first message, a single newline-terminated JSON
object:

```json
{"protocol":"cog/1","encoding":"json"}
```

or:

```json
{"protocol":"cog/1","encoding":"binary"}
```

| Field      | Type   | Required | Values               |
|------------|--------|----------|----------------------|
| `protocol` | string | yes      | `"cog/1"`            |
| `encoding` | string | yes      | `"json"` or `"binary"` |

### 4.2 Server Handshake Response

On success:

```json
{"protocol":"cog/1","status":"ok"}
```

On error (unsupported protocol version):

```json
{"protocol":"cog/1","status":"error","message":"unsupported protocol version"}
```

On error (invalid encoding):

```json
{"protocol":"cog/1","status":"error","message":"unsupported encoding"}
```

| Field      | Type   | Required | Values                    |
|------------|--------|----------|---------------------------|
| `protocol` | string | yes      | `"cog/1"`                 |
| `status`   | string | yes      | `"ok"` or `"error"`       |
| `message`  | string | on error | Human-readable explanation |

### 4.3 Post-Handshake

After the server sends a successful handshake response:

- If `encoding` was `"json"`, all subsequent frames use NDJSON format.
- If `encoding` was `"binary"`, all subsequent frames use binary format.

The handshake itself is always NDJSON regardless of negotiated encoding.

If the handshake fails, the server MUST close the connection.

---

## 5. NDJSON Wire Format

### 5.1 Framing

Each frame is a single JSON object serialized on one line, terminated by a
newline character (`0x0A`, `\n`).

```
{"id":1,"payload":{"Gmail":{"Search":{"query":"from:alice"}}}}\n
```

Rules:

- Each line MUST be valid UTF-8.
- Each line MUST be a single, complete JSON object.
- Newline characters within JSON string values MUST be escaped as `\n`
  (two characters: backslash, lowercase n). Raw `0x0A` bytes MUST NOT appear
  within a JSON line.
- Carriage return (`0x0D`, `\r`) before the newline is tolerated but not
  required. Parsers MUST accept both `\n` and `\r\n` line endings.
- Empty lines (zero bytes before `\n`) MUST be silently ignored.

### 5.2 Maximum Line Length

The maximum length of a single NDJSON line (including the terminating newline)
is **16 MiB** (16,777,216 bytes).

If a client sends a line exceeding this limit, the server MUST respond with
error code 2 (`invalid_request`) and MAY close the connection.

### 5.3 Encoding

All NDJSON text MUST be encoded as UTF-8 without BOM.

### 5.4 Streaming Reads

Implementations SHOULD use a buffered reader and scan for `\n` to extract
frames. Partial reads are expected; implementations MUST buffer incomplete
lines until the terminating newline arrives.

---

## 6. Binary Wire Format

### 6.1 Frame Structure

Each binary frame consists of a 4-byte little-endian length prefix followed
by a postcard-encoded payload.

```
+---+---+---+---+-------------------+
| Length (u32 LE) |    Payload        |
| (4 bytes)       | (Length bytes)    |
+---+---+---+---+-------------------+
```

| Offset | Size    | Field   | Description                            |
|--------|---------|---------|----------------------------------------|
| 0      | 4 bytes | Length  | Payload size in bytes, little-endian u32 |
| 4      | Length  | Payload | postcard-serialized Rust enum          |

### 6.2 Maximum Frame Size

The maximum payload size is **16 MiB** (16,777,216 bytes). The length prefix
value MUST NOT exceed `16_777_216`. If it does, the receiver MUST treat this
as a protocol error and close the connection.

The total frame size on the wire is therefore at most 16 MiB + 4 bytes.

### 6.3 Postcard Encoding

The payload is serialized using the [postcard](https://docs.rs/postcard) crate,
a compact `serde`-compatible binary format. The same Rust types used for JSON
serialization are used for postcard serialization.

### 6.4 Byte Order

The 4-byte length prefix is always **little-endian** (`u32::to_le_bytes()`).

### 6.5 Zero-Length Frames

A frame with length 0 (four zero bytes) is a **keepalive** and MUST be silently
ignored by the receiver. Either side MAY send keepalives.

---

## 7. Request Envelope

All client-to-server messages (after handshake) use the `CogRequest` envelope:

```rust
struct CogRequest {
    id: u64,
    payload: RequestPayload,
}
```

### 7.1 JSON Representation

```json
{
  "id": 42,
  "payload": {
    "Gmail": {
      "Search": {
        "query": "is:unread",
        "max_results": 10
      }
    }
  }
}
```

### 7.2 Fields

| Field     | Type             | Required | Description                          |
|-----------|------------------|----------|--------------------------------------|
| `id`      | `u64`            | yes      | Client-assigned request identifier. MUST be unique within a connection. Echoed verbatim in the response. |
| `payload` | `RequestPayload` | yes      | The service-specific request. See section 12 for all variants. |

### 7.3 Request IDs

- The `id` field is an unsigned 64-bit integer assigned by the client.
- The server echoes the `id` in the corresponding `CogResponse`.
- Clients SHOULD use monotonically increasing IDs starting from 1.
- ID 0 is reserved for server-initiated messages (events, unsolicited shutdown).
- The server MUST NOT assume any ordering or uniqueness properties of IDs
  beyond using them to correlate responses.

---

## 8. Response Envelope

All server-to-client responses use the `CogResponse` envelope:

```rust
struct CogResponse {
    id: u64,
    result: ResponseResult,
}

enum ResponseResult {
    Ok(ResponsePayload),
    Err(ErrorResponse),
}
```

### 8.1 JSON Representation (Success)

```json
{
  "id": 42,
  "result": {
    "Ok": {
      "Gmail": {
        "Search": {
          "messages": [
            {"id": "18a1b2c3d4e5f6", "thread_id": "18a1b2c3d4e5f6", "snippet": "Hello..."}
          ],
          "next_page_token": null,
          "result_size_estimate": 1
        }
      }
    }
  }
}
```

### 8.2 JSON Representation (Error)

```json
{
  "id": 42,
  "result": {
    "Err": {
      "code": 3,
      "message": "message not found",
      "details": null
    }
  }
}
```

### 8.3 Fields

| Field    | Type             | Required | Description                                  |
|----------|------------------|----------|----------------------------------------------|
| `id`     | `u64`            | yes      | Echoed from the corresponding `CogRequest`.  |
| `result` | `ResponseResult` | yes      | Either `Ok(ResponsePayload)` or `Err(ErrorResponse)`. |

---

## 9. Error Response

```rust
struct ErrorResponse {
    code: u32,
    message: String,
    details: Option<serde_json::Value>,
}
```

### 9.1 Fields

| Field     | Type            | Required | Description                              |
|-----------|-----------------|----------|------------------------------------------|
| `code`    | `u32`           | yes      | Numeric error code (see table below).    |
| `message` | `String`        | yes      | Human-readable error description.        |
| `details` | `Option<Value>` | no       | Arbitrary JSON with additional context. MAY be `null` or omitted. |

### 9.2 Error Codes

| Code | Name                  | Description                                                |
|------|-----------------------|------------------------------------------------------------|
| 1    | `internal`            | Internal server error. The request could not be processed due to a server-side bug or unexpected condition. |
| 2    | `invalid_request`     | The request is malformed, missing required fields, has invalid field values, or exceeds backpressure limits. |
| 3    | `not_found`           | The requested resource does not exist.                     |
| 4    | `auth_required`       | No valid credentials are available. The client must authenticate first. |
| 5    | `permission_denied`   | Credentials exist but lack the required scopes or permissions for this operation. |
| 6    | `rate_limited`        | The upstream Google API returned a rate-limit error (HTTP 429). The `details` field MAY contain a `retry_after_seconds` value. |
| 7    | `destructive_denied`  | The requested operation requires the `destructive-permanent` feature gate, which was not enabled at compile time. |
| 8    | `bulk_trash_denied`   | The requested operation affects more than 50 resources and requires the `destructive-bulk-trash` feature gate, which was not enabled at compile time. |
| 9    | `feature_disabled`    | The requested feature (e.g., `gemini-web`, `notebooklm`) is not enabled. |
| 10   | `shutdown_in_progress`| The server is shutting down and cannot accept new requests. |

### 9.3 Error Code Ranges

- `1-99`: Protocol and server errors (defined by this specification).
- `100-999`: Reserved for future protocol use.
- `1000+`: Application-specific errors (reserved for extensions).

---

## 10. Server Push -- Monitor Events

When a client has subscribed to monitor events (via `Monitor::Subscribe`),
the server pushes `CogEvent` messages to that client without a preceding
request.

```rust
struct CogEvent {
    event_type: String,
    service: String,
    payload: serde_json::Value,
    timestamp: String,
}
```

### 10.1 JSON Representation

```json
{
  "event_type": "new_message",
  "service": "gmail",
  "payload": {
    "message_id": "18a1b2c3d4e5f6",
    "thread_id": "18a1b2c3d4e5f6",
    "from": "alice@example.com",
    "subject": "Meeting tomorrow",
    "snippet": "Hi, just wanted to confirm..."
  },
  "timestamp": "2026-02-28T15:30:00Z"
}
```

### 10.2 Fields

| Field        | Type     | Required | Description                                  |
|--------------|----------|----------|----------------------------------------------|
| `event_type` | `String` | yes      | Type of event (e.g., `new_message`, `file_changed`, `event_updated`). |
| `service`    | `String` | yes      | Source service name (e.g., `gmail`, `drive`, `calendar`, `keep`). |
| `payload`    | `Value`  | yes      | Service-specific event data. Structure varies by `event_type`. |
| `timestamp`  | `String` | yes      | ISO 8601 timestamp of when the event was detected. |

### 10.3 Event Types

| Service    | Event Type        | Description                           |
|------------|-------------------|---------------------------------------|
| `gmail`    | `new_message`     | New message received.                 |
| `gmail`    | `message_changed` | Message labels or state changed.      |
| `drive`    | `file_changed`    | File created, modified, or trashed.   |
| `drive`    | `file_deleted`    | File permanently deleted.             |
| `calendar` | `event_updated`   | Calendar event created or modified.   |
| `calendar` | `event_deleted`   | Calendar event removed.               |
| `keep`     | `note_updated`    | Note created or modified.             |

### 10.4 Distinguishing Events from Responses

In NDJSON mode, events and responses are both JSON objects on the wire. They
are distinguished by their top-level keys:

- A `CogResponse` always has `"id"` and `"result"` keys.
- A `CogEvent` always has `"event_type"` and `"service"` keys.

In binary mode, they are distinguished by the postcard-encoded enum tag.

Clients MUST inspect incoming frames to determine whether they are responses
or events, as events may arrive interleaved with responses.

---

## 11. Request Payload Envelope

The `RequestPayload` enum routes each request to its target service:

```rust
enum RequestPayload {
    Auth(AuthRequest),
    Gmail(GmailRequest),
    Calendar(CalendarRequest),
    Drive(DriveRequest),
    Docs(DocsRequest),
    Sheets(SheetsRequest),
    Slides(SlidesRequest),
    Forms(FormsRequest),
    Contacts(ContactsRequest),
    Tasks(TasksRequest),
    People(PeopleRequest),
    Chat(ChatRequest),
    Classroom(ClassroomRequest),
    Groups(GroupsRequest),
    Keep(KeepRequest),
    AppScript(AppScriptRequest),
    #[cfg(feature = "gemini-web")]
    Gemini(GeminiRequest),
    #[cfg(feature = "notebooklm")]
    NotebookLm(NotebookLmRequest),
    Monitor(MonitorRequest),
    Index(IndexRequest),
    Ping,
    Shutdown { reason: Option<String> },
}
```

The `ResponsePayload` enum mirrors this structure. Each service defines its
own response variants. See section 12 for the full catalog.

---

## 12. Per-Service Message Catalogs

This section defines every request and response variant for each service.
Fields marked with `?` are optional (`Option<T>` in Rust). Variants marked
with `**` require the `destructive-permanent` compile-time feature gate.
Variants marked with `***` require the `destructive-bulk-trash` feature gate.

---

### 12.1 Auth Service

Manages OAuth2 credentials, service accounts, and keyring operations.

#### AuthRequest

```rust
enum AuthRequest {
    Login {
        scopes: Option<Vec<String>>,
        client: Option<String>,
        force: Option<bool>,
    },
    Status,
    List,
    Remove {
        account: String,
    },
    Credentials {
        account: Option<String>,
    },
    ServiceAccount {
        key_file: String,
        scopes: Option<Vec<String>>,
    },
    Keyring {
        action: KeyringAction,
        account: String,
    },
    Alias {
        action: AliasAction,
        alias: String,
        account: Option<String>,
    },
}

enum KeyringAction {
    Get,
    Set,
    Delete,
}

enum AliasAction {
    Set,
    Remove,
    List,
}
```

| Variant          | Fields                                                        | Description                                |
|------------------|---------------------------------------------------------------|--------------------------------------------|
| `Login`          | `scopes?: Vec<String>`, `client?: String`, `force?: bool`     | Initiate OAuth2 login flow.                |
| `Status`         | (none)                                                        | Show current authentication status.        |
| `List`           | (none)                                                        | List all authenticated accounts.           |
| `Remove`         | `account: String`                                             | Remove credentials for an account.         |
| `Credentials`    | `account?: String`                                            | Get raw credential info for an account.    |
| `ServiceAccount` | `key_file: String`, `scopes?: Vec<String>`                    | Authenticate with a service account key.   |
| `Keyring`        | `action: KeyringAction`, `account: String`                    | Manage keyring entries (get/set/delete).    |
| `Alias`          | `action: AliasAction`, `alias: String`, `account?: String`    | Manage account aliases.                    |

#### AuthResponse

```rust
enum AuthResponse {
    Login { account: String, scopes: Vec<String> },
    Status { account: String, valid: bool, scopes: Vec<String>, expires_at: Option<String> },
    List { accounts: Vec<AccountInfo> },
    Remove { success: bool },
    Credentials { token_type: String, access_token: String, expires_at: Option<String> },
    ServiceAccount { account: String },
    Keyring { value: Option<String> },
    Alias { aliases: Vec<AliasInfo> },
}

struct AccountInfo {
    email: String,
    scopes: Vec<String>,
    expires_at: Option<String>,
    is_service_account: bool,
}

struct AliasInfo {
    alias: String,
    account: String,
}
```

---

### 12.2 Gmail Service

Covers message search, threads, sending, drafts, labels, filters, forwarding,
vacation settings, delegates, push notifications, and history.

#### GmailRequest

```rust
enum GmailRequest {
    Search {
        query: String,
        max_results: Option<u32>,
        page_token: Option<String>,
        label_ids: Option<Vec<String>>,
        include_spam_trash: Option<bool>,
    },
    MessagesSearch {
        query: String,
        max_results: Option<u32>,
        page_token: Option<String>,
        format: Option<MessageFormat>,
    },
    ThreadGet {
        thread_id: String,
        format: Option<MessageFormat>,
    },
    Get {
        message_id: String,
        format: Option<MessageFormat>,
    },
    Send {
        to: Vec<String>,
        cc: Option<Vec<String>>,
        bcc: Option<Vec<String>>,
        subject: String,
        body: String,
        html: Option<bool>,
        reply_to: Option<String>,
        in_reply_to: Option<String>,
        thread_id: Option<String>,
        attachments: Option<Vec<Attachment>>,
    },
    DraftsList {
        max_results: Option<u32>,
        page_token: Option<String>,
    },
    DraftsCreate {
        to: Vec<String>,
        cc: Option<Vec<String>>,
        bcc: Option<Vec<String>>,
        subject: String,
        body: String,
        html: Option<bool>,
    },
    DraftsSend {
        draft_id: String,
    },
    LabelsList,
    LabelsCreate {
        name: String,
        label_list_visibility: Option<String>,
        message_list_visibility: Option<String>,
        color: Option<LabelColor>,
    },
    LabelsDelete {                          // ** destructive-permanent
        label_id: String,
    },
    BatchDelete {                           // ** destructive-permanent
        message_ids: Vec<String>,
    },
    BatchModify {
        message_ids: Vec<String>,
        add_label_ids: Option<Vec<String>>,
        remove_label_ids: Option<Vec<String>>,
    },
    FiltersGet {
        filter_id: Option<String>,
    },
    FiltersCreate {
        criteria: FilterCriteria,
        action: FilterAction,
    },
    FiltersDelete {                         // ** destructive-permanent
        filter_id: String,
    },
    AutoForward {
        action: Option<AutoForwardAction>,
        forwarding_email: Option<String>,
        disposition: Option<String>,
    },
    Vacation {
        action: Option<VacationAction>,
        enable_auto_reply: Option<bool>,
        response_subject: Option<String>,
        response_body_plain_text: Option<String>,
        response_body_html: Option<String>,
        restrict_to_contacts: Option<bool>,
        restrict_to_domain: Option<bool>,
        start_time: Option<String>,
        end_time: Option<String>,
    },
    Delegates {
        action: Option<DelegateAction>,
        delegate_email: Option<String>,
    },
    WatchStart {
        topic_name: String,
        label_ids: Option<Vec<String>>,
    },
    History {
        start_history_id: String,
        max_results: Option<u32>,
        page_token: Option<String>,
        label_id: Option<String>,
        history_types: Option<Vec<String>>,
    },
    Trash {
        message_id: String,
    },
    Modify {
        message_id: String,
        add_label_ids: Option<Vec<String>>,
        remove_label_ids: Option<Vec<String>>,
    },
}

enum MessageFormat {
    Minimal,
    Full,
    Raw,
    Metadata,
}

enum AutoForwardAction { Get, Set, Disable }
enum VacationAction { Get, Set }
enum DelegateAction { List, Add, Remove }
```

| Variant             | Key Fields                                    | Feature Gate               |
|---------------------|-----------------------------------------------|----------------------------|
| `Search`            | `query`, `max_results?`, `page_token?`, `label_ids?`, `include_spam_trash?` | -- |
| `MessagesSearch`    | `query`, `max_results?`, `page_token?`, `format?` | -- |
| `ThreadGet`         | `thread_id`, `format?`                        | --                         |
| `Get`               | `message_id`, `format?`                       | --                         |
| `Send`              | `to`, `subject`, `body`, `cc?`, `bcc?`, `html?`, `reply_to?`, `in_reply_to?`, `thread_id?`, `attachments?` | -- |
| `DraftsList`        | `max_results?`, `page_token?`                 | --                         |
| `DraftsCreate`      | `to`, `subject`, `body`, `cc?`, `bcc?`, `html?` | --                      |
| `DraftsSend`        | `draft_id`                                    | --                         |
| `LabelsList`        | (none)                                        | --                         |
| `LabelsCreate`      | `name`, `label_list_visibility?`, `message_list_visibility?`, `color?` | -- |
| `LabelsDelete`**    | `label_id`                                    | `destructive-permanent`    |
| `BatchDelete`**     | `message_ids`                                 | `destructive-permanent`    |
| `BatchModify`       | `message_ids`, `add_label_ids?`, `remove_label_ids?` | `destructive-bulk-trash` if >50 messages |
| `FiltersGet`        | `filter_id?`                                  | --                         |
| `FiltersCreate`     | `criteria`, `action`                          | --                         |
| `FiltersDelete`**   | `filter_id`                                   | `destructive-permanent`    |
| `AutoForward`       | `action?`, `forwarding_email?`, `disposition?`| --                         |
| `Vacation`          | `action?`, `enable_auto_reply?`, `response_subject?`, `response_body_plain_text?`, `response_body_html?`, etc. | -- |
| `Delegates`         | `action?`, `delegate_email?`                  | --                         |
| `WatchStart`        | `topic_name`, `label_ids?`                    | --                         |
| `History`           | `start_history_id`, `max_results?`, `page_token?`, `label_id?`, `history_types?` | -- |
| `Trash`             | `message_id`                                  | --                         |
| `Modify`            | `message_id`, `add_label_ids?`, `remove_label_ids?` | --                  |

#### GmailResponse

```rust
enum GmailResponse {
    Search {
        messages: Vec<MessageSummary>,
        next_page_token: Option<String>,
        result_size_estimate: u32,
    },
    MessagesSearch {
        messages: Vec<Message>,
        next_page_token: Option<String>,
        result_size_estimate: u32,
    },
    ThreadGet {
        thread: Thread,
    },
    Get {
        message: Message,
    },
    Send {
        message_id: String,
        thread_id: String,
        label_ids: Vec<String>,
    },
    DraftsList {
        drafts: Vec<DraftSummary>,
        next_page_token: Option<String>,
    },
    DraftsCreate {
        draft_id: String,
    },
    DraftsSend {
        message_id: String,
        thread_id: String,
    },
    LabelsList {
        labels: Vec<Label>,
    },
    LabelsCreate {
        label: Label,
    },
    LabelsDelete { success: bool },         // ** destructive-permanent
    BatchDelete { success: bool },          // ** destructive-permanent
    BatchModify { success: bool },
    FiltersGet {
        filters: Vec<Filter>,
    },
    FiltersCreate {
        filter: Filter,
    },
    FiltersDelete { success: bool },        // ** destructive-permanent
    AutoForward {
        enabled: bool,
        email: Option<String>,
        disposition: Option<String>,
    },
    Vacation {
        enabled: bool,
        subject: Option<String>,
        body: Option<String>,
        start_time: Option<String>,
        end_time: Option<String>,
    },
    Delegates {
        delegates: Vec<Delegate>,
    },
    WatchStart {
        history_id: String,
        expiration: String,
    },
    History {
        history: Vec<HistoryRecord>,
        next_page_token: Option<String>,
        history_id: String,
    },
    Trash {
        message: MessageSummary,
    },
    Modify {
        message: MessageSummary,
    },
}
```

---

### 12.3 Calendar Service

Covers calendar listing, event CRUD, RSVP, free/busy queries, team calendars,
colors, and ACLs.

#### CalendarRequest

```rust
enum CalendarRequest {
    Calendars {
        show_hidden: Option<bool>,
        show_deleted: Option<bool>,
    },
    Events {
        calendar_id: Option<String>,
        time_min: Option<String>,
        time_max: Option<String>,
        max_results: Option<u32>,
        page_token: Option<String>,
        order_by: Option<String>,
        single_events: Option<bool>,
        query: Option<String>,
    },
    EventGet {
        calendar_id: Option<String>,
        event_id: String,
    },
    Search {
        query: String,
        calendar_id: Option<String>,
        time_min: Option<String>,
        time_max: Option<String>,
        max_results: Option<u32>,
    },
    Create {
        calendar_id: Option<String>,
        summary: String,
        description: Option<String>,
        location: Option<String>,
        start: EventDateTime,
        end: EventDateTime,
        attendees: Option<Vec<String>>,
        recurrence: Option<Vec<String>>,
        reminders: Option<Reminders>,
        visibility: Option<String>,
        color_id: Option<String>,
        conference: Option<bool>,
    },
    Update {
        calendar_id: Option<String>,
        event_id: String,
        summary: Option<String>,
        description: Option<String>,
        location: Option<String>,
        start: Option<EventDateTime>,
        end: Option<EventDateTime>,
        attendees: Option<Vec<String>>,
        recurrence: Option<Vec<String>>,
        color_id: Option<String>,
    },
    Delete {                               // ** destructive-permanent
        calendar_id: Option<String>,
        event_id: String,
        send_updates: Option<String>,
    },
    Respond {
        calendar_id: Option<String>,
        event_id: String,
        status: RsvpStatus,
        comment: Option<String>,
    },
    FreeBusy {
        time_min: String,
        time_max: String,
        items: Vec<String>,
    },
    Team {
        query: Option<String>,
        max_results: Option<u32>,
    },
    Colors,
    Acl {
        calendar_id: String,
        action: Option<AclAction>,
        scope_type: Option<String>,
        scope_value: Option<String>,
        role: Option<String>,
    },
}

enum RsvpStatus { Accepted, Declined, Tentative }
enum AclAction { List, Insert, Delete }

struct EventDateTime {
    date_time: Option<String>,   // RFC 3339 datetime (e.g., "2026-03-01T09:00:00-05:00")
    date: Option<String>,        // Date only for all-day events (e.g., "2026-03-01")
    time_zone: Option<String>,   // IANA time zone (e.g., "America/New_York")
}

struct Reminders {
    use_default: bool,
    overrides: Option<Vec<ReminderOverride>>,
}

struct ReminderOverride {
    method: String,              // "email" or "popup"
    minutes: u32,
}
```

| Variant      | Key Fields                                                   | Feature Gate            |
|--------------|--------------------------------------------------------------|-------------------------|
| `Calendars`  | `show_hidden?`, `show_deleted?`                              | --                      |
| `Events`     | `calendar_id?`, `time_min?`, `time_max?`, `max_results?`, `page_token?`, `order_by?`, `single_events?`, `query?` | -- |
| `EventGet`   | `calendar_id?`, `event_id`                                   | --                      |
| `Search`     | `query`, `calendar_id?`, `time_min?`, `time_max?`, `max_results?` | --                 |
| `Create`     | `summary`, `start`, `end`, `calendar_id?`, `description?`, `location?`, `attendees?`, `recurrence?`, `reminders?`, `visibility?`, `color_id?`, `conference?` | -- |
| `Update`     | `event_id`, `calendar_id?`, `summary?`, `description?`, `location?`, `start?`, `end?`, `attendees?`, `recurrence?`, `color_id?` | -- |
| `Delete`**   | `event_id`, `calendar_id?`, `send_updates?`                  | `destructive-permanent` |
| `Respond`    | `event_id`, `status`, `calendar_id?`, `comment?`             | --                      |
| `FreeBusy`   | `time_min`, `time_max`, `items`                              | --                      |
| `Team`       | `query?`, `max_results?`                                     | --                      |
| `Colors`     | (none)                                                       | --                      |
| `Acl`        | `calendar_id`, `action?`, `scope_type?`, `scope_value?`, `role?` | --                  |

#### CalendarResponse

```rust
enum CalendarResponse {
    Calendars { calendars: Vec<CalendarEntry> },
    Events { events: Vec<Event>, next_page_token: Option<String> },
    EventGet { event: Event },
    Search { events: Vec<Event> },
    Create { event: Event },
    Update { event: Event },
    Delete { success: bool },               // ** destructive-permanent
    Respond { event: Event },
    FreeBusy { calendars: HashMap<String, Vec<TimePeriod>> },
    Team { calendars: Vec<CalendarEntry> },
    Colors { event_colors: HashMap<String, ColorPair>, calendar_colors: HashMap<String, ColorPair> },
    Acl { rules: Vec<AclRule> },
}

struct CalendarEntry {
    id: String,
    summary: String,
    description: Option<String>,
    time_zone: Option<String>,
    color_id: Option<String>,
    primary: bool,
    selected: bool,
    access_role: String,
}

struct TimePeriod {
    start: String,
    end: String,
}

struct ColorPair {
    background: String,
    foreground: String,
}

struct AclRule {
    id: String,
    scope_type: String,
    scope_value: String,
    role: String,
}
```

---

### 12.4 Drive Service

Covers file listing, search, download, upload, copy, move, trash, permanent
delete, sharing, and shared drives.

#### DriveRequest

```rust
enum DriveRequest {
    List {
        folder_id: Option<String>,
        max_results: Option<u32>,
        page_token: Option<String>,
        order_by: Option<String>,
        fields: Option<String>,
        include_trashed: Option<bool>,
    },
    Search {
        query: String,
        max_results: Option<u32>,
        page_token: Option<String>,
        order_by: Option<String>,
        corpora: Option<String>,
        drive_id: Option<String>,
    },
    Get {
        file_id: String,
        fields: Option<String>,
    },
    Upload {
        name: String,
        parent_id: Option<String>,
        mime_type: Option<String>,
        content_base64: String,
        description: Option<String>,
        starred: Option<bool>,
    },
    Download {
        file_id: String,
        export_mime_type: Option<String>,
        revision_id: Option<String>,
    },
    Copy {
        file_id: String,
        name: Option<String>,
        parent_id: Option<String>,
    },
    Mkdir {
        name: String,
        parent_id: Option<String>,
        description: Option<String>,
    },
    Rename {
        file_id: String,
        name: String,
    },
    Move {
        file_id: String,
        new_parent_id: String,
        remove_parents: Option<bool>,
    },
    Trash {
        file_id: String,
    },
    PermanentDelete {                      // ** destructive-permanent
        file_id: String,
    },
    EmptyTrash,                            // ** destructive-permanent
    Share {
        file_id: String,
        role: String,
        share_type: String,
        email_address: Option<String>,
        domain: Option<String>,
        send_notification: Option<bool>,
        message: Option<String>,
        allow_file_discovery: Option<bool>,
    },
    Unshare {
        file_id: String,
        permission_id: String,
    },
    Permissions {
        file_id: String,
    },
    Drives {
        max_results: Option<u32>,
        page_token: Option<String>,
    },
    BatchTrash {
        file_ids: Vec<String>,
    },
}
```

| Variant             | Key Fields                                                   | Feature Gate            |
|---------------------|--------------------------------------------------------------|-------------------------|
| `List`              | `folder_id?`, `max_results?`, `page_token?`, `order_by?`, `fields?`, `include_trashed?` | -- |
| `Search`            | `query`, `max_results?`, `page_token?`, `order_by?`, `corpora?`, `drive_id?` | -- |
| `Get`               | `file_id`, `fields?`                                         | --                      |
| `Upload`            | `name`, `content_base64`, `parent_id?`, `mime_type?`, `description?`, `starred?` | -- |
| `Download`          | `file_id`, `export_mime_type?`, `revision_id?`               | --                      |
| `Copy`              | `file_id`, `name?`, `parent_id?`                             | --                      |
| `Mkdir`             | `name`, `parent_id?`, `description?`                         | --                      |
| `Rename`            | `file_id`, `name`                                            | --                      |
| `Move`              | `file_id`, `new_parent_id`, `remove_parents?`                | --                      |
| `Trash`             | `file_id`                                                    | --                      |
| `PermanentDelete`** | `file_id`                                                    | `destructive-permanent` |
| `EmptyTrash`**      | (none)                                                       | `destructive-permanent` |
| `Share`             | `file_id`, `role`, `share_type`, `email_address?`, `domain?`, `send_notification?`, `message?`, `allow_file_discovery?` | -- |
| `Unshare`           | `file_id`, `permission_id`                                   | --                      |
| `Permissions`       | `file_id`                                                    | --                      |
| `Drives`            | `max_results?`, `page_token?`                                | --                      |
| `BatchTrash`        | `file_ids`                                                   | `destructive-bulk-trash` if >50 items |

#### DriveResponse

```rust
enum DriveResponse {
    List { files: Vec<FileMetadata>, next_page_token: Option<String> },
    Search { files: Vec<FileMetadata>, next_page_token: Option<String> },
    Get { file: FileMetadata },
    Upload { file: FileMetadata },
    Download { content_base64: String, mime_type: String, name: String },
    Copy { file: FileMetadata },
    Mkdir { file: FileMetadata },
    Rename { file: FileMetadata },
    Move { file: FileMetadata },
    Trash { file: FileMetadata },
    PermanentDelete { success: bool },      // ** destructive-permanent
    EmptyTrash { success: bool },           // ** destructive-permanent
    Share { permission: Permission },
    Unshare { success: bool },
    Permissions { permissions: Vec<Permission> },
    Drives { drives: Vec<SharedDrive>, next_page_token: Option<String> },
    BatchTrash { succeeded: Vec<String>, failed: Vec<BatchError> },
}

struct FileMetadata {
    id: String,
    name: String,
    mime_type: String,
    modified_time: String,
    created_time: Option<String>,
    size: Option<u64>,
    owners: Vec<String>,
    parents: Option<Vec<String>>,
    starred: bool,
    trashed: bool,
    shared: bool,
    web_view_link: Option<String>,
    description: Option<String>,
}

struct Permission {
    id: String,
    role: String,
    permission_type: String,
    email_address: Option<String>,
    domain: Option<String>,
    display_name: Option<String>,
}

struct SharedDrive {
    id: String,
    name: String,
    color_rgb: Option<String>,
    created_time: Option<String>,
}

struct BatchError {
    id: String,
    code: u32,
    message: String,
}
```

---

### 12.5 Docs Service

Covers document info, content retrieval, creation, copying, export, writing,
find-and-replace, sed-style transforms, and tab listing.

#### DocsRequest

```rust
enum DocsRequest {
    Info {
        document_id: String,
    },
    Cat {
        document_id: String,
        tab_id: Option<String>,
    },
    Create {
        title: String,
        body: Option<String>,
        folder_id: Option<String>,
    },
    Copy {
        document_id: String,
        title: Option<String>,
        folder_id: Option<String>,
    },
    Export {
        document_id: String,
        mime_type: Option<String>,
    },
    Write {
        document_id: String,
        text: String,
        index: Option<u32>,
        tab_id: Option<String>,
    },
    FindReplace {
        document_id: String,
        find: String,
        replace: String,
        match_case: Option<bool>,
        tab_id: Option<String>,
    },
    Sed {
        document_id: String,
        expression: String,
        tab_id: Option<String>,
    },
    ListTabs {
        document_id: String,
    },
}
```

| Variant       | Key Fields                                                   | Feature Gate |
|---------------|--------------------------------------------------------------|--------------|
| `Info`        | `document_id`                                                | --           |
| `Cat`         | `document_id`, `tab_id?`                                     | --           |
| `Create`      | `title`, `body?`, `folder_id?`                               | --           |
| `Copy`        | `document_id`, `title?`, `folder_id?`                        | --           |
| `Export`      | `document_id`, `mime_type?`                                  | --           |
| `Write`       | `document_id`, `text`, `index?`, `tab_id?`                   | --           |
| `FindReplace` | `document_id`, `find`, `replace`, `match_case?`, `tab_id?`   | --           |
| `Sed`         | `document_id`, `expression`, `tab_id?`                       | --           |
| `ListTabs`    | `document_id`                                                | --           |

#### DocsResponse

```rust
enum DocsResponse {
    Info {
        document_id: String,
        title: String,
        revision_id: String,
        tabs: Vec<TabInfo>,
    },
    Cat {
        text: String,
    },
    Create {
        document_id: String,
        title: String,
    },
    Copy {
        document_id: String,
        title: String,
    },
    Export {
        content_base64: String,
        mime_type: String,
    },
    Write {
        success: bool,
        writes_applied: u32,
    },
    FindReplace {
        occurrences_changed: u32,
    },
    Sed {
        occurrences_changed: u32,
    },
    ListTabs {
        tabs: Vec<TabInfo>,
    },
}

struct TabInfo {
    tab_id: String,
    title: String,
    index: u32,
}
```

---

### 12.6 Sheets Service

Covers spreadsheet metadata, cell reads, creation, update, append, clear,
formatting, row/column insertion, notes, export, and sheet copying.

#### SheetsRequest

```rust
enum SheetsRequest {
    Metadata {
        spreadsheet_id: String,
    },
    Get {
        spreadsheet_id: String,
        range: String,
        value_render_option: Option<String>,
        date_time_render_option: Option<String>,
    },
    Create {
        title: String,
        sheets: Option<Vec<SheetProperties>>,
        folder_id: Option<String>,
    },
    Update {
        spreadsheet_id: String,
        range: String,
        values: Vec<Vec<serde_json::Value>>,
        value_input_option: Option<String>,
    },
    Append {
        spreadsheet_id: String,
        range: String,
        values: Vec<Vec<serde_json::Value>>,
        value_input_option: Option<String>,
        insert_data_option: Option<String>,
    },
    Clear {
        spreadsheet_id: String,
        range: String,
    },
    Format {
        spreadsheet_id: String,
        requests: Vec<serde_json::Value>,
    },
    Insert {
        spreadsheet_id: String,
        sheet_id: u32,
        dimension: String,
        start_index: u32,
        end_index: u32,
    },
    Notes {
        spreadsheet_id: String,
        range: String,
        note: Option<String>,
    },
    Export {
        spreadsheet_id: String,
        mime_type: Option<String>,
        sheet_id: Option<u32>,
    },
    Copy {
        spreadsheet_id: String,
        sheet_id: u32,
        destination_spreadsheet_id: String,
    },
}

struct SheetProperties {
    title: String,
    index: Option<u32>,
    sheet_type: Option<String>,
    grid_properties: Option<GridProperties>,
}

struct GridProperties {
    row_count: Option<u32>,
    column_count: Option<u32>,
    frozen_row_count: Option<u32>,
    frozen_column_count: Option<u32>,
}
```

| Variant    | Key Fields                                                          | Feature Gate |
|------------|---------------------------------------------------------------------|--------------|
| `Metadata` | `spreadsheet_id`                                                    | --           |
| `Get`      | `spreadsheet_id`, `range`, `value_render_option?`, `date_time_render_option?` | -- |
| `Create`   | `title`, `sheets?`, `folder_id?`                                    | --           |
| `Update`   | `spreadsheet_id`, `range`, `values`, `value_input_option?`          | --           |
| `Append`   | `spreadsheet_id`, `range`, `values`, `value_input_option?`, `insert_data_option?` | -- |
| `Clear`    | `spreadsheet_id`, `range`                                           | --           |
| `Format`   | `spreadsheet_id`, `requests`                                        | --           |
| `Insert`   | `spreadsheet_id`, `sheet_id`, `dimension`, `start_index`, `end_index` | --         |
| `Notes`    | `spreadsheet_id`, `range`, `note?`                                  | --           |
| `Export`   | `spreadsheet_id`, `mime_type?`, `sheet_id?`                         | --           |
| `Copy`     | `spreadsheet_id`, `sheet_id`, `destination_spreadsheet_id`          | --           |

#### SheetsResponse

```rust
enum SheetsResponse {
    Metadata {
        spreadsheet_id: String,
        title: String,
        sheets: Vec<SheetInfo>,
        locale: String,
    },
    Get {
        range: String,
        values: Vec<Vec<serde_json::Value>>,
        major_dimension: String,
    },
    Create {
        spreadsheet_id: String,
        title: String,
        url: String,
    },
    Update {
        updated_range: String,
        updated_rows: u32,
        updated_columns: u32,
        updated_cells: u32,
    },
    Append {
        updates: AppendResult,
    },
    Clear {
        cleared_range: String,
    },
    Format {
        replies: Vec<serde_json::Value>,
    },
    Insert {
        success: bool,
    },
    Notes {
        success: bool,
    },
    Export {
        content_base64: String,
        mime_type: String,
    },
    Copy {
        sheet_id: u32,
        title: String,
    },
}

struct SheetInfo {
    sheet_id: u32,
    title: String,
    index: u32,
    sheet_type: String,
    row_count: Option<u32>,
    column_count: Option<u32>,
}

struct AppendResult {
    spreadsheet_id: String,
    table_range: Option<String>,
    updated_range: String,
    updated_rows: u32,
    updated_columns: u32,
    updated_cells: u32,
}
```

---

### 12.7 Slides Service

Covers presentation info, creation, creation from Markdown, copying, export,
slide listing, slide addition, notes update, slide replacement, and slide
deletion.

#### SlidesRequest

```rust
enum SlidesRequest {
    Info {
        presentation_id: String,
    },
    Create {
        title: String,
        folder_id: Option<String>,
    },
    CreateFromMarkdown {
        title: String,
        markdown: String,
        folder_id: Option<String>,
    },
    Copy {
        presentation_id: String,
        title: Option<String>,
        folder_id: Option<String>,
    },
    Export {
        presentation_id: String,
        mime_type: Option<String>,
    },
    ListSlides {
        presentation_id: String,
    },
    AddSlide {
        presentation_id: String,
        layout: Option<String>,
        insertion_index: Option<u32>,
    },
    UpdateNotes {
        presentation_id: String,
        slide_id: String,
        notes: String,
    },
    ReplaceSlide {
        presentation_id: String,
        slide_id: String,
        requests: Vec<serde_json::Value>,
    },
    DeleteSlide {                          // ** destructive-permanent
        presentation_id: String,
        slide_id: String,
    },
}
```

| Variant              | Key Fields                                                | Feature Gate            |
|----------------------|-----------------------------------------------------------|-------------------------|
| `Info`               | `presentation_id`                                         | --                      |
| `Create`             | `title`, `folder_id?`                                     | --                      |
| `CreateFromMarkdown` | `title`, `markdown`, `folder_id?`                         | --                      |
| `Copy`               | `presentation_id`, `title?`, `folder_id?`                 | --                      |
| `Export`             | `presentation_id`, `mime_type?`                            | --                      |
| `ListSlides`         | `presentation_id`                                         | --                      |
| `AddSlide`           | `presentation_id`, `layout?`, `insertion_index?`          | --                      |
| `UpdateNotes`        | `presentation_id`, `slide_id`, `notes`                    | --                      |
| `ReplaceSlide`       | `presentation_id`, `slide_id`, `requests`                 | --                      |
| `DeleteSlide`**      | `presentation_id`, `slide_id`                             | `destructive-permanent` |

#### SlidesResponse

```rust
enum SlidesResponse {
    Info {
        presentation_id: String,
        title: String,
        slides_count: u32,
        locale: String,
    },
    Create {
        presentation_id: String,
        title: String,
        url: String,
    },
    CreateFromMarkdown {
        presentation_id: String,
        title: String,
        slides_count: u32,
        url: String,
    },
    Copy {
        presentation_id: String,
        title: String,
    },
    Export {
        content_base64: String,
        mime_type: String,
    },
    ListSlides {
        slides: Vec<SlideInfo>,
    },
    AddSlide {
        slide_id: String,
    },
    UpdateNotes {
        success: bool,
    },
    ReplaceSlide {
        replies: Vec<serde_json::Value>,
    },
    DeleteSlide { success: bool },          // ** destructive-permanent
}

struct SlideInfo {
    object_id: String,
    index: u32,
    layout: Option<String>,
    speaker_notes: Option<String>,
}
```

---

### 12.8 Forms Service

Covers form retrieval, creation, and form responses.

#### FormsRequest

```rust
enum FormsRequest {
    Get {
        form_id: String,
    },
    Create {
        title: String,
        document_title: Option<String>,
        description: Option<String>,
    },
    ResponsesList {
        form_id: String,
        page_token: Option<String>,
        page_size: Option<u32>,
    },
    ResponsesGet {
        form_id: String,
        response_id: String,
    },
}
```

| Variant         | Key Fields                                      | Feature Gate |
|-----------------|-------------------------------------------------|--------------|
| `Get`           | `form_id`                                       | --           |
| `Create`        | `title`, `document_title?`, `description?`      | --           |
| `ResponsesList` | `form_id`, `page_token?`, `page_size?`          | --           |
| `ResponsesGet`  | `form_id`, `response_id`                        | --           |

#### FormsResponse

```rust
enum FormsResponse {
    Get {
        form_id: String,
        title: String,
        description: Option<String>,
        responder_uri: Option<String>,
        items: Vec<FormItem>,
    },
    Create {
        form_id: String,
        title: String,
        responder_uri: String,
    },
    ResponsesList {
        responses: Vec<FormResponseEntry>,
        next_page_token: Option<String>,
    },
    ResponsesGet {
        response: FormResponseEntry,
    },
}

struct FormItem {
    item_id: String,
    title: String,
    description: Option<String>,
    question_type: Option<String>,
}

struct FormResponseEntry {
    response_id: String,
    create_time: String,
    last_submitted_time: String,
    respondent_email: Option<String>,
    answers: HashMap<String, serde_json::Value>,
}
```

---

### 12.9 Contacts Service

Covers contact CRUD, other contacts, and directory lookups.

#### ContactsRequest

```rust
enum ContactsRequest {
    List {
        max_results: Option<u32>,
        page_token: Option<String>,
        sort_order: Option<String>,
    },
    Search {
        query: String,
        max_results: Option<u32>,
    },
    Get {
        resource_name: String,
    },
    Create {
        given_name: String,
        family_name: Option<String>,
        email: Option<String>,
        phone: Option<String>,
        organization: Option<String>,
        title: Option<String>,
        notes: Option<String>,
    },
    Update {
        resource_name: String,
        given_name: Option<String>,
        family_name: Option<String>,
        email: Option<String>,
        phone: Option<String>,
        organization: Option<String>,
        title: Option<String>,
        notes: Option<String>,
        etag: String,
    },
    Delete {                               // ** destructive-permanent
        resource_name: String,
    },
    OtherList {
        max_results: Option<u32>,
        page_token: Option<String>,
    },
    OtherSearch {
        query: String,
        max_results: Option<u32>,
    },
    DirectoryList {
        max_results: Option<u32>,
        page_token: Option<String>,
        sources: Option<Vec<String>>,
    },
    DirectorySearch {
        query: String,
        max_results: Option<u32>,
        sources: Option<Vec<String>>,
    },
}
```

| Variant           | Key Fields                                                   | Feature Gate            |
|-------------------|--------------------------------------------------------------|-------------------------|
| `List`            | `max_results?`, `page_token?`, `sort_order?`                 | --                      |
| `Search`          | `query`, `max_results?`                                      | --                      |
| `Get`             | `resource_name`                                              | --                      |
| `Create`          | `given_name`, `family_name?`, `email?`, `phone?`, `organization?`, `title?`, `notes?` | -- |
| `Update`          | `resource_name`, `etag`, `given_name?`, `family_name?`, `email?`, `phone?`, `organization?`, `title?`, `notes?` | -- |
| `Delete`**        | `resource_name`                                              | `destructive-permanent` |
| `OtherList`       | `max_results?`, `page_token?`                                | --                      |
| `OtherSearch`     | `query`, `max_results?`                                      | --                      |
| `DirectoryList`   | `max_results?`, `page_token?`, `sources?`                    | --                      |
| `DirectorySearch` | `query`, `max_results?`, `sources?`                          | --                      |

#### ContactsResponse

```rust
enum ContactsResponse {
    List { contacts: Vec<Contact>, next_page_token: Option<String>, total_people: Option<u32> },
    Search { contacts: Vec<Contact> },
    Get { contact: Contact },
    Create { contact: Contact },
    Update { contact: Contact },
    Delete { success: bool },               // ** destructive-permanent
    OtherList { contacts: Vec<Contact>, next_page_token: Option<String> },
    OtherSearch { contacts: Vec<Contact> },
    DirectoryList { people: Vec<DirectoryPerson>, next_page_token: Option<String> },
    DirectorySearch { people: Vec<DirectoryPerson> },
}

struct Contact {
    resource_name: String,
    etag: String,
    given_name: Option<String>,
    family_name: Option<String>,
    display_name: Option<String>,
    emails: Vec<ContactField>,
    phones: Vec<ContactField>,
    organizations: Vec<Organization>,
    notes: Option<String>,
    photos: Vec<String>,
}

struct ContactField {
    value: String,
    field_type: Option<String>,
    primary: bool,
}

struct Organization {
    name: Option<String>,
    title: Option<String>,
    department: Option<String>,
}

struct DirectoryPerson {
    resource_name: String,
    display_name: Option<String>,
    emails: Vec<String>,
    phones: Vec<String>,
    department: Option<String>,
    title: Option<String>,
}
```

---

### 12.10 Tasks Service

Covers task lists, task CRUD, completion, and clearing.

#### TasksRequest

```rust
enum TasksRequest {
    TaskLists {
        max_results: Option<u32>,
        page_token: Option<String>,
    },
    TaskListCreate {
        title: String,
    },
    List {
        tasklist_id: Option<String>,
        max_results: Option<u32>,
        page_token: Option<String>,
        show_completed: Option<bool>,
        show_hidden: Option<bool>,
        due_min: Option<String>,
        due_max: Option<String>,
    },
    Get {
        tasklist_id: Option<String>,
        task_id: String,
    },
    Add {
        tasklist_id: Option<String>,
        title: String,
        notes: Option<String>,
        due: Option<String>,
        parent: Option<String>,
        previous: Option<String>,
    },
    Update {
        tasklist_id: Option<String>,
        task_id: String,
        title: Option<String>,
        notes: Option<String>,
        due: Option<String>,
        status: Option<String>,
    },
    Done {
        tasklist_id: Option<String>,
        task_id: String,
    },
    Undo {
        tasklist_id: Option<String>,
        task_id: String,
    },
    Delete {                               // ** destructive-permanent
        tasklist_id: Option<String>,
        task_id: String,
    },
    Clear {                                // ** destructive-permanent
        tasklist_id: Option<String>,
    },
}
```

| Variant          | Key Fields                                                   | Feature Gate            |
|------------------|--------------------------------------------------------------|-------------------------|
| `TaskLists`      | `max_results?`, `page_token?`                                | --                      |
| `TaskListCreate` | `title`                                                      | --                      |
| `List`           | `tasklist_id?`, `max_results?`, `page_token?`, `show_completed?`, `show_hidden?`, `due_min?`, `due_max?` | -- |
| `Get`            | `task_id`, `tasklist_id?`                                    | --                      |
| `Add`            | `title`, `tasklist_id?`, `notes?`, `due?`, `parent?`, `previous?` | --                 |
| `Update`         | `task_id`, `tasklist_id?`, `title?`, `notes?`, `due?`, `status?` | --                  |
| `Done`           | `task_id`, `tasklist_id?`                                    | --                      |
| `Undo`           | `task_id`, `tasklist_id?`                                    | --                      |
| `Delete`**       | `task_id`, `tasklist_id?`                                    | `destructive-permanent` |
| `Clear`**        | `tasklist_id?`                                               | `destructive-permanent` |

#### TasksResponse

```rust
enum TasksResponse {
    TaskLists { items: Vec<TaskList>, next_page_token: Option<String> },
    TaskListCreate { task_list: TaskList },
    List { items: Vec<Task>, next_page_token: Option<String> },
    Get { task: Task },
    Add { task: Task },
    Update { task: Task },
    Done { task: Task },
    Undo { task: Task },
    Delete { success: bool },               // ** destructive-permanent
    Clear { success: bool },                // ** destructive-permanent
}

struct TaskList {
    id: String,
    title: String,
    updated: String,
}

struct Task {
    id: String,
    title: String,
    notes: Option<String>,
    status: String,                         // "needsAction" or "completed"
    due: Option<String>,
    completed: Option<String>,
    updated: String,
    parent: Option<String>,
    position: String,
    links: Vec<TaskLink>,
}

struct TaskLink {
    link_type: Option<String>,
    description: Option<String>,
    link: Option<String>,
}
```

---

### 12.11 People Service

Covers profile information, user lookup, search, and relationship queries.

#### PeopleRequest

```rust
enum PeopleRequest {
    Me,
    Get {
        resource_name: String,
        person_fields: Option<String>,
    },
    Search {
        query: String,
        max_results: Option<u32>,
        sources: Option<Vec<String>>,
    },
    Relations {
        resource_name: Option<String>,
    },
}
```

| Variant     | Key Fields                                    | Feature Gate |
|-------------|-----------------------------------------------|--------------|
| `Me`        | (none)                                        | --           |
| `Get`       | `resource_name`, `person_fields?`             | --           |
| `Search`    | `query`, `max_results?`, `sources?`           | --           |
| `Relations` | `resource_name?`                              | --           |

#### PeopleResponse

```rust
enum PeopleResponse {
    Me { person: Person },
    Get { person: Person },
    Search { people: Vec<Person> },
    Relations { connections: Vec<Person> },
}

struct Person {
    resource_name: String,
    display_name: Option<String>,
    email_addresses: Vec<String>,
    phone_numbers: Vec<String>,
    photos: Vec<String>,
    organizations: Vec<Organization>,
    biographies: Vec<String>,
}
```

---

### 12.12 Chat Service

Covers Google Chat spaces, messages, threads, and direct messages.

#### ChatRequest

```rust
enum ChatRequest {
    SpacesList {
        max_results: Option<u32>,
        page_token: Option<String>,
        filter: Option<String>,
    },
    SpacesFind {
        display_name: String,
    },
    SpacesCreate {
        display_name: String,
        space_type: Option<String>,
        external_user_allowed: Option<bool>,
    },
    MessagesList {
        space_name: String,
        max_results: Option<u32>,
        page_token: Option<String>,
        filter: Option<String>,
        order_by: Option<String>,
        show_deleted: Option<bool>,
    },
    MessagesSend {
        space_name: String,
        text: String,
        thread_key: Option<String>,
        request_id: Option<String>,
    },
    ThreadsList {
        space_name: String,
        max_results: Option<u32>,
        page_token: Option<String>,
    },
    DmSpace {
        user_id: String,
    },
    DmSend {
        user_id: String,
        text: String,
    },
}
```

| Variant         | Key Fields                                                   | Feature Gate |
|-----------------|--------------------------------------------------------------|--------------|
| `SpacesList`    | `max_results?`, `page_token?`, `filter?`                     | --           |
| `SpacesFind`    | `display_name`                                               | --           |
| `SpacesCreate`  | `display_name`, `space_type?`, `external_user_allowed?`      | --           |
| `MessagesList`  | `space_name`, `max_results?`, `page_token?`, `filter?`, `order_by?`, `show_deleted?` | -- |
| `MessagesSend`  | `space_name`, `text`, `thread_key?`, `request_id?`           | --           |
| `ThreadsList`   | `space_name`, `max_results?`, `page_token?`                  | --           |
| `DmSpace`       | `user_id`                                                    | --           |
| `DmSend`        | `user_id`, `text`                                            | --           |

#### ChatResponse

```rust
enum ChatResponse {
    SpacesList { spaces: Vec<Space>, next_page_token: Option<String> },
    SpacesFind { space: Option<Space> },
    SpacesCreate { space: Space },
    MessagesList { messages: Vec<ChatMessage>, next_page_token: Option<String> },
    MessagesSend { message: ChatMessage },
    ThreadsList { threads: Vec<ChatThread>, next_page_token: Option<String> },
    DmSpace { space: Space },
    DmSend { message: ChatMessage },
}

struct Space {
    name: String,
    display_name: Option<String>,
    space_type: String,
    single_user_bot_dm: bool,
    threaded: bool,
    external_user_allowed: bool,
}

struct ChatMessage {
    name: String,
    sender: Option<String>,
    text: String,
    create_time: String,
    thread_name: Option<String>,
}

struct ChatThread {
    name: String,
    last_message_time: Option<String>,
}
```

---

### 12.13 Classroom Service

Covers courses, roster management, coursework, submissions, announcements,
topics, invitations, guardians, and profile info.

#### ClassroomRequest

```rust
enum ClassroomRequest {
    CoursesList {
        student_id: Option<String>,
        teacher_id: Option<String>,
        course_states: Option<Vec<String>>,
        max_results: Option<u32>,
        page_token: Option<String>,
    },
    CoursesGet {
        course_id: String,
    },
    CoursesCreate {
        name: String,
        section: Option<String>,
        description: Option<String>,
        room: Option<String>,
        owner_id: Option<String>,
    },
    CoursesUpdate {
        course_id: String,
        name: Option<String>,
        section: Option<String>,
        description: Option<String>,
        room: Option<String>,
        course_state: Option<String>,
    },
    CoursesArchive {
        course_id: String,
    },
    CoursesDelete {                        // ** destructive-permanent
        course_id: String,
    },
    Roster {
        course_id: String,
    },
    StudentsAdd {
        course_id: String,
        user_id: String,
    },
    TeachersAdd {
        course_id: String,
        user_id: String,
    },
    CourseworkList {
        course_id: String,
        max_results: Option<u32>,
        page_token: Option<String>,
        order_by: Option<String>,
        course_work_states: Option<Vec<String>>,
    },
    CourseworkGet {
        course_id: String,
        coursework_id: String,
    },
    CourseworkCreate {
        course_id: String,
        title: String,
        description: Option<String>,
        work_type: String,
        max_points: Option<f64>,
        due_date: Option<String>,
        due_time: Option<String>,
        state: Option<String>,
        materials: Option<Vec<serde_json::Value>>,
    },
    SubmissionsList {
        course_id: String,
        coursework_id: String,
        max_results: Option<u32>,
        page_token: Option<String>,
        states: Option<Vec<String>>,
    },
    SubmissionsGrade {
        course_id: String,
        coursework_id: String,
        submission_id: String,
        assigned_grade: Option<f64>,
        draft_grade: Option<f64>,
    },
    SubmissionsReturn {
        course_id: String,
        coursework_id: String,
        submission_id: String,
    },
    AnnouncementsList {
        course_id: String,
        max_results: Option<u32>,
        page_token: Option<String>,
        order_by: Option<String>,
        announcement_states: Option<Vec<String>>,
    },
    AnnouncementsCreate {
        course_id: String,
        text: String,
        materials: Option<Vec<serde_json::Value>>,
        state: Option<String>,
        assignee_mode: Option<String>,
    },
    TopicsList {
        course_id: String,
        max_results: Option<u32>,
        page_token: Option<String>,
    },
    TopicsCreate {
        course_id: String,
        name: String,
    },
    InvitationsList {
        course_id: Option<String>,
        user_id: Option<String>,
        max_results: Option<u32>,
        page_token: Option<String>,
    },
    InvitationsCreate {
        course_id: String,
        user_id: String,
        role: String,
    },
    InvitationsAccept {
        invitation_id: String,
    },
    GuardiansList {
        student_id: String,
        max_results: Option<u32>,
        page_token: Option<String>,
    },
    Profile {
        user_id: Option<String>,
    },
}
```

| Variant              | Key Fields                                                   | Feature Gate            |
|----------------------|--------------------------------------------------------------|-------------------------|
| `CoursesList`        | `student_id?`, `teacher_id?`, `course_states?`, `max_results?`, `page_token?` | -- |
| `CoursesGet`         | `course_id`                                                  | --                      |
| `CoursesCreate`      | `name`, `section?`, `description?`, `room?`, `owner_id?`    | --                      |
| `CoursesUpdate`      | `course_id`, `name?`, `section?`, `description?`, `room?`, `course_state?` | --     |
| `CoursesArchive`     | `course_id`                                                  | --                      |
| `CoursesDelete`**    | `course_id`                                                  | `destructive-permanent` |
| `Roster`             | `course_id`                                                  | --                      |
| `StudentsAdd`        | `course_id`, `user_id`                                       | --                      |
| `TeachersAdd`        | `course_id`, `user_id`                                       | --                      |
| `CourseworkList`     | `course_id`, `max_results?`, `page_token?`, `order_by?`, `course_work_states?` | -- |
| `CourseworkGet`      | `course_id`, `coursework_id`                                 | --                      |
| `CourseworkCreate`   | `course_id`, `title`, `work_type`, `description?`, `max_points?`, `due_date?`, `due_time?`, `state?`, `materials?` | -- |
| `SubmissionsList`    | `course_id`, `coursework_id`, `max_results?`, `page_token?`, `states?` | --         |
| `SubmissionsGrade`   | `course_id`, `coursework_id`, `submission_id`, `assigned_grade?`, `draft_grade?` | -- |
| `SubmissionsReturn`  | `course_id`, `coursework_id`, `submission_id`                | --                      |
| `AnnouncementsList`  | `course_id`, `max_results?`, `page_token?`, `order_by?`, `announcement_states?` | -- |
| `AnnouncementsCreate`| `course_id`, `text`, `materials?`, `state?`, `assignee_mode?` | --                    |
| `TopicsList`         | `course_id`, `max_results?`, `page_token?`                   | --                      |
| `TopicsCreate`       | `course_id`, `name`                                          | --                      |
| `InvitationsList`    | `course_id?`, `user_id?`, `max_results?`, `page_token?`     | --                      |
| `InvitationsCreate`  | `course_id`, `user_id`, `role`                               | --                      |
| `InvitationsAccept`  | `invitation_id`                                              | --                      |
| `GuardiansList`      | `student_id`, `max_results?`, `page_token?`                  | --                      |
| `Profile`            | `user_id?`                                                   | --                      |

#### ClassroomResponse

```rust
enum ClassroomResponse {
    CoursesList { courses: Vec<Course>, next_page_token: Option<String> },
    CoursesGet { course: Course },
    CoursesCreate { course: Course },
    CoursesUpdate { course: Course },
    CoursesArchive { course: Course },
    CoursesDelete { success: bool },        // ** destructive-permanent
    Roster { students: Vec<Student>, teachers: Vec<Teacher> },
    StudentsAdd { student: Student },
    TeachersAdd { teacher: Teacher },
    CourseworkList { coursework: Vec<CourseWork>, next_page_token: Option<String> },
    CourseworkGet { coursework: CourseWork },
    CourseworkCreate { coursework: CourseWork },
    SubmissionsList { submissions: Vec<Submission>, next_page_token: Option<String> },
    SubmissionsGrade { submission: Submission },
    SubmissionsReturn { submission: Submission },
    AnnouncementsList { announcements: Vec<Announcement>, next_page_token: Option<String> },
    AnnouncementsCreate { announcement: Announcement },
    TopicsList { topics: Vec<Topic>, next_page_token: Option<String> },
    TopicsCreate { topic: Topic },
    InvitationsList { invitations: Vec<Invitation>, next_page_token: Option<String> },
    InvitationsCreate { invitation: Invitation },
    InvitationsAccept { success: bool },
    GuardiansList { guardians: Vec<Guardian>, next_page_token: Option<String> },
    Profile { user_profile: UserProfile },
}

struct Course {
    id: String,
    name: String,
    section: Option<String>,
    description: Option<String>,
    room: Option<String>,
    owner_id: String,
    course_state: String,
    alternate_link: String,
    creation_time: String,
    update_time: String,
    enrollment_code: Option<String>,
}

struct Student {
    user_id: String,
    profile: UserProfile,
    course_id: String,
}

struct Teacher {
    user_id: String,
    profile: UserProfile,
    course_id: String,
}

struct CourseWork {
    id: String,
    course_id: String,
    title: String,
    description: Option<String>,
    work_type: String,
    state: String,
    max_points: Option<f64>,
    due_date: Option<String>,
    due_time: Option<String>,
    creation_time: String,
    update_time: String,
    alternate_link: String,
}

struct Submission {
    id: String,
    course_id: String,
    coursework_id: String,
    user_id: String,
    state: String,
    assigned_grade: Option<f64>,
    draft_grade: Option<f64>,
    late: bool,
    creation_time: String,
    update_time: String,
    alternate_link: String,
}

struct Announcement {
    id: String,
    course_id: String,
    text: String,
    state: String,
    creator_user_id: String,
    creation_time: String,
    update_time: String,
    alternate_link: String,
}

struct Topic {
    topic_id: String,
    course_id: String,
    name: String,
    update_time: String,
}

struct Invitation {
    id: String,
    user_id: String,
    course_id: String,
    role: String,
}

struct Guardian {
    student_id: String,
    guardian_id: String,
    invited_email_address: String,
    guardian_profile: Option<UserProfile>,
}

struct UserProfile {
    id: String,
    name: Option<String>,
    email_address: Option<String>,
    photo_url: Option<String>,
}
```

---

### 12.14 Groups Service

Covers Google Groups listing and member enumeration.

#### GroupsRequest

```rust
enum GroupsRequest {
    List {
        customer: Option<String>,
        domain: Option<String>,
        user_key: Option<String>,
        max_results: Option<u32>,
        page_token: Option<String>,
    },
    Members {
        group_id: String,
        max_results: Option<u32>,
        page_token: Option<String>,
        roles: Option<String>,
    },
}
```

| Variant   | Key Fields                                                   | Feature Gate |
|-----------|--------------------------------------------------------------|--------------|
| `List`    | `customer?`, `domain?`, `user_key?`, `max_results?`, `page_token?` | --     |
| `Members` | `group_id`, `max_results?`, `page_token?`, `roles?`         | --           |

#### GroupsResponse

```rust
enum GroupsResponse {
    List { groups: Vec<Group>, next_page_token: Option<String> },
    Members { members: Vec<GroupMember>, next_page_token: Option<String> },
}

struct Group {
    id: String,
    email: String,
    name: String,
    description: Option<String>,
    member_count: Option<u32>,
    admin_created: bool,
}

struct GroupMember {
    id: Option<String>,
    email: String,
    role: String,
    member_type: String,
    status: String,
}
```

---

### 12.15 Keep Service

Covers Google Keep note listing, retrieval, search, and attachment access.

#### KeepRequest

```rust
enum KeepRequest {
    List {
        max_results: Option<u32>,
        page_token: Option<String>,
        filter: Option<String>,
    },
    Get {
        note_id: String,
    },
    Search {
        query: String,
        max_results: Option<u32>,
    },
    Attachment {
        note_id: String,
        attachment_id: String,
    },
}
```

| Variant      | Key Fields                                    | Feature Gate |
|--------------|-----------------------------------------------|--------------|
| `List`       | `max_results?`, `page_token?`, `filter?`      | --           |
| `Get`        | `note_id`                                     | --           |
| `Search`     | `query`, `max_results?`                       | --           |
| `Attachment` | `note_id`, `attachment_id`                    | --           |

#### KeepResponse

```rust
enum KeepResponse {
    List { notes: Vec<Note>, next_page_token: Option<String> },
    Get { note: Note },
    Search { notes: Vec<Note> },
    Attachment { content_base64: String, mime_type: String },
}

struct Note {
    name: String,
    title: Option<String>,
    body: NoteBody,
    create_time: String,
    update_time: String,
    trashed: bool,
    attachments: Vec<NoteAttachment>,
}

struct NoteBody {
    text: Option<TextContent>,
    list: Option<ListContent>,
}

struct TextContent {
    text: String,
}

struct ListContent {
    items: Vec<ListItem>,
}

struct ListItem {
    text: String,
    checked: bool,
}

struct NoteAttachment {
    name: String,
    mime_types: Vec<String>,
}
```

---

### 12.16 AppScript Service

Covers Google Apps Script project management and execution.

#### AppScriptRequest

```rust
enum AppScriptRequest {
    Get {
        script_id: String,
    },
    Content {
        script_id: String,
        version_number: Option<u32>,
    },
    Create {
        title: String,
        parent_id: Option<String>,
    },
    Run {
        script_id: String,
        function: String,
        parameters: Option<Vec<serde_json::Value>>,
        dev_mode: Option<bool>,
    },
}
```

| Variant   | Key Fields                                                | Feature Gate |
|-----------|-----------------------------------------------------------|--------------|
| `Get`     | `script_id`                                               | --           |
| `Content` | `script_id`, `version_number?`                            | --           |
| `Create`  | `title`, `parent_id?`                                     | --           |
| `Run`     | `script_id`, `function`, `parameters?`, `dev_mode?`       | --           |

#### AppScriptResponse

```rust
enum AppScriptResponse {
    Get {
        script_id: String,
        title: String,
        create_time: String,
        update_time: String,
    },
    Content {
        files: Vec<ScriptFile>,
    },
    Create {
        script_id: String,
        title: String,
    },
    Run {
        result: Option<serde_json::Value>,
        error: Option<ScriptError>,
    },
}

struct ScriptFile {
    name: String,
    file_type: String,              // "SERVER_JS", "HTML", "JSON"
    source: String,
    create_time: Option<String>,
    update_time: Option<String>,
    function_set: Option<Vec<String>>,
}

struct ScriptError {
    message: String,
    error_type: String,
    script_stack_trace: Vec<StackEntry>,
}

struct StackEntry {
    function: String,
    line_number: u32,
}
```

---

### 12.17 Gemini Service (feature-gated: `gemini-web`)

**Experimental. Read-only.** Provides access to Gemini web chat history.
This entire service requires the `gemini-web` feature gate at compile time.
Attempting to send any `Gemini` request without the feature enabled will
result in error code 9 (`feature_disabled`).

#### GeminiRequest

```rust
#[cfg(feature = "gemini-web")]
enum GeminiRequest {
    ListConversations {
        max_results: Option<u32>,
        page_token: Option<String>,
    },
    GetConversation {
        conversation_id: String,
    },
    SearchConversations {
        query: String,
        max_results: Option<u32>,
    },
}
```

| Variant               | Key Fields                        | Feature Gate  |
|-----------------------|-----------------------------------|---------------|
| `ListConversations`   | `max_results?`, `page_token?`     | `gemini-web`  |
| `GetConversation`     | `conversation_id`                 | `gemini-web`  |
| `SearchConversations` | `query`, `max_results?`           | `gemini-web`  |

**No write operations exist for this service.** The type system enforces
read-only access at compile time. There are no methods to create, modify,
or delete conversations.

#### GeminiResponse

```rust
#[cfg(feature = "gemini-web")]
enum GeminiResponse {
    ListConversations {
        conversations: Vec<ConversationSummary>,
        next_page_token: Option<String>,
    },
    GetConversation {
        conversation: Conversation,
    },
    SearchConversations {
        conversations: Vec<ConversationSummary>,
    },
}

#[cfg(feature = "gemini-web")]
struct ConversationSummary {
    id: String,
    title: Option<String>,
    create_time: String,
    update_time: String,
    turn_count: u32,
}

#[cfg(feature = "gemini-web")]
struct Conversation {
    id: String,
    title: Option<String>,
    create_time: String,
    update_time: String,
    turns: Vec<ConversationTurn>,
}

#[cfg(feature = "gemini-web")]
struct ConversationTurn {
    role: String,                   // "user" or "model"
    text: String,
    create_time: String,
}
```

---

### 12.18 NotebookLm Service (feature-gated: `notebooklm`)

**Experimental.** Provides access to NotebookLM notebooks. This entire
service requires the `notebooklm` feature gate at compile time. Attempting
to send any `NotebookLm` request without the feature enabled will result
in error code 9 (`feature_disabled`).

#### NotebookLmRequest

```rust
#[cfg(feature = "notebooklm")]
enum NotebookLmRequest {
    List {
        max_results: Option<u32>,
        page_token: Option<String>,
    },
    Get {
        notebook_id: String,
    },
}
```

| Variant | Key Fields                     | Feature Gate  |
|---------|--------------------------------|---------------|
| `List`  | `max_results?`, `page_token?`  | `notebooklm`  |
| `Get`   | `notebook_id`                  | `notebooklm`  |

#### NotebookLmResponse

```rust
#[cfg(feature = "notebooklm")]
enum NotebookLmResponse {
    List {
        notebooks: Vec<NotebookSummary>,
        next_page_token: Option<String>,
    },
    Get {
        notebook: Notebook,
    },
}

#[cfg(feature = "notebooklm")]
struct NotebookSummary {
    id: String,
    title: String,
    create_time: String,
    update_time: String,
    source_count: u32,
}

#[cfg(feature = "notebooklm")]
struct Notebook {
    id: String,
    title: String,
    create_time: String,
    update_time: String,
    sources: Vec<NotebookSource>,
    notes: Vec<NotebookNote>,
}

#[cfg(feature = "notebooklm")]
struct NotebookSource {
    id: String,
    title: String,
    source_type: String,
    original_url: Option<String>,
}

#[cfg(feature = "notebooklm")]
struct NotebookNote {
    id: String,
    title: Option<String>,
    content: String,
    create_time: String,
    update_time: String,
}
```

---

### 12.19 Monitor Service

Controls server-push event subscriptions for real-time monitoring.

#### MonitorRequest

```rust
enum MonitorRequest {
    Subscribe {
        services: Vec<String>,
        filter: Option<serde_json::Value>,
    },
    Unsubscribe {
        services: Option<Vec<String>>,
    },
    Status,
}
```

| Variant       | Key Fields                     | Feature Gate |
|---------------|--------------------------------|--------------|
| `Subscribe`   | `services`, `filter?`          | --           |
| `Unsubscribe` | `services?`                    | --           |
| `Status`      | (none)                         | --           |

The `services` field accepts the following values: `"gmail"`, `"drive"`,
`"calendar"`, `"keep"`. If `Unsubscribe` is sent with `services: null`,
all subscriptions are removed.

The `filter` field is an optional JSON object for service-specific filter
criteria. Examples:

- Gmail: `{"label_ids": ["INBOX"]}` -- only notify for INBOX changes.
- Drive: `{"folder_id": "1abc..."}` -- only notify for changes in a folder.
- Calendar: `{"calendar_id": "primary"}` -- only notify for primary calendar.

#### MonitorResponse

```rust
enum MonitorResponse {
    Subscribe { subscribed: Vec<String> },
    Unsubscribe { remaining: Vec<String> },
    Status { subscriptions: Vec<SubscriptionInfo> },
}

struct SubscriptionInfo {
    service: String,
    active: bool,
    last_poll: Option<String>,
    cursor: Option<String>,
    interval_seconds: u32,
}
```

Default polling intervals:

| Service    | Interval |
|------------|----------|
| `gmail`    | 30s      |
| `drive`    | 60s      |
| `calendar` | 60s      |
| `keep`     | 120s     |

---

### 12.20 Index Service

Controls the offline content index (rusty-genius).

#### IndexRequest

```rust
enum IndexRequest {
    Query {
        query: String,
        namespaces: Option<Vec<String>>,
        max_results: Option<u32>,
    },
    Refresh {
        namespaces: Option<Vec<String>>,
        full: Option<bool>,
    },
    Status,
}
```

| Variant   | Key Fields                                   | Feature Gate |
|-----------|----------------------------------------------|--------------|
| `Query`   | `query`, `namespaces?`, `max_results?`       | --           |
| `Refresh` | `namespaces?`, `full?`                       | --           |
| `Status`  | (none)                                       | --           |

The `namespaces` field accepts: `"docs"`, `"gmail"`, `"keep"`, `"drive"`.
If omitted, all namespaces are included. Setting `full` to `true` on a
`Refresh` request forces a complete re-index rather than an incremental
update from the last cursor.

#### IndexResponse

```rust
enum IndexResponse {
    Query {
        results: Vec<IndexResult>,
        total_count: u32,
    },
    Refresh {
        namespaces_refreshed: Vec<String>,
        documents_indexed: u32,
    },
    Status {
        namespaces: Vec<IndexNamespaceStatus>,
    },
}

struct IndexResult {
    namespace: String,
    document_id: String,
    title: Option<String>,
    snippet: String,
    score: f64,
}

struct IndexNamespaceStatus {
    namespace: String,
    document_count: u32,
    last_refresh: Option<String>,
    cursor: Option<String>,
}
```

---

### 12.21 Ping

A no-op request used for liveness checks and latency measurement. Has no
fields.

#### Request

```json
{"id": 99, "payload": "Ping"}
```

#### Response

```json
{"id": 99, "result": {"Ok": "Pong"}}
```

The response payload for Ping is always the unit variant `"Pong"` in the
`ResponsePayload` enum.

---

### 12.22 Shutdown

Requests a graceful server shutdown. The optional `reason` field is logged
by the server.

#### Request

With reason:

```json
{"id": 100, "payload": {"Shutdown": {"reason": "user requested"}}}
```

Without reason:

```json
{"id": 100, "payload": {"Shutdown": {}}}
```

#### Response

```json
{"id": 100, "result": {"Ok": "Shutdown"}}
```

After sending this response, the server completes all in-flight requests and
then closes all connections. See section 14 for the full shutdown procedure.

---

## 13. Backpressure

### 13.1 Outstanding Request Limit

A client MUST NOT have more than **64 outstanding requests** (requests sent
but not yet answered) on a single connection at any time.

### 13.2 Server Enforcement

If a client exceeds 64 outstanding requests, the server MAY:

1. Reject the excess request immediately with error code 2
   (`invalid_request`) and include `"backpressure limit exceeded"` in the
   message field.
2. Queue the request internally (implementation-defined behavior).
3. Close the connection if the client persistently violates the limit.

### 13.3 Flow Control Recommendation

Clients SHOULD implement a semaphore or similar mechanism to limit
concurrency. A reasonable default is 16 concurrent outstanding requests for
well-behaved clients. The 64-request hard limit provides headroom for bursts.

### 13.4 Server-Side Queuing

The server maintains an internal queue for requests that arrive faster than
they can be dispatched to Google APIs. The server SHOULD process requests
in FIFO order within each connection. The server MAY process requests
concurrently across different connections.

### 13.5 Large Responses

For responses that may be large (e.g., `Drive::Download`, `Docs::Export`),
clients should be prepared for delayed responses. The backpressure limit
applies to the number of requests, not the size of responses.

---

## 14. Shutdown Protocol

### 14.1 Client-Initiated Shutdown

1. Client sends a `Shutdown` request with an optional `reason` string.
2. Server acknowledges with a `Shutdown` response (`"Ok": "Shutdown"`).
3. Server stops accepting new requests on this connection. Any further
   requests receive error code 10 (`shutdown_in_progress`).
4. Server completes all in-flight requests for this connection and sends
   their responses.
5. Server closes the connection.

### 14.2 Server-Initiated Shutdown

The server MAY send an unsolicited shutdown notification to all connected
clients:

```json
{"id": 0, "result": {"Ok": {"Shutdown": {"reason": "server maintenance"}}}}
```

- The `id` is 0 (server-initiated, not associated with any client request).
- After sending this, the server follows the same drain procedure: it stops
  accepting new requests, completes in-flight work, and closes connections.
- Clients SHOULD treat receipt of an unsolicited shutdown as a signal to
  stop sending requests and prepare to reconnect later.

### 14.3 Shutdown Timeout

The server SHOULD enforce a drain timeout (recommended: 30 seconds). If
in-flight requests do not complete within this timeout, the server SHOULD
cancel remaining work and force-close connections.

### 14.4 Abrupt Disconnection

If a client disconnects without sending a Shutdown request (e.g., process
crash, network partition, or `SIGKILL`), the server MUST:

1. Cancel any in-flight requests for that connection.
2. Remove any monitor subscriptions for that connection.
3. Clean up connection resources.

If the server process terminates, the socket file SHOULD be removed on clean
shutdown. On restart, the server MUST detect and remove any stale socket file
before binding a new one. A stale socket is detected by attempting to connect
to it; if the connection fails, the file is stale.

---

## 15. Examples

This section provides complete wire-format examples in NDJSON (JSON encoding).
Each example shows the exact bytes that would appear on the socket (minus the
terminating newline, which is implied).

### 15.1 Handshake (JSON Encoding)

**Client sends:**

```json
{"protocol":"cog/1","encoding":"json"}
```

**Server responds:**

```json
{"protocol":"cog/1","status":"ok"}
```

All subsequent messages use NDJSON framing.

---

### 15.2 Handshake (Binary Encoding)

**Client sends** (NDJSON -- handshake is always text):

```json
{"protocol":"cog/1","encoding":"binary"}
```

**Server responds** (NDJSON -- handshake response is always text):

```json
{"protocol":"cog/1","status":"ok"}
```

All subsequent messages use binary (4-byte LE length prefix + postcard)
framing.

---

### 15.3 Gmail Search

**Client sends:**

```json
{"id":1,"payload":{"Gmail":{"Search":{"query":"from:alice@example.com is:unread","max_results":5}}}}
```

**Server responds (formatted for readability):**

```json
{
  "id": 1,
  "result": {
    "Ok": {
      "Gmail": {
        "Search": {
          "messages": [
            {
              "id": "18a1b2c3d4e5f6",
              "thread_id": "18a1b2c3d4e5f6",
              "snippet": "Hey, are we still on for lunch?",
              "from": "alice@example.com",
              "subject": "Lunch tomorrow",
              "date": "2026-02-28T10:30:00Z",
              "label_ids": ["INBOX", "UNREAD"]
            },
            {
              "id": "18a1b2c3d4e5f7",
              "thread_id": "18a1b2c3d4e5f7",
              "snippet": "Please review the attached document...",
              "from": "alice@example.com",
              "subject": "Q1 Report Review",
              "date": "2026-02-27T14:15:00Z",
              "label_ids": ["INBOX", "UNREAD"]
            }
          ],
          "next_page_token": null,
          "result_size_estimate": 2
        }
      }
    }
  }
}
```

**On the wire (single NDJSON line):**

```
{"id":1,"result":{"Ok":{"Gmail":{"Search":{"messages":[{"id":"18a1b2c3d4e5f6","thread_id":"18a1b2c3d4e5f6","snippet":"Hey, are we still on for lunch?","from":"alice@example.com","subject":"Lunch tomorrow","date":"2026-02-28T10:30:00Z","label_ids":["INBOX","UNREAD"]},{"id":"18a1b2c3d4e5f7","thread_id":"18a1b2c3d4e5f7","snippet":"Please review the attached document...","from":"alice@example.com","subject":"Q1 Report Review","date":"2026-02-27T14:15:00Z","label_ids":["INBOX","UNREAD"]}],"next_page_token":null,"result_size_estimate":2}}}}}
```

---

### 15.4 Drive List

**Client sends:**

```json
{"id":2,"payload":{"Drive":{"List":{"folder_id":"root","max_results":3}}}}
```

**Server responds (formatted):**

```json
{
  "id": 2,
  "result": {
    "Ok": {
      "Drive": {
        "List": {
          "files": [
            {
              "id": "1abc2def3ghi4jkl",
              "name": "Project Notes",
              "mime_type": "application/vnd.google-apps.document",
              "modified_time": "2026-02-28T09:00:00Z",
              "created_time": "2026-01-15T08:30:00Z",
              "size": null,
              "owners": ["user@example.com"],
              "parents": ["0AHdef456"],
              "starred": false,
              "trashed": false,
              "shared": false,
              "web_view_link": "https://docs.google.com/document/d/1abc2def3ghi4jkl/edit",
              "description": null
            },
            {
              "id": "5mno6pqr7stu8vwx",
              "name": "Budget 2026",
              "mime_type": "application/vnd.google-apps.spreadsheet",
              "modified_time": "2026-02-27T16:45:00Z",
              "created_time": "2026-02-01T10:00:00Z",
              "size": null,
              "owners": ["user@example.com"],
              "parents": ["0AHdef456"],
              "starred": true,
              "trashed": false,
              "shared": true,
              "web_view_link": "https://docs.google.com/spreadsheets/d/5mno6pqr7stu8vwx/edit",
              "description": "Annual budget planning"
            },
            {
              "id": "9yza0bcd1efg2hij",
              "name": "presentation.pdf",
              "mime_type": "application/pdf",
              "modified_time": "2026-02-25T11:20:00Z",
              "created_time": "2026-02-25T11:20:00Z",
              "size": 2458624,
              "owners": ["user@example.com"],
              "parents": ["0AHdef456"],
              "starred": false,
              "trashed": false,
              "shared": false,
              "web_view_link": "https://drive.google.com/file/d/9yza0bcd1efg2hij/view",
              "description": null
            }
          ],
          "next_page_token": "CAMQx9fR0cXYxdcaGAEggIDA8Oy0nbYB"
        }
      }
    }
  }
}
```

---

### 15.5 Error Response (Not Found)

**Client sends (requesting a non-existent message):**

```json
{"id":3,"payload":{"Gmail":{"Get":{"message_id":"nonexistent123","format":"Full"}}}}
```

**Server responds:**

```json
{
  "id": 3,
  "result": {
    "Err": {
      "code": 3,
      "message": "message not found: nonexistent123",
      "details": {
        "google_error_code": 404,
        "google_message": "Requested entity was not found."
      }
    }
  }
}
```

---

### 15.6 Error Response (Destructive Operation Denied)

**Client sends (permanent delete without feature gate):**

```json
{"id":4,"payload":{"Drive":{"PermanentDelete":{"file_id":"1abc2def3ghi4jkl"}}}}
```

**Server responds:**

```json
{
  "id": 4,
  "result": {
    "Err": {
      "code": 7,
      "message": "permanent deletion requires the 'destructive-permanent' feature gate, which is not enabled in this build",
      "details": {
        "feature": "destructive-permanent",
        "operation": "Drive::PermanentDelete"
      }
    }
  }
}
```

---

### 15.7 Error Response (Bulk Trash Denied)

**Client sends (batch trash of 75 files without feature gate):**

```json
{"id":5,"payload":{"Drive":{"BatchTrash":{"file_ids":["id1","id2","id3","...75 total..."]}}}}
```

**Server responds:**

```json
{
  "id": 5,
  "result": {
    "Err": {
      "code": 8,
      "message": "bulk operation affects 75 resources (>50 limit); requires 'destructive-bulk-trash' feature gate",
      "details": {
        "feature": "destructive-bulk-trash",
        "count": 75,
        "limit": 50
      }
    }
  }
}
```

---

### 15.8 Error Response (Rate Limited)

**Server responds when upstream Google API returns 429:**

```json
{
  "id": 6,
  "result": {
    "Err": {
      "code": 6,
      "message": "rate limited by Google API",
      "details": {
        "retry_after_seconds": 30,
        "google_error_code": 429,
        "service": "gmail"
      }
    }
  }
}
```

---

### 15.9 Error Response (Feature Disabled)

**Client sends Gemini request without `gemini-web` feature:**

```json
{"id":7,"payload":{"Gemini":{"ListConversations":{"max_results":10}}}}
```

**Server responds:**

```json
{
  "id": 7,
  "result": {
    "Err": {
      "code": 9,
      "message": "the 'gemini-web' feature is not enabled in this build",
      "details": {
        "feature": "gemini-web",
        "service": "Gemini"
      }
    }
  }
}
```

---

### 15.10 Monitor Event

After subscribing:

```json
{"id":10,"payload":{"Monitor":{"Subscribe":{"services":["gmail","drive"]}}}}
```

Server confirms subscription:

```json
{"id":10,"result":{"Ok":{"Monitor":{"Subscribe":{"subscribed":["gmail","drive"]}}}}}
```

Later, the server pushes an event (no corresponding client request):

```json
{
  "event_type": "new_message",
  "service": "gmail",
  "payload": {
    "message_id": "18b2c3d4e5f6a7",
    "thread_id": "18b2c3d4e5f6a7",
    "from": "bob@example.com",
    "subject": "Re: Project Update",
    "snippet": "Thanks for the update. I have a few questions...",
    "label_ids": ["INBOX", "UNREAD", "IMPORTANT"]
  },
  "timestamp": "2026-02-28T16:42:15Z"
}
```

And a Drive event:

```json
{
  "event_type": "file_changed",
  "service": "drive",
  "payload": {
    "file_id": "1abc2def3ghi4jkl",
    "name": "Project Notes",
    "mime_type": "application/vnd.google-apps.document",
    "change_type": "modified",
    "modified_by": "alice@example.com"
  },
  "timestamp": "2026-02-28T16:43:30Z"
}
```

---

### 15.11 Ping / Pong

**Client sends:**

```json
{"id":99,"payload":"Ping"}
```

**Server responds:**

```json
{"id":99,"result":{"Ok":"Pong"}}
```

---

### 15.12 Shutdown (Client-Initiated)

**Client sends:**

```json
{"id":100,"payload":{"Shutdown":{"reason":"agent session complete"}}}
```

**Server responds:**

```json
{"id":100,"result":{"Ok":"Shutdown"}}
```

The server then completes any remaining in-flight requests and closes the
connection.

---

### 15.13 Shutdown (Server-Initiated)

The server pushes an unsolicited shutdown message to all connected clients:

```json
{"id":0,"result":{"Ok":{"Shutdown":{"reason":"server maintenance window"}}}}
```

Clients should stop sending requests and prepare to reconnect.

---

## Appendix A: Feature Gate Summary

| Feature Gate             | Affected Variants                                              | Error Code |
|--------------------------|----------------------------------------------------------------|------------|
| `destructive-permanent`  | Gmail: `LabelsDelete`, `BatchDelete`, `FiltersDelete`          | 7          |
|                          | Calendar: `Delete`                                             | 7          |
|                          | Drive: `PermanentDelete`, `EmptyTrash`                         | 7          |
|                          | Slides: `DeleteSlide`                                          | 7          |
|                          | Contacts: `Delete`                                             | 7          |
|                          | Tasks: `Delete`, `Clear`                                       | 7          |
|                          | Classroom: `CoursesDelete`                                     | 7          |
| `destructive-bulk-trash` | Gmail: `BatchModify` (when >50 messages)                       | 8          |
|                          | Drive: `BatchTrash` (when >50 files)                           | 8          |
|                          | Any bulk operation exceeding 50 items                          | 8          |
| `gemini-web`             | Gemini: all variants (`ListConversations`, `GetConversation`, `SearchConversations`) | 9 |
| `notebooklm`             | NotebookLm: all variants (`List`, `Get`)                       | 9          |

When a feature gate is not enabled:

- **Compile-time (binary mode)**: The enum variants do not exist in the
  compiled binary. Postcard decoding will reject unknown discriminant tags.
- **Runtime (NDJSON mode)**: The JSON deserializer fails to match the variant
  name. The server returns the appropriate error code.

---

## Appendix B: JSON Serialization Conventions

### B.1 Enum Representation

Rust enums are serialized using serde's **externally tagged** representation
(the default). This means:

- **Unit variants**: serialized as a JSON string.
  ```json
  "Ping"
  ```
- **Struct variants**: serialized as a JSON object with a single key (the
  variant name) mapping to the variant's fields.
  ```json
  {"Gmail": {"Search": {"query": "test"}}}
  ```
- **Nested enums**: follow the same rules recursively.
  ```json
  {"Ok": {"Gmail": {"Search": {"messages": [], "next_page_token": null, "result_size_estimate": 0}}}}
  {"Err": {"code": 1, "message": "internal error", "details": null}}
  ```

### B.2 Option Fields

`Option<T>` fields:

- `Some(value)` is serialized as the value itself.
- `None` is serialized as `null` or omitted from the JSON object.

Both representations MUST be accepted by parsers. When serializing,
implementations SHOULD omit `None` fields using serde's
`#[serde(skip_serializing_if = "Option::is_none")]` attribute.

### B.3 Numeric Types

- `u32` and `u64`: serialized as JSON numbers. Values exceeding JavaScript's
  safe integer range (2^53 - 1) for `u64` are still valid protocol messages
  but may lose precision in JavaScript-based clients. Clients that need
  exact `u64` values should parse them as strings or use a big-integer library.
- `f64`: serialized as a JSON number. `NaN` and `Infinity` are not valid JSON
  and MUST NOT appear on the wire.

### B.4 Binary Data

Binary data (file contents, attachments, exported documents) is encoded as
**base64** (standard alphabet with padding, RFC 4648 section 4) in a `String`
field. By convention, these fields are named with a `_base64` suffix
(e.g., `content_base64`).

### B.5 Timestamps

All timestamps use **ISO 8601** format with timezone designator. The server
SHOULD use UTC (`Z` suffix). Example: `"2026-02-28T15:30:00Z"`.

### B.6 HashMap Serialization

`HashMap<String, T>` is serialized as a JSON object with string keys:

```json
{"primary": [{"start": "...", "end": "..."}], "work": []}
```

---

## Appendix C: Binary Serialization Notes

### C.1 Postcard Format

Postcard uses a variable-length integer encoding (varint) and compact struct
layout. Key properties relevant to this protocol:

- **Enum variants**: encoded as a varint discriminant followed by the variant's
  fields in declaration order.
- **`Option<T>`**: encoded as a single byte (`0x00` for `None`, `0x01` for
  `Some`) followed by `T` if present.
- **Strings**: encoded as a varint length followed by UTF-8 bytes.
- **`Vec<T>`**: encoded as a varint length (element count) followed by `T`
  elements serialized in sequence.
- **Structs**: fields encoded in declaration order with no separators or
  field names.
- **`u32`/`u64`**: encoded as variable-length integers (1-5 bytes for u32,
  1-10 bytes for u64).
- **`f64`**: encoded as 8 bytes in little-endian IEEE 754 format.
- **`bool`**: encoded as a single byte (`0x00` for false, `0x01` for true).

### C.2 Compatibility

The postcard encoding is NOT self-describing. Both the client and server MUST
use identical Rust type definitions for serialization and deserialization to
succeed. Protocol version negotiation via the handshake (`"protocol":"cog/1"`)
ensures type compatibility.

### C.3 Feature-Gated Variants

When a feature gate is disabled at compile time, the corresponding enum
variants do not exist in the binary. Their discriminant values are not
assigned, and subsequent variants may receive different discriminant numbers.
This means binary messages from a build with different feature flags enabled
will fail to decode. This is intentional -- it prevents accidental use of
gated features.

### C.4 Endianness

All multi-byte values within the postcard payload follow postcard's encoding
rules (varint for integers, little-endian for floats). The 4-byte frame
length prefix is always little-endian, independent of postcard.

---

## Appendix D: Security Considerations

### D.1 Authentication

The COG Protocol does not define its own authentication mechanism. Security
relies on:

1. **Unix socket permissions**: The socket file is created with mode `0600`
   (owner read/write only).
2. **Peer credential verification**: The server checks `SO_PEERCRED` (Linux)
   or `LOCAL_PEERCRED` (macOS) to verify the connecting process UID matches
   the server's UID.
3. **Same-user model**: Only processes running as the same user can connect.

### D.2 No Encryption

UDS traffic is not encrypted. This is acceptable because:

- Communication is strictly local (same machine, same kernel).
- Both endpoints run as the same user.
- The kernel enforces process isolation and socket permissions.
- No network exposure is possible through UDS.

### D.3 Credential Isolation

Google OAuth2 tokens and service account keys are managed by the server
process and are never directly exposed through the protocol wire format.
The `Auth::Credentials` request returns short-lived access tokens (typically
valid for 1 hour). Refresh tokens and service account private keys remain
within the server's memory.

### D.4 Destructive Operation Safety

The feature gate system provides defense-in-depth against accidental data
loss:

1. **Compile-time**: Destructive enum variants must be explicitly opted into
   via Cargo feature flags at build time.
2. **Runtime (CLI)**: Even with the feature enabled, the CLI binary requires
   `--force` or interactive confirmation for destructive operations.
3. **Bulk threshold**: Operations affecting more than 50 resources require
   the separate `destructive-bulk-trash` feature gate.
4. **Protocol-level enforcement**: The server validates feature gates before
   dispatching to the Google API, returning structured error responses.

### D.5 Input Validation

The server MUST validate all incoming requests before processing:

- String lengths must not exceed reasonable bounds.
- Array sizes must not exceed configured limits.
- Email addresses should be syntactically valid.
- File IDs and resource names should match expected patterns.

Invalid input MUST result in error code 2 (`invalid_request`).

---

## Appendix E: Protocol Version History

| Version | Date       | Description                    |
|---------|------------|--------------------------------|
| 1.0     | 2026-02-28 | Initial draft specification.   |

---

## Appendix F: Reference Implementation

The reference implementation of this protocol is provided by the `cog-ndjson`
crate within the rusty-cog workspace. It includes:

- NDJSON parser and serializer with streaming support.
- Binary (postcard) frame codec.
- Handshake state machine.
- Request dispatch and response routing.
- Monitor event broadcasting.
- Comprehensive fuzz testing targets (see `fuzz/fuzz_targets/`).

The implementation uses the `smol` async runtime (no tokio dependency).
