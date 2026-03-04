use serde::Serialize;

/// Output format for CLI results.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// Colored table (default human-readable output).
    Table,
    /// JSON output (--json flag).
    Json,
    /// Tab-separated values (--plain flag).
    Tsv,
}

/// Output options derived from CLI global flags.
#[derive(Debug, Clone)]
pub struct OutputOptions {
    pub format: OutputFormat,
    pub results_only: bool,
    pub select_fields: Vec<String>,
    pub color: ColorMode,
}

/// Color output mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorMode {
    Auto,
    Always,
    Never,
}

impl Default for OutputOptions {
    fn default() -> Self {
        Self {
            format: OutputFormat::Table,
            results_only: false,
            select_fields: Vec::new(),
            color: ColorMode::Auto,
        }
    }
}

impl OutputOptions {
    /// Determine if color should be used based on mode and terminal.
    pub fn use_color(&self) -> bool {
        match self.color {
            ColorMode::Always => true,
            ColorMode::Never => false,
            ColorMode::Auto => atty_is_terminal(),
        }
    }
}

/// Print a value in the requested output format.
pub fn print_output<T: Serialize>(value: &T, options: &OutputOptions) -> anyhow::Result<()> {
    match options.format {
        OutputFormat::Json => {
            let json = if options.select_fields.is_empty() {
                serde_json::to_string_pretty(value)?
            } else {
                let mut v = serde_json::to_value(value)?;
                if let serde_json::Value::Object(ref mut map) = v {
                    let selected: serde_json::Map<String, serde_json::Value> = map
                        .iter()
                        .filter(|(k, _)| options.select_fields.contains(k))
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect();
                    v = serde_json::Value::Object(selected);
                }
                serde_json::to_string_pretty(&v)?
            };
            println!("{json}");
        }
        OutputFormat::Tsv => {
            let v = serde_json::to_value(value)?;
            if let serde_json::Value::Array(arr) = &v {
                for item in arr {
                    if let serde_json::Value::Object(map) = item {
                        let line: Vec<String> = map
                            .values()
                            .map(|v| match v {
                                serde_json::Value::String(s) => s.clone(),
                                other => other.to_string(),
                            })
                            .collect();
                        println!("{}", line.join("\t"));
                    }
                }
            } else if let serde_json::Value::Object(map) = &v {
                let line: Vec<String> = map
                    .values()
                    .map(|v| match v {
                        serde_json::Value::String(s) => s.clone(),
                        other => other.to_string(),
                    })
                    .collect();
                println!("{}", line.join("\t"));
            }
        }
        OutputFormat::Table => {
            // Default: pretty-print as JSON for now
            // TODO: Implement colored table formatting per service
            let json = serde_json::to_string_pretty(value)?;
            println!("{json}");
        }
    }
    Ok(())
}

fn atty_is_terminal() -> bool {
    // Simple check - in production would use atty or is-terminal crate
    std::env::var("TERM").is_ok()
}
