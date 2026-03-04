use std::process::Command;

fn cog_binary() -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_cog"));
    cmd.env("COG_HOME", "/tmp/cog-test-home");
    cmd
}

#[test]
fn version_flag() {
    let output = cog_binary()
        .arg("--version")
        .output()
        .expect("failed to run cog");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("cog"));
}

#[test]
fn help_flag() {
    let output = cog_binary()
        .arg("--help")
        .output()
        .expect("failed to run cog");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Google Workspace CLI"));
}

#[test]
fn version_subcommand() {
    let output = cog_binary()
        .arg("version")
        .output()
        .expect("failed to run cog");
    assert!(output.status.success());
}

#[test]
fn schema_subcommand() {
    let output = cog_binary()
        .arg("schema")
        .arg("--json")
        .output()
        .expect("failed to run cog");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("gmail"));
    assert!(stdout.contains("drive"));
}

#[test]
fn exit_codes_subcommand() {
    let output = cog_binary()
        .args(["agent", "exit-codes", "--json"])
        .output()
        .expect("failed to run cog");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("auth-required"));
    assert!(stdout.contains("rate-limited"));
}

#[test]
fn config_path() {
    let output = cog_binary()
        .args(["config", "path"])
        .output()
        .expect("failed to run cog");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("cog-test-home"));
}

#[test]
fn unknown_command_exits_usage() {
    let output = cog_binary()
        .arg("nonexistent-command")
        .output()
        .expect("failed to run cog");
    // clap returns exit code 2 for unknown commands
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn enable_commands_blocks_disallowed() {
    let output = cog_binary()
        .args(["--enable-commands", "auth", "gmail", "search", "test"])
        .output()
        .expect("failed to run cog");
    assert!(!output.status.success());
}
