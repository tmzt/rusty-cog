Start by cloning https://github.com/steipete/gogcli

Implement in Rust with smol, no tokio. Use surf for https access.

Keep the same LICENSE(s)
Refer to the original in our README.md and acknowledge the author(s)

1. Include a LDJSON mode with a UDS socket, using rust enums for the messages. Split the enums into nested groupings by Google service.

2. *Carefully* separate any "destructive writes" such as deletes behind a feature gate. We'll define "destructive writes" to mean permenant deletion (not moving to trash) of any resource, or deleting more than 50 resources at once (even in trash).

3. Write tests to ensure (step 2) is implemented properly.

4. Implement the *exact same* CLI interface as `gogcli` with clap, support extended binary lookup in ~/.rustycog/bin for custom subcommands.

5. Implement a "monitor" mode for incoming email, new documents, etc. Use a --monitor argument to enable or allow NDJSON command to enable.

6. Implement a low-level async library using smol/features and a comprehensive service mode (cog-api subcommand) in separate crates.

6. Make this useful for an agent to interact with Google Workspace, including Keep and NotebookLM. Enable experimental (feature-gated) Gemini web access (https://gemini.google.com) for access to chat history *only*. Make Gemini web service *strictly read-only*.

Plan and carefully implement. Consider each feature. Write comprehensive tests and add fuzzing mode for NDJSON protocol.
