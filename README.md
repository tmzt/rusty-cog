# rusty-cog

A Rust port of [gogcli](https://github.com/steipete/gogcli) by [Peter Steinberger](https://github.com/steipete).

Fast, script-friendly CLI for Google Workspace: Gmail, Calendar, Drive, Docs, Sheets, Slides, Forms, Contacts, Tasks, People, Chat, Classroom, Groups, Keep, and Apps Script.

## Features

- **Full gogcli compatibility** -- same CLI interface, same exit codes
- **Async runtime** -- built on `smol` (no tokio)
- **COG Protocol v1** -- Unix Domain Socket interface with NDJSON and binary wire formats for agent/automation use
- **Split destructive gates** -- permanent deletion and bulk operations require explicit feature flags
- **Monitor mode** -- real-time polling for Gmail, Drive, Calendar, Keep changes
- **Offline indexing** -- `Indexable` trait for content search (engine provided by [rusty-genius](https://github.com/AmbientELab-Worktrees/rusty-genius))
- **Experimental** -- feature-gated read-only Gemini web access and NotebookLM integration

## Installation

```sh
cargo install rusty-cog
```

The binary is named `cog`.

## Quick Start

```sh
# Authenticate
cog login user@example.com

# Search Gmail
cog gmail search "is:unread" --json

# List Drive files
cog ls

# Send an email
cog send --to recipient@example.com --subject "Hello" --body "World"

# Monitor for new emails
cog gmail search "is:unread" --monitor
```

## Build

```sh
cargo build                                    # Safe mode (default)
cargo build --features destructive-permanent   # Enable permanent deletion
cargo build --features destructive-bulk-trash  # Enable bulk operations >50
cargo build --features experimental            # Gemini web + NotebookLM
cargo build --all-features                     # Everything
```

## Feature Gates

| Feature | Description |
|---------|-------------|
| `destructive-permanent` | Permanent deletion of resources (bypasses trash) |
| `destructive-bulk-trash` | Bulk operations affecting >50 resources |
| `gemini-web` | Read-only Gemini conversation history access |
| `notebooklm` | NotebookLM integration |
| `experimental` | Alias for `gemini-web` + `notebooklm` |

## Architecture

Four workspace crates:

| Crate | Purpose |
|-------|---------|
| `cog-core` | Async Google API library (smol + hyper) |
| `cog-ndjson` | COG Protocol v1 wire formats and UDS server |
| `cog-api` | Service daemon mode |
| `rusty-cog` | CLI binary (`cog`) |

## Protocol

See [COG Protocol v1 Specification](docs/COG_PROTOv1.md) for the full UDS protocol documentation.

## Acknowledgments

This project is a Rust reimplementation of [gogcli](https://github.com/steipete/gogcli) by [Peter Steinberger](https://github.com/steipete). The CLI interface, command structure, and exit codes are designed to be fully compatible with the original.

## License

MIT -- see [LICENSE](LICENSE) for details.
