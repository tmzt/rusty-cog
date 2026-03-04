# rusty-cog

Rust port of [gogcli](https://github.com/steipete/gogcli) by [Peter Steinberger](https://github.com/steipete).
Google Workspace CLI covering Gmail, Calendar, Drive, Docs, Sheets, Slides, Forms, Contacts, Tasks, People, Chat, Classroom, Groups, Keep, and Apps Script.

## Architecture

Cargo workspace with four crates:

| Crate | Type | Purpose |
|-------|------|---------|
| `cog-core` | lib | Low-level async Google API library. Zero CLI deps. smol + hyper. |
| `cog-ndjson` | lib | COG Protocol v1: NDJSON + binary wire formats, UDS server, dispatch. |
| `cog-api` | lib+bin | Service daemon mode. Long-lived process exposing UDS interface. |
| `rusty-cog` | bin | CLI binary (`cog`). clap-based, mirrors gogcli's interface exactly. |

Dependency flow: `rusty-cog` -> `cog-api` -> `cog-ndjson` -> `cog-core`

## Technology Stack

- **Runtime**: `smol` 2.0 -- NO TOKIO. Not even transitively.
- **HTTP**: `hyper` 1.x + `smol-hyper` (smol-native, no tokio). `surf` is unmaintained since 2021.
- **TLS**: `rustls`
- **CLI**: `clap` 4.5 (derive API)
- **JSON**: `serde` + `serde_json`
- **Config**: `json5` at `$COG_HOME/config.json5` (default `~/.config/cog/`)
- **Errors**: `thiserror`
- **Keyring**: `keyring` crate (OS-native) with file fallback
- **Binary wire format**: `postcard` (serde-compatible, no_std friendly, compact)

## Paths and COG_HOME

All configuration, credentials, and state live under `COG_HOME`:

| Default | Override |
|---------|----------|
| `~/.config/cog/` | `$COG_HOME` environment variable |

Layout under `COG_HOME`:

```
$COG_HOME/
  config.json5              # Main config
  credentials.json          # Default OAuth client credentials
  credentials-{client}.json # Named OAuth client credentials
  keyring/                  # File-based keyring fallback
  sa-{email}.json           # Service account keys
  monitor_state.json        # Monitor cursor persistence
  index/                    # Content index data (rusty-genius)
  cog.sock                  # UDS socket (or $XDG_RUNTIME_DIR/cog.sock)
```

External subcommands: `~/.rustycog/bin/cog-<name>` (always here, not under COG_HOME).

## Build Commands

```sh
cargo build                                    # Safe mode (no destructive features)
cargo build --features destructive-permanent   # Include permanent delete operations
cargo build --features destructive-bulk-trash  # Include bulk trash >50 operations
cargo build --features experimental            # Include Gemini web + NotebookLM
cargo build --all-features                     # Everything
cargo test                                     # Default features
cargo test --features destructive-permanent    # Test permanent delete paths
cargo test --features destructive-bulk-trash   # Test bulk trash paths
cargo fuzz run ndjson_request                  # Fuzz NDJSON text parser
cargo fuzz run binary_request                  # Fuzz binary protocol parser
cargo fuzz run ndjson_roundtrip                # Fuzz JSON roundtrip
cargo fuzz run binary_roundtrip                # Fuzz binary roundtrip
cargo fuzz run protocol_stream                 # Fuzz multi-message stream
cargo fuzz run envelope_arbitrary              # Structure-aware fuzzing
cargo fuzz run handshake                       # Fuzz connection negotiation
```

## Feature Gates

| Feature | What it enables |
|---------|----------------|
| `destructive-permanent` | Permanent deletion of any resource (bypasses trash). |
| `destructive-bulk-trash` | Bulk operations affecting >50 resources (even to trash). |
| `gemini-web` | Experimental read-only Gemini web access (chat history only). |
| `notebooklm` | Experimental NotebookLM integration. |
| `experimental` | Alias for `gemini-web` + `notebooklm`. |

Features propagate: `rusty-cog` -> `cog-ndjson` -> `cog-core`.

### `destructive-permanent` -- Permanent deletion

Methods gated with `#[cfg(feature = "destructive-permanent")]`. They don't compile without it.

What requires it:
- Gmail: `delete` (permanent message delete)
- Drive: `permanent_delete`, `empty_trash`
- Slides: `delete_slide`
- Calendar: `delete` (permanent event removal)
- Tasks: `delete`, `clear` (no trash in Tasks API)
- Contacts: `delete` (no trash in Contacts API)
- Classroom: `delete` (courses)
- Gmail labels: `delete`
- Gmail filters: `delete`

### `destructive-bulk-trash` -- Bulk operations >50

Runtime check via `cog_core::destructive::check_bulk_trash(count)`. Returns `Error::BulkTrashDenied` when count > 50 and feature is disabled. Protocol enum variants for bulk ops are also gated.

What requires it:
- Gmail: `batch_trash`, `batch_modify` when >50 messages
- Drive: `batch_trash` when >50 files
- Any bulk operation exceeding 50 items

### What is NOT destructive (no feature gate):
- Moving a single resource to trash (reversible)
- Docs text range deletion (content editing, not resource deletion)
- Clearing cell values in Sheets
- Any read operation
- Bulk operations affecting <=50 items

## COG Protocol v1

**The protocol MUST be fully documented in `docs/COG_PROTOv1.md` before any implementation begins.**

Two wire formats over Unix Domain Socket, negotiated at connection time:

### Text mode (NDJSON)
Line-delimited JSON. One JSON object per `\n`-terminated line. Human-readable, debuggable.

### Binary mode
Length-prefixed binary frames using same Rust enum types, serialized with `postcard`. Lower overhead for high-throughput agent use.

### Connection handshake
First message from client:
```json
{"protocol": "cog/1", "encoding": "json"}
```
or:
```json
{"protocol": "cog/1", "encoding": "binary"}
```
After server acknowledges, all subsequent frames use the negotiated encoding.

### Shared enum hierarchy (both encodings)
```
CogRequest { id, payload: RequestPayload }

RequestPayload
  Gmail(GmailRequest)        -- Search, Get, Send, Thread, Labels, Trash, Delete**, ...
  Calendar(CalendarRequest)  -- Events, Create, Update, Delete**, ...
  Drive(DriveRequest)        -- List, Search, Download, Upload, Trash, PermanentDelete**, ...
  Docs(DocsRequest)          -- Cat, Export, Create, Write, FindReplace, ...
  Sheets(SheetsRequest)      -- Get, Update, Append, Create, ...
  Slides(SlidesRequest)      -- Export, Create, AddSlide, DeleteSlide**, ...
  Forms(FormsRequest)        -- Get, Create, Responses, ...
  Contacts(ContactsRequest)  -- Search, List, Create, Delete**, ...
  Tasks(TasksRequest)        -- Lists, List, Add, Done, Delete**, Clear**, ...
  People(PeopleRequest)      -- Me, Get, Search, ...
  Chat(ChatRequest)          -- Spaces, Messages, DM, ...
  Classroom(ClassroomRequest)-- Courses, Students, Teachers, ...
  Groups(GroupsRequest)      -- List, Members
  Keep(KeepRequest)          -- List, Get, Search
  AppScript(AppScriptRequest)-- Get, Content, Run, Create
  Gemini(GeminiRequest)*     -- List, Get, Search (read-only)
  NotebookLm(NotebookLmRequest)* -- List, Get
  Auth(AuthRequest)          -- Login, Status, ...
  Monitor(MonitorRequest)    -- Subscribe, Unsubscribe, Status
  Index(IndexRequest)        -- Query, Refresh, Status
  Ping
  Shutdown

* = experimental feature-gated
** = destructive-permanent feature-gated
```

### Socket location
`$COG_HOME/cog.sock`, falling back to `$XDG_RUNTIME_DIR/cog.sock` or `/tmp/cog-{uid}.sock`.

## Indexable Trait

Enables offline content indexing for search and retrieval. The indexing engine (`rusty-genius`) is a separate component implemented later. `cog-core` defines the trait; services implement it.

```rust
pub trait Indexable {
    type Document: serde::Serialize;

    async fn fetch_indexable(
        &self,
        since: Option<&str>,
        limit: usize,
    ) -> Result<(Vec<Self::Document>, Option<String>)>;

    fn index_namespace(&self) -> &'static str;
}
```

Services implementing Indexable:

| Service | What is indexed | Cursor type |
|---------|----------------|-------------|
| Docs | Document text, title, metadata | `modifiedTime` |
| Gmail | Subject, snippet, headers, body text | `historyId` |
| Keep | Note title, text content, list items | `updateTime` |
| Drive | File metadata and names | `changeToken` |

Protocol messages: `IndexRequest::Query`, `IndexRequest::Refresh`, `IndexRequest::Status`.
Index data at `$COG_HOME/index/`.

## CLI Interface

Mirrors gogcli exactly. Binary: `cog`. Global flags:

```
--color <auto|always|never>    Color output
-a, --account <email>          Account email
--client <name>                OAuth client name
-j, --json                     JSON output
-p, --plain                    TSV output (no colors)
--results-only                 In JSON mode, only primary result
--select <fields>              Project JSON fields
-n, --dry-run                  Print intended actions without executing
-y, --force                    Skip destructive confirmations
--no-input                     Never prompt (CI mode)
-v, --verbose                  Debug logging
--version                      Print version
--enable-commands <cmds>       Restrict available commands
```

### Desire-path shortcuts
| Shortcut | Maps to |
|----------|---------|
| `cog send` | `cog gmail send` |
| `cog ls` | `cog drive ls` |
| `cog search` | `cog drive search` |
| `cog open` | URL generator |
| `cog download` | `cog drive download` |
| `cog upload` | `cog drive upload` |
| `cog login` | `cog auth add` |
| `cog logout` | `cog auth remove` |
| `cog status` | `cog auth status` |
| `cog me` / `cog whoami` | `cog people me` |

### Exit codes (match gogcli)
0=ok, 1=error, 2=usage, 3=empty, 4=auth-required, 5=not-found, 6=permission-denied, 7=rate-limited, 8=retryable, 10=config, 130=cancelled

## Monitor Mode

Polling-based monitoring activated by `--monitor` flag or protocol `Subscribe` command.

| Service | API | Default interval |
|---------|-----|-----------------|
| Gmail | `history.list` | 30s |
| Drive | `changes.list` | 60s |
| Calendar | `events.list` + `updatedMin` | 60s |
| Keep | `notes.list` | 120s |

## Gemini Web Access

Feature-gated behind `gemini-web`. **STRICTLY READ-ONLY** -- no write methods exist in the source. The type system enforces this. Only: `list_conversations`, `get_conversation`, `search_conversations`.

## Adding a New Google Service

1. Types: `cog-core/src/types/<service>.rs`
2. Service client: `cog-core/src/services/<service>.rs`
3. If content is searchable offline: implement `Indexable` trait
4. Protocol enums: `cog-ndjson/src/services/<service>.rs`
5. CLI commands: `rusty-cog/src/cli/<service>.rs`
6. Register in root enums (`RequestPayload`, `Commands`) and CLI parser
7. Tests at each level
8. Permanent deletion: `#[cfg(feature = "destructive-permanent")]`
9. Bulk operations >50: `#[cfg(feature = "destructive-bulk-trash")]`

## Testing and Fuzzing

### Tests
- Unit: type serde roundtrips, error mapping, retry logic, circuit breaker, config, OAuth2
- Integration: CLI smoke, destructive gate verification, protocol (both modes), external subcommands
- CI matrix: default, `destructive-permanent`, `destructive-bulk-trash`, all-features

### Fuzzing suite (`fuzz/fuzz_targets/`)

| Target | What | Strategy |
|--------|------|----------|
| `ndjson_request` | JSON request parsing | Byte-level |
| `ndjson_response` | JSON response parsing | Byte-level |
| `binary_request` | Binary request parsing | Byte-level |
| `binary_response` | Binary response parsing | Byte-level |
| `ndjson_roundtrip` | JSON serialize/deserialize | Structure-aware (`Arbitrary`) |
| `binary_roundtrip` | Binary serialize/deserialize | Structure-aware (`Arbitrary`) |
| `protocol_stream` | Multi-message stream | Byte-level, split on `\n` |
| `envelope_arbitrary` | Full request + dispatch | Structure-aware |
| `handshake` | Connection negotiation | Byte-level |
