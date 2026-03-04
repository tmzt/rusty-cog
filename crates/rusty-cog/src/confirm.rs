use std::io::{self, Write};

/// Prompt the user for confirmation of a destructive action.
///
/// Returns `true` if the user confirms, `false` if they decline.
/// Always returns `true` if `--force` is set or `--no-input` is set.
pub fn confirm_destructive(message: &str, force: bool, no_input: bool) -> bool {
    if force {
        return true;
    }

    if no_input {
        eprintln!("error: destructive operation requires confirmation (use --force in non-interactive mode)");
        return false;
    }

    eprint!("{message} [y/N] ");
    io::stderr().flush().ok();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return false;
    }

    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}
