mod cli;
mod confirm;
mod exit;
mod mcp;
pub mod mcp_cards;
mod output;

use clap::Parser;
use cli::{Cli, Commands, AgentCommands};
use exit::ExitCode;
use output::{ColorMode, OutputFormat, OutputOptions};

fn main() {
    let cli = Cli::parse();

    // Initialize logging
    if cli.verbose {
        tracing_subscriber::fmt()
            .with_env_filter("debug")
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::from_default_env()
            )
            .init();
    }

    // Build output options from global flags
    let output_opts = OutputOptions {
        format: if cli.json {
            OutputFormat::Json
        } else if cli.plain {
            OutputFormat::Tsv
        } else {
            OutputFormat::Table
        },
        results_only: cli.results_only,
        select_fields: cli
            .select
            .as_deref()
            .map(|s| s.split(',').map(|f| f.trim().to_string()).collect())
            .unwrap_or_default(),
        color: match cli.color.as_str() {
            "always" => ColorMode::Always,
            "never" => ColorMode::Never,
            _ => ColorMode::Auto,
        },
    };

    // Check command allowlist
    if let Some(ref enabled) = cli.enable_commands {
        let allowed: Vec<&str> = enabled.split(',').map(|s| s.trim()).collect();
        let cmd_name = command_name(&cli.command);
        if !allowed.contains(&cmd_name) {
            eprintln!("error: command '{cmd_name}' is not in the enabled commands list");
            std::process::exit(ExitCode::Usage.code());
        }
    }

    let result = smol::block_on(run(cli, output_opts));

    match result {
        Ok(()) => std::process::exit(ExitCode::Ok.code()),
        Err(e) => {
            let code = ExitCode::from(&e);
            eprintln!("error: {e}");
            std::process::exit(code.code());
        }
    }
}

async fn run(cli: Cli, output_opts: OutputOptions) -> cog_core::Result<()> {
    match &cli.command {
        // -- Desire-path shortcuts --
        Commands::Login(args) => {
            run_auth_add(&cli, args, &output_opts).await
        }
        Commands::Logout(args) => {
            run_auth_remove(&cli, args, &output_opts).await
        }
        Commands::Status => {
            run_auth_status(&cli, &output_opts).await
        }
        Commands::Me | Commands::WhoAmI => {
            run_people_me(&cli, &output_opts).await
        }
        Commands::Send(args) => {
            run_gmail_send(&cli, args, &output_opts).await
        }
        Commands::Ls(args) => {
            run_drive_ls(&cli, args, &output_opts).await
        }
        Commands::Search(args) => {
            run_drive_search(&cli, args, &output_opts).await
        }
        Commands::Download(args) => {
            run_drive_download(&cli, args, &output_opts).await
        }
        Commands::Upload(args) => {
            run_drive_upload(&cli, args, &output_opts).await
        }
        Commands::Open { id } => {
            run_open(id).await
        }

        // -- Service commands --
        Commands::Auth(args) => {
            run_auth(&cli, args, &output_opts).await
        }
        Commands::Gmail(args) => {
            run_gmail(&cli, args, &output_opts).await
        }
        Commands::Calendar(args) => {
            run_calendar(&cli, args, &output_opts).await
        }
        Commands::Drive(args) => {
            run_drive(&cli, args, &output_opts).await
        }
        Commands::Docs(_args) => {
            Err(cog_core::Error::Other("docs commands not yet implemented".into()))
        }
        Commands::Sheets(_args) => {
            Err(cog_core::Error::Other("sheets commands not yet implemented".into()))
        }
        Commands::Slides(_args) => {
            Err(cog_core::Error::Other("slides commands not yet implemented".into()))
        }
        Commands::Forms(_args) => {
            Err(cog_core::Error::Other("forms commands not yet implemented".into()))
        }
        Commands::Contacts(_args) => {
            Err(cog_core::Error::Other("contacts commands not yet implemented".into()))
        }
        Commands::Tasks(_args) => {
            Err(cog_core::Error::Other("tasks commands not yet implemented".into()))
        }
        Commands::People(_args) => {
            Err(cog_core::Error::Other("people commands not yet implemented".into()))
        }
        Commands::Chat(_args) => {
            Err(cog_core::Error::Other("chat commands not yet implemented".into()))
        }
        Commands::Classroom(_args) => {
            Err(cog_core::Error::Other("classroom commands not yet implemented".into()))
        }
        Commands::Groups(_args) => {
            Err(cog_core::Error::Other("groups commands not yet implemented".into()))
        }
        Commands::Keep(_args) => {
            Err(cog_core::Error::Other("keep commands not yet implemented".into()))
        }
        Commands::AppScript(_args) => {
            Err(cog_core::Error::Other("appscript commands not yet implemented".into()))
        }
        Commands::Config(args) => {
            run_config(args, &output_opts).await
        }
        Commands::Time(_args) => {
            Err(cog_core::Error::Other("time commands not yet implemented".into()))
        }
        Commands::Completion { shell } => {
            run_completion(shell)
        }
        Commands::Mcp => {
            mcp::run_mcp(&cli).await
        }
        Commands::Version => {
            println!("cog {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        Commands::Schema => {
            run_schema(&output_opts).await
        }
        Commands::Agent { command } => {
            match command {
                AgentCommands::ExitCodes => {
                    run_exit_codes(&output_opts).await
                }
            }
        }

        #[cfg(feature = "gemini-web")]
        Commands::Gemini(_args) => {
            Err(cog_core::Error::Other("gemini commands not yet implemented".into()))
        }
        #[cfg(feature = "notebooklm")]
        Commands::NotebookLm(_args) => {
            Err(cog_core::Error::Other("notebooklm commands not yet implemented".into()))
        }
    }
}

// -- Auth commands --

async fn run_auth(
    cli: &Cli,
    args: &cli::auth::AuthArgs,
    output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    use cli::auth::AuthCommands;

    match &args.command {
        AuthCommands::Add(login_args) => run_auth_add(cli, login_args, output_opts).await,
        AuthCommands::Remove(logout_args) => run_auth_remove(cli, logout_args, output_opts).await,
        AuthCommands::Status => run_auth_status(cli, output_opts).await,
        AuthCommands::List { check } => run_auth_list(cli, *check, output_opts).await,
        AuthCommands::Services => run_auth_services(output_opts).await,
        AuthCommands::Credentials { path, sub } => {
            run_auth_credentials(path.as_deref(), sub.as_ref(), output_opts).await
        }
        AuthCommands::Keyring { backend } => run_auth_keyring(backend.as_deref()).await,
        AuthCommands::Manage => open_browser_url("https://myaccount.google.com/permissions"),
        _ => Err(cog_core::Error::Other(
            "auth subcommand not yet implemented".into(),
        )),
    }
}

/// Default services to authorize if none specified on the command line.
/// Trimmed to scopes that work for personal @gmail.com accounts. Pass
/// `--services <name>...` to request additional services explicitly; the
/// match arms in `cog_core::auth::OAuth2Client::scopes_for_services` still
/// route all of them, including the workspace-only ones (keep, people).
const DEFAULT_LOGIN_SERVICES: &[&str] = &[
    "gmail",
    "calendar",
    "drive",
    // "docs",
    // "sheets",
    // "slides",
    // "forms",
    // "contacts",
    // "tasks",
    // "people" removed: maps to directory.readonly — Workspace-only.
    // "keep"   removed: Google Keep API is Workspace-only.
];

async fn run_auth_add(
    cli: &Cli,
    args: &cli::auth::LoginArgs,
    _output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    let config = cog_core::Config::load()?;

    // Resolve client name
    let client_name = cli.client.as_deref().or_else(|| {
        args.email
            .as_deref()
            .and_then(|e| config.client_for_account(e))
    });

    // Load OAuth credentials
    let credentials = cog_core::load_credentials(client_name)?;

    // Build scopes
    let services: Vec<&str> = if args.services.is_empty() {
        DEFAULT_LOGIN_SERVICES.to_vec()
    } else {
        args.services.iter().map(|s| s.as_str()).collect()
    };
    let scopes = cog_core::auth::OAuth2Client::scopes_for_services(&services, args.readonly);

    let http = cog_core::HttpClient::new()?;
    let oauth = cog_core::auth::OAuth2Client::new(credentials.clone(), http);
    let scope_refs: Vec<&str> = scopes.iter().copied().collect();

    if args.remote {
        // Remote/headless mode: print URL, user pastes code
        return run_auth_add_remote(&oauth, &scope_refs, client_name, cli).await;
    }

    // Determine redirect strategy:
    //   - --listen <addr>: bind to user-specified address, use http://<addr> as redirect
    //   - "web" credentials: use exact redirect URIs from credentials file
    //   - "installed" (desktop): loopback redirect on our port range
    let (listener, redirect_uri) = if let Some(spec) = args.listen.as_deref() {
        bind_oauth_explicit(spec).await?
    } else if credentials.is_web() {
        bind_oauth_web(&credentials).await?
    } else {
        bind_oauth_installed().await?
    };

    let auth_url = oauth.authorization_url(&scope_refs, &redirect_uri)?;

    // Try to open browser, fall back to printing URL
    if args.manual || !try_open_browser(&auth_url) {
        eprintln!("Open this URL in your browser to authorize:\n");
        eprintln!("  {auth_url}\n");
    } else {
        eprintln!("Opening browser for authorization...");
        eprintln!("If the browser doesn't open, visit:\n");
        eprintln!("  {auth_url}\n");
    }

    eprintln!("Waiting for authorization...");

    // Wait for the OAuth callback
    let code = accept_oauth_callback(&listener).await?;

    // Exchange the code for tokens
    eprintln!("Exchanging authorization code...");
    let token = oauth
        .exchange_code(&code, &redirect_uri, &scope_refs, client_name)
        .await?;

    // Store the token
    let keyring_config = config.keyring_backend.as_deref();
    let keyring = cog_core::auth::keyring::KeyringBackend::from_config(keyring_config);
    let email = token.email.clone();
    keyring.store(&token)?;

    eprintln!("Logged in as {email}");
    let service_list = services.join(", ");
    eprintln!("Authorized services: {service_list}");

    Ok(())
}

/// Remote/headless mode: print URL, user pastes the code.
async fn run_auth_add_remote(
    oauth: &cog_core::auth::OAuth2Client,
    scopes: &[&str],
    client_name: Option<&str>,
    _cli: &Cli,
) -> cog_core::Result<()> {
    // For remote mode, use oob redirect
    let redirect_uri = "urn:ietf:wg:oauth:2.0:oob";
    let auth_url = oauth.authorization_url(scopes, redirect_uri)?;

    eprintln!("Open this URL on any device to authorize:\n");
    eprintln!("  {auth_url}\n");
    eprintln!("Then paste the authorization code below.");

    eprint!("Authorization code: ");
    std::io::Write::flush(&mut std::io::stderr()).ok();

    let mut code = String::new();
    std::io::stdin()
        .read_line(&mut code)
        .map_err(|e| cog_core::Error::Other(format!("failed to read code: {e}")))?;
    let code = code.trim();

    if code.is_empty() {
        return Err(cog_core::Error::Cancelled);
    }

    eprintln!("Exchanging authorization code...");
    let token = oauth.exchange_code(code, redirect_uri, scopes, client_name).await?;

    let config = cog_core::Config::load()?;
    let keyring = cog_core::auth::keyring::KeyringBackend::from_config(
        config.keyring_backend.as_deref(),
    );
    let email = token.email.clone();
    keyring.store(&token)?;

    eprintln!("Logged in as {email}");
    Ok(())
}

/// OAuth callback port range for installed (desktop) credentials.
const OAUTH_PORT_BASE: u16 = 44760;
const OAUTH_PORT_RANGE: u16 = 10;

/// Bind to a user-specified listen address. Accepts:
///   "IP:PORT"    -> bind IP:PORT,      redirect http://IP:PORT
///   "IP"         -> bind IP:44760,     redirect http://IP:44760
///   ":PORT"      -> bind 127.0.0.1:PORT, redirect http://127.0.0.1:PORT
/// Rejects wildcard hosts (0.0.0.0, ::, [::]) since the redirect URI needs a
/// routable host.
async fn bind_oauth_explicit(
    spec: &str,
) -> cog_core::Result<(smol::net::TcpListener, String)> {
    let (host, port) = parse_listen_spec(spec)?;
    let listener = smol::net::TcpListener::bind(format!("{host}:{port}"))
        .await
        .map_err(|e| cog_core::Error::Other(format!("bind {host}:{port} failed: {e}")))?;
    Ok((listener, format!("http://{host}:{port}")))
}

fn parse_listen_spec(spec: &str) -> cog_core::Result<(String, u16)> {
    if spec.is_empty() {
        return Err(cog_core::Error::Config(
            "--listen: empty address".to_string(),
        ));
    }

    if let Some(rest) = spec.strip_prefix(':') {
        let port: u16 = rest.parse().map_err(|_| {
            cog_core::Error::Config(format!("invalid port in --listen {spec:?}"))
        })?;
        return Ok(("127.0.0.1".to_string(), port));
    }

    let (host, port) = match spec.rsplit_once(':') {
        Some((h, p)) if !h.is_empty() && p.chars().all(|c| c.is_ascii_digit()) => {
            let port: u16 = p.parse().map_err(|_| {
                cog_core::Error::Config(format!("invalid port in --listen {spec:?}"))
            })?;
            (h.to_string(), port)
        }
        _ => (spec.to_string(), OAUTH_PORT_BASE),
    };

    if host == "0.0.0.0" || host == "::" || host == "[::]" {
        return Err(cog_core::Error::Config(format!(
            "--listen {spec:?}: wildcard hosts not allowed; specify a routable IP \
             (the value is used as the OAuth redirect URI host)"
        )));
    }

    Ok((host, port))
}

/// Installed (desktop) credentials: Google allows any loopback port.
/// Try our known range 44760-44769.
async fn bind_oauth_installed() -> cog_core::Result<(smol::net::TcpListener, String)> {
    for port in OAUTH_PORT_BASE..OAUTH_PORT_BASE + OAUTH_PORT_RANGE {
        match smol::net::TcpListener::bind(format!("127.0.0.1:{port}")).await {
            Ok(listener) => {
                return Ok((listener, format!("http://127.0.0.1:{port}")));
            }
            Err(_) => continue,
        }
    }
    Err(cog_core::Error::Other(format!(
        "could not bind to any port in range {OAUTH_PORT_BASE}-{}",
        OAUTH_PORT_BASE + OAUTH_PORT_RANGE - 1
    )))
}

/// Web credentials: must use an exact redirect URI registered in Cloud Console.
/// Reads the loopback URIs from the credentials file and binds to one.
async fn bind_oauth_web(
    creds: &cog_core::auth::ClientCredentials,
) -> cog_core::Result<(smol::net::TcpListener, String)> {
    for uri in creds.redirect_uris() {
        if let Some((host, port)) = parse_loopback_uri(uri) {
            match smol::net::TcpListener::bind(format!("{host}:{port}")).await {
                Ok(listener) => return Ok((listener, uri.to_string())),
                Err(_) => continue,
            }
        }
    }

    Err(cog_core::Error::Config(format!(
        "no usable loopback redirect URI in credentials file.\n\
         Add a URI like http://127.0.0.1:44760 to your web client's redirect URIs,\n\
         or use Desktop (installed) credentials instead."
    )))
}

/// Parse a loopback redirect URI into (host, port).
fn parse_loopback_uri(uri: &str) -> Option<(&str, u16)> {
    let without_scheme = uri.strip_prefix("http://")?;
    let (host, port_str) = if let Some((h, p)) = without_scheme.split_once(':') {
        (h, p.trim_end_matches('/'))
    } else {
        (without_scheme.trim_end_matches('/'), "80")
    };
    // Only loopback addresses
    if host != "127.0.0.1" && host != "localhost" {
        return None;
    }
    let port: u16 = port_str.parse().ok()?;
    Some((host, port))
}

/// Accept the OAuth redirect callback and extract the authorization code.
async fn accept_oauth_callback(
    listener: &smol::net::TcpListener,
) -> cog_core::Result<String> {
    use futures_lite::io::{AsyncReadExt, AsyncWriteExt};

    let (mut stream, _addr) = listener
        .accept()
        .await
        .map_err(|e| cog_core::Error::Other(format!("accept failed: {e}")))?;

    // Read the HTTP request
    let mut buf = vec![0u8; 4096];
    let n = stream
        .read(&mut buf)
        .await
        .map_err(|e| cog_core::Error::Other(format!("read failed: {e}")))?;
    let request = String::from_utf8_lossy(&buf[..n]);

    // Extract query parameters from: GET /?code=xxx&scope=... HTTP/1.1
    let query_string = request
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|path| path.split('?').nth(1))
        .unwrap_or("");

    let get_param = |name: &str| -> Option<String> {
        query_string
            .split('&')
            .find_map(|pair| {
                let (k, v) = pair.split_once('=')?;
                if k == name { Some(v.replace('+', " ")) } else { None }
            })
    };

    let code = get_param("code");
    let error = get_param("error");

    if let Some(error) = error {
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nConnection: close\r\n\r\n\
             <html><body><h2>Authorization failed</h2><p>Error: {error}</p>\
             <p>You can close this tab.</p></body></html>"
        );
        stream.write_all(response.as_bytes()).await.ok();
        stream.flush().await.ok();
        return Err(cog_core::Error::OAuth2(format!(
            "authorization denied: {error}"
        )));
    }

    let code = code.ok_or_else(|| {
        cog_core::Error::OAuth2("no authorization code in callback".into())
    })?;

    // Send success response
    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nConnection: close\r\n\r\n\
        <html><body><h2>Authorization successful!</h2>\
        <p>You can close this tab and return to the terminal.</p></body></html>";
    stream.write_all(response.as_bytes()).await.ok();
    stream.flush().await.ok();

    Ok(code)
}

/// Try to open a URL in the user's browser. Returns true if a browser command was found.
fn try_open_browser(url: &str) -> bool {
    // Try platform-specific openers in order
    let openers = if cfg!(target_os = "macos") {
        vec!["open"]
    } else if cfg!(target_os = "windows") {
        vec!["start"]
    } else {
        vec!["xdg-open", "sensible-browser", "x-www-browser"]
    };

    for opener in openers {
        match std::process::Command::new(opener)
            .arg(url)
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
        {
            Ok(_) => return true,
            Err(_) => continue,
        }
    }

    false
}

/// Open a URL in the browser, or print it.
fn open_browser_url(url: &str) -> cog_core::Result<()> {
    if try_open_browser(url) {
        eprintln!("Opened {url} in browser");
    } else {
        println!("{url}");
    }
    Ok(())
}

async fn run_auth_remove(
    _cli: &Cli,
    args: &cli::auth::LogoutArgs,
    _output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    let config = cog_core::Config::load()?;
    let email = config.resolve_account(&args.email);
    let keyring = cog_core::auth::keyring::KeyringBackend::from_config(
        config.keyring_backend.as_deref(),
    );
    keyring.remove(&email)?;
    eprintln!("Removed account {email}");
    Ok(())
}

async fn run_auth_status(
    cli: &Cli,
    output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    let config = cog_core::Config::load()?;
    let keyring = cog_core::auth::keyring::KeyringBackend::from_config(
        config.keyring_backend.as_deref(),
    );

    // If a specific account is given, show that one
    if let Some(ref account) = cli.account {
        let email = config.resolve_account(account);
        match keyring.get(&email)? {
            Some(token) => {
                let status = serde_json::json!({
                    "email": token.email,
                    "scopes": token.scopes,
                    "client": token.client_name,
                    "valid": token.is_valid(),
                });
                output::print_output(&status, output_opts)
                    .map_err(|e| cog_core::Error::Other(e.to_string()))
            }
            None => Err(cog_core::Error::AuthRequired(format!(
                "no token for {email}, run: cog login"
            ))),
        }
    } else {
        // Show all accounts
        let accounts = keyring.list()?;
        if accounts.is_empty() {
            eprintln!("No accounts configured. Run: cog login");
            return Err(cog_core::Error::AuthRequired("no accounts".into()));
        }
        for email in &accounts {
            if let Some(token) = keyring.get(email)? {
                let valid = if token.is_valid() { "valid" } else { "expired" };
                eprintln!("  {email} ({valid})");
            } else {
                eprintln!("  {email} (no token)");
            }
        }
        Ok(())
    }
}

async fn run_auth_list(
    _cli: &Cli,
    check: bool,
    output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    let config = cog_core::Config::load()?;
    let keyring = cog_core::auth::keyring::KeyringBackend::from_config(
        config.keyring_backend.as_deref(),
    );

    let accounts = keyring.list()?;
    if accounts.is_empty() {
        eprintln!("No accounts stored.");
        return Ok(());
    }

    let mut entries = Vec::new();
    for email in &accounts {
        let mut entry = serde_json::json!({"email": email});
        if check {
            if let Some(token) = keyring.get(email)? {
                entry["valid"] = serde_json::json!(token.is_valid());
                entry["scopes"] = serde_json::json!(token.scopes.len());
                entry["client"] = serde_json::json!(token.client_name);
            }
        }
        entries.push(entry);
    }

    output::print_output(&serde_json::json!({"accounts": entries}), output_opts)
        .map_err(|e| cog_core::Error::Other(e.to_string()))
}

async fn run_auth_services(output_opts: &OutputOptions) -> cog_core::Result<()> {
    let services = serde_json::json!({
        "services": [
            {"name": "gmail", "description": "Gmail"},
            {"name": "calendar", "description": "Google Calendar"},
            {"name": "drive", "description": "Google Drive"},
            {"name": "docs", "description": "Google Docs"},
            {"name": "sheets", "description": "Google Sheets"},
            {"name": "slides", "description": "Google Slides"},
            {"name": "forms", "description": "Google Forms"},
            {"name": "contacts", "description": "Google Contacts"},
            {"name": "tasks", "description": "Google Tasks"},
            {"name": "people", "description": "Google People / Directory"},
            {"name": "chat", "description": "Google Chat"},
            {"name": "classroom", "description": "Google Classroom"},
            {"name": "keep", "description": "Google Keep"},
            {"name": "appscript", "description": "Apps Script"},
            {"name": "groups", "description": "Google Groups"},
        ]
    });
    output::print_output(&services, output_opts)
        .map_err(|e| cog_core::Error::Other(e.to_string()))
}

async fn run_auth_credentials(
    path: Option<&str>,
    sub: Option<&cli::auth::CredentialsSub>,
    _output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    use cli::auth::CredentialsSub;

    match (path, sub) {
        (_, Some(CredentialsSub::Path)) | (None, None) => {
            let home = cog_core::cog_home()?;
            println!("{}", home.join("credentials.json").display());
            Ok(())
        }
        (Some(p), None) => {
            // Import credentials from given path
            let content = std::fs::read_to_string(p)?;
            // Validate it parses
            let _: cog_core::auth::ClientCredentials = serde_json::from_str(&content)
                .map_err(|e| cog_core::Error::Config(format!("invalid credentials: {e}")))?;
            let dest = cog_core::cog_home()?.join("credentials.json");
            std::fs::create_dir_all(dest.parent().unwrap())?;
            std::fs::write(&dest, content)?;
            eprintln!("Credentials saved to {}", dest.display());
            Ok(())
        }
        (_, Some(CredentialsSub::Reset)) => {
            let dest = cog_core::cog_home()?.join("credentials.json");
            if dest.exists() {
                std::fs::remove_file(&dest)?;
                eprintln!("Credentials removed");
            } else {
                eprintln!("No credentials file to remove");
            }
            Ok(())
        }
        (_, Some(CredentialsSub::Import { path })) => {
            let content = std::fs::read_to_string(path)?;
            let _: cog_core::auth::ClientCredentials = serde_json::from_str(&content)
                .map_err(|e| cog_core::Error::Config(format!("invalid credentials: {e}")))?;
            let dest = cog_core::cog_home()?.join("credentials.json");
            std::fs::create_dir_all(dest.parent().unwrap())?;
            std::fs::write(&dest, content)?;
            eprintln!("Credentials imported to {}", dest.display());
            Ok(())
        }
        (_, Some(CredentialsSub::Export { path })) => {
            let src = cog_core::cog_home()?.join("credentials.json");
            if !src.exists() {
                return Err(cog_core::Error::NotFound("no credentials file".into()));
            }
            std::fs::copy(&src, path)?;
            eprintln!("Credentials exported to {path}");
            Ok(())
        }
    }
}

async fn run_auth_keyring(backend: Option<&str>) -> cog_core::Result<()> {
    match backend {
        Some(name) => {
            let mut config = cog_core::Config::load()?;
            config.keyring_backend = Some(name.to_string());
            config.save()?;
            eprintln!("Keyring backend set to: {name}");
            Ok(())
        }
        None => {
            let config = cog_core::Config::load()?;
            let backend = config.keyring_backend.as_deref().unwrap_or("auto");
            println!("{backend}");
            Ok(())
        }
    }
}

async fn run_people_me(
    _cli: &Cli,
    _output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    Err(cog_core::Error::Other("people me not yet implemented".into()))
}

async fn run_gmail_send(
    _cli: &Cli,
    _args: &cli::gmail::SendArgs,
    _output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    Err(cog_core::Error::Other("gmail send not yet implemented".into()))
}

/// Dispatch for `cog drive <subcommand>`. Read-only subcommands are wired to
/// the service client; write operations return a clear "not yet wired" error
/// so the user can see which surface is intentionally missing.
async fn run_drive(
    cli: &Cli,
    args: &cli::drive::DriveArgs,
    output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    use cli::drive::DriveCommands;

    match &args.command {
        DriveCommands::Ls(ls_args) => run_drive_ls(cli, ls_args, output_opts).await,
        DriveCommands::Search(s_args) => run_drive_search(cli, s_args, output_opts).await,
        DriveCommands::Get { id } => run_drive_get(cli, id, output_opts).await,
        DriveCommands::Download(d_args) => run_drive_download(cli, d_args, output_opts).await,
        DriveCommands::Permissions { id } => {
            run_drive_permissions(cli, id, output_opts).await
        }
        DriveCommands::Drives { max } => run_drive_drives(cli, *max, output_opts).await,
        DriveCommands::Url { id } => run_drive_url(id).await,

        // Write operations: not wired yet.
        DriveCommands::Upload(_)
        | DriveCommands::Copy { .. }
        | DriveCommands::Mkdir { .. }
        | DriveCommands::Rename { .. }
        | DriveCommands::Move { .. }
        | DriveCommands::Delete { .. }
        | DriveCommands::Share { .. }
        | DriveCommands::Unshare { .. } => Err(cog_core::Error::Other(
            "drive write operations not yet implemented; read-only subcommands: \
             ls, search, get, download, permissions, drives, url".into(),
        )),
    }
}

/// Resolve the selected account and return a ready-to-use (`HttpClient`,
/// access-token) pair for constructing any service client.
///
/// Picks the account from `-a/--account`, or falls back to the single stored
/// account if exactly one exists. Pulls the stored token, refreshes it via
/// `OAuth2Client::get_access_token` if expired, and persists any refreshed
/// fields back to the keyring.
async fn load_access_token(
    cli: &Cli,
) -> cog_core::Result<(cog_core::HttpClient, String)> {
    let config = cog_core::Config::load()?;
    let keyring = cog_core::auth::keyring::KeyringBackend::from_config(
        config.keyring_backend.as_deref(),
    );

    let email = match cli.account.as_deref() {
        Some(account) => config.resolve_account(account),
        None => {
            let accounts = keyring.list()?;
            match accounts.len() {
                0 => {
                    return Err(cog_core::Error::AuthRequired(
                        "no accounts configured — run: cog login".into(),
                    ));
                }
                1 => accounts.into_iter().next().unwrap(),
                _ => {
                    return Err(cog_core::Error::Config(format!(
                        "multiple accounts configured; specify one with -a/--account. \
                         Found: {}",
                        accounts.join(", ")
                    )));
                }
            }
        }
    };

    let mut token = keyring.get(&email)?.ok_or_else(|| {
        cog_core::Error::AuthRequired(format!("no token for {email} — run: cog login"))
    })?;

    let client_name = cli
        .client
        .as_deref()
        .or_else(|| config.client_for_account(&email));
    let credentials = cog_core::load_credentials(client_name)?;
    let http = cog_core::HttpClient::new()?;
    let oauth = cog_core::auth::OAuth2Client::new(credentials, http.clone());
    let access_token = oauth.get_access_token(&mut token).await?;
    keyring.store(&token)?;

    Ok((http, access_token))
}

pub(crate) async fn load_drive_service(
    cli: &Cli,
) -> cog_core::Result<cog_core::services::drive::DriveService> {
    let (http, token) = load_access_token(cli).await?;
    Ok(cog_core::services::drive::DriveService::new(http, token))
}

pub(crate) async fn load_gmail_service(
    cli: &Cli,
) -> cog_core::Result<cog_core::services::gmail::GmailService> {
    let (http, token) = load_access_token(cli).await?;
    Ok(cog_core::services::gmail::GmailService::new(http, token))
}

pub(crate) async fn load_calendar_service(
    cli: &Cli,
) -> cog_core::Result<cog_core::services::calendar::CalendarService> {
    let (http, token) = load_access_token(cli).await?;
    Ok(cog_core::services::calendar::CalendarService::new(http, token))
}

async fn run_drive_ls(
    cli: &Cli,
    args: &cli::drive::LsArgs,
    output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    let drive = load_drive_service(cli).await?;
    let (files, next_page_token) = drive
        .list(args.parent.as_deref(), args.max, None, None)
        .await?;

    let body = if next_page_token.is_some() {
        serde_json::json!({ "files": files, "nextPageToken": next_page_token })
    } else {
        serde_json::json!({ "files": files })
    };
    output::print_output(&body, output_opts)
        .map_err(|e| cog_core::Error::Other(e.to_string()))
}

async fn run_drive_search(
    cli: &Cli,
    args: &cli::drive::SearchArgs,
    output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    let drive = load_drive_service(cli).await?;

    // Without --raw-query, wrap the user's text in a `name contains` clause and
    // exclude trashed items. With --raw-query, pass straight through so power
    // users can write arbitrary Drive query syntax.
    let query = if args.raw_query {
        args.query.clone()
    } else {
        let escaped = args.query.replace('\\', "\\\\").replace('\'', "\\'");
        format!("name contains '{escaped}' and trashed = false")
    };

    let (files, next_page_token) = drive.search(&query, args.max, None).await?;
    let body = if next_page_token.is_some() {
        serde_json::json!({ "files": files, "nextPageToken": next_page_token })
    } else {
        serde_json::json!({ "files": files })
    };
    output::print_output(&body, output_opts)
        .map_err(|e| cog_core::Error::Other(e.to_string()))
}

async fn run_drive_get(
    cli: &Cli,
    file_id: &str,
    output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    let drive = load_drive_service(cli).await?;
    let file = drive.get(file_id).await?;
    output::print_output(&file, output_opts)
        .map_err(|e| cog_core::Error::Other(e.to_string()))
}

async fn run_drive_download(
    cli: &Cli,
    args: &cli::drive::DownloadArgs,
    _output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    let drive = load_drive_service(cli).await?;

    let bytes = if let Some(fmt) = args.format.as_deref() {
        let mime = export_mime_for_format(fmt)?;
        drive.export(&args.file_id, &mime).await?
    } else {
        drive.download(&args.file_id).await?
    };

    match args.out.as_deref() {
        Some(path) => {
            std::fs::write(path, &bytes)?;
            eprintln!("wrote {} bytes to {path}", bytes.len());
        }
        None => {
            use std::io::Write;
            std::io::stdout().write_all(&bytes)?;
            std::io::stdout().flush()?;
        }
    }
    Ok(())
}

/// Map a short format name to a Drive export MIME type. Anything containing
/// a `/` is passed through as a raw MIME type.
fn export_mime_for_format(fmt: &str) -> cog_core::Result<String> {
    if fmt.contains('/') {
        return Ok(fmt.to_string());
    }
    let mime = match fmt.to_ascii_lowercase().as_str() {
        "pdf" => "application/pdf",
        "txt" | "text" => "text/plain",
        "html" | "htm" => "text/html",
        "csv" => "text/csv",
        "tsv" => "text/tab-separated-values",
        "md" | "markdown" => "text/markdown",
        "rtf" => "application/rtf",
        "odt" => "application/vnd.oasis.opendocument.text",
        "ods" => "application/vnd.oasis.opendocument.spreadsheet",
        "odp" => "application/vnd.oasis.opendocument.presentation",
        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "pptx" => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        "epub" => "application/epub+zip",
        "zip" => "application/zip",
        "json" => "application/vnd.google-apps.script+json",
        other => {
            return Err(cog_core::Error::Config(format!(
                "unknown export format '{other}'; pass a full MIME type \
                 (e.g. --format application/pdf) or one of: pdf, docx, xlsx, \
                 pptx, odt, ods, odp, txt, html, csv, tsv, md, rtf, epub, zip"
            )));
        }
    };
    Ok(mime.to_string())
}

async fn run_drive_permissions(
    cli: &Cli,
    file_id: &str,
    output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    let drive = load_drive_service(cli).await?;
    let perms = drive.permissions(file_id).await?;
    output::print_output(&serde_json::json!({ "permissions": perms }), output_opts)
        .map_err(|e| cog_core::Error::Other(e.to_string()))
}

async fn run_drive_drives(
    cli: &Cli,
    _max: Option<u32>,
    output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    let drive = load_drive_service(cli).await?;
    let (drives, next_page_token) = drive.drives(None).await?;
    let body = if next_page_token.is_some() {
        serde_json::json!({ "drives": drives, "nextPageToken": next_page_token })
    } else {
        serde_json::json!({ "drives": drives })
    };
    output::print_output(&body, output_opts)
        .map_err(|e| cog_core::Error::Other(e.to_string()))
}

async fn run_drive_url(id: &str) -> cog_core::Result<()> {
    println!("https://drive.google.com/open?id={id}");
    Ok(())
}

async fn run_drive_upload(
    _cli: &Cli,
    _args: &cli::drive::UploadArgs,
    _output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    Err(cog_core::Error::Other(
        "drive upload not yet implemented".into(),
    ))
}

// ---------------------------------------------------------------------------
// Gmail (read-only)
// ---------------------------------------------------------------------------

async fn run_gmail(
    cli: &Cli,
    args: &cli::gmail::GmailArgs,
    output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    use cli::gmail::{
        DraftsCommands, FiltersCommands, GmailCommands, LabelsCommands, MessagesCommands,
        ThreadCommands,
    };

    match &args.command {
        GmailCommands::Search {
            query,
            max,
            include_spam_trash: _,
            label_ids: _,
        } => run_gmail_search(cli, query, *max, output_opts).await,
        GmailCommands::Inbox {
            max,
            unread,
            newer_than,
            q,
        } => run_gmail_inbox(cli, *max, *unread, newer_than.as_deref(), q.as_deref(), output_opts).await,
        GmailCommands::Get { id, format: _ } => run_gmail_get(cli, id, output_opts).await,
        GmailCommands::Url { id } => run_gmail_url(id).await,
        GmailCommands::Messages { sub } => match sub {
            MessagesCommands::Search { query, max } => {
                run_gmail_messages_search(cli, query, *max, output_opts).await
            }
        },
        GmailCommands::Thread { sub } => match sub {
            ThreadCommands::Get { id, format: _ } => run_gmail_thread_get(cli, id, output_opts).await,
            ThreadCommands::Modify { .. } => Err(read_only_err("gmail thread modify")),
        },
        GmailCommands::Labels { sub } => match sub {
            LabelsCommands::List => run_gmail_labels_list(cli, output_opts).await,
            LabelsCommands::Get { .. }
            | LabelsCommands::Create { .. }
            | LabelsCommands::Update { .. } => Err(read_only_err("gmail labels write ops")),
            #[cfg(feature = "destructive-permanent")]
            LabelsCommands::Delete { .. } => Err(read_only_err("gmail labels delete")),
        },
        GmailCommands::Drafts { sub } => match sub {
            DraftsCommands::List { max } => run_gmail_drafts_list(cli, *max, output_opts).await,
            DraftsCommands::Get { .. }
            | DraftsCommands::Create(_)
            | DraftsCommands::Update { .. }
            | DraftsCommands::Send { .. }
            | DraftsCommands::Delete { .. } => Err(read_only_err("gmail drafts write ops")),
        },
        GmailCommands::Filters { sub } => match sub {
            FiltersCommands::List => run_gmail_filters_list(cli, output_opts).await,
            FiltersCommands::Get { .. } | FiltersCommands::Create { .. } => {
                Err(read_only_err("gmail filters write ops"))
            }
            #[cfg(feature = "destructive-permanent")]
            FiltersCommands::Delete { .. } => Err(read_only_err("gmail filters delete")),
        },
        GmailCommands::History {
            start_history_id,
            max,
        } => run_gmail_history(cli, start_history_id.as_deref(), *max, output_opts).await,

        // Write-only / not yet wired.
        GmailCommands::Send(_)
        | GmailCommands::Attachment { .. }
        | GmailCommands::Batch { .. }
        | GmailCommands::AutoForward { .. }
        | GmailCommands::Forwarding { .. }
        | GmailCommands::SendAs { .. }
        | GmailCommands::Vacation { .. }
        | GmailCommands::Delegates { .. }
        | GmailCommands::Watch { .. }
        | GmailCommands::Track { .. } => Err(cog_core::Error::Other(
            "gmail: only read-only subcommands are wired yet — available: \
             search, inbox, get, url, messages search, thread get, labels list, \
             drafts list, filters list, history".into(),
        )),
    }
}

fn read_only_err(what: &str) -> cog_core::Error {
    cog_core::Error::Other(format!("{what} not implemented in read-only build"))
}

async fn run_gmail_search(
    cli: &Cli,
    query: &str,
    max: Option<u32>,
    output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    let gmail = load_gmail_service(cli).await?;
    let (messages, next_page_token) = gmail.search(query, max, None).await?;
    let body = serde_json::json!({
        "messages": messages,
        "nextPageToken": next_page_token,
    });
    output::print_output(&body, output_opts)
        .map_err(|e| cog_core::Error::Other(e.to_string()))
}

/// List inbox messages with full payload (snippet + headers). Uses
/// `messages_search` rather than plain `search` because the latter returns
/// id/threadId only, which isn't useful for a human browsing new mail.
async fn run_gmail_inbox(
    cli: &Cli,
    max: u32,
    unread: bool,
    newer_than: Option<&str>,
    extra: Option<&str>,
    output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    let mut parts = vec!["in:inbox".to_string()];
    if unread {
        parts.push("is:unread".to_string());
    }
    if let Some(window) = newer_than {
        parts.push(format!("newer_than:{window}"));
    }
    if let Some(extra) = extra {
        parts.push(extra.to_string());
    }
    let query = parts.join(" ");

    let gmail = load_gmail_service(cli).await?;
    let messages = gmail.messages_search(&query, Some(max)).await?;
    output::print_output(
        &serde_json::json!({ "query": query, "messages": messages }),
        output_opts,
    )
    .map_err(|e| cog_core::Error::Other(e.to_string()))
}

async fn run_gmail_messages_search(
    cli: &Cli,
    query: &str,
    max: Option<u32>,
    output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    let gmail = load_gmail_service(cli).await?;
    let messages = gmail.messages_search(query, max).await?;
    output::print_output(&serde_json::json!({ "messages": messages }), output_opts)
        .map_err(|e| cog_core::Error::Other(e.to_string()))
}

async fn run_gmail_get(
    cli: &Cli,
    id: &str,
    output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    let gmail = load_gmail_service(cli).await?;
    let message = gmail.get(id).await?;
    output::print_output(&message, output_opts)
        .map_err(|e| cog_core::Error::Other(e.to_string()))
}

async fn run_gmail_thread_get(
    cli: &Cli,
    id: &str,
    output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    let gmail = load_gmail_service(cli).await?;
    let thread = gmail.thread_get(id).await?;
    output::print_output(&thread, output_opts)
        .map_err(|e| cog_core::Error::Other(e.to_string()))
}

async fn run_gmail_labels_list(
    cli: &Cli,
    output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    let gmail = load_gmail_service(cli).await?;
    let labels = gmail.labels_list().await?;
    output::print_output(&serde_json::json!({ "labels": labels }), output_opts)
        .map_err(|e| cog_core::Error::Other(e.to_string()))
}

async fn run_gmail_drafts_list(
    cli: &Cli,
    max: Option<u32>,
    output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    let gmail = load_gmail_service(cli).await?;
    let (drafts, next_page_token) = gmail.drafts_list(max, None).await?;
    output::print_output(
        &serde_json::json!({ "drafts": drafts, "nextPageToken": next_page_token }),
        output_opts,
    )
    .map_err(|e| cog_core::Error::Other(e.to_string()))
}

async fn run_gmail_filters_list(
    cli: &Cli,
    output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    let gmail = load_gmail_service(cli).await?;
    let filters = gmail.filters_list().await?;
    output::print_output(&serde_json::json!({ "filters": filters }), output_opts)
        .map_err(|e| cog_core::Error::Other(e.to_string()))
}

async fn run_gmail_history(
    cli: &Cli,
    start_history_id: Option<&str>,
    _max: Option<u32>,
    output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    let start = start_history_id.ok_or_else(|| {
        cog_core::Error::Config(
            "gmail history requires --start-history-id; \
             get a current historyId via: cog gmail get <any-message-id>".into(),
        )
    })?;
    let gmail = load_gmail_service(cli).await?;
    let (records, next_page_token, history_id) = gmail.history(start, None).await?;
    output::print_output(
        &serde_json::json!({
            "history": records,
            "nextPageToken": next_page_token,
            "historyId": history_id,
        }),
        output_opts,
    )
    .map_err(|e| cog_core::Error::Other(e.to_string()))
}

async fn run_gmail_url(id: &str) -> cog_core::Result<()> {
    println!("https://mail.google.com/mail/u/0/#all/{id}");
    Ok(())
}

// ---------------------------------------------------------------------------
// Calendar (read-only)
// ---------------------------------------------------------------------------

async fn run_calendar(
    cli: &Cli,
    args: &cli::calendar::CalendarArgs,
    output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    use cli::calendar::{AclCommands, CalendarCommands};

    match &args.command {
        CalendarCommands::Calendars { max: _ } => run_calendar_list(cli, output_opts).await,
        CalendarCommands::Events(a) => run_calendar_events(cli, a, output_opts).await,
        CalendarCommands::Event(a) | CalendarCommands::Get(a) => {
            run_calendar_event_get(cli, a, output_opts).await
        }
        CalendarCommands::Search {
            query,
            calendar_id,
            max,
        } => run_calendar_search(cli, query, calendar_id.as_deref(), *max, output_opts).await,
        CalendarCommands::FreeBusy {
            start,
            end,
            calendars,
        } => run_calendar_freebusy(cli, start, end, calendars, output_opts).await,
        CalendarCommands::Colors => run_calendar_colors(cli, output_opts).await,
        CalendarCommands::Acl { calendar_id, sub } => {
            let cal_id = calendar_id.as_deref().unwrap_or("primary");
            match sub {
                Some(AclCommands::List) | None => {
                    run_calendar_acl_list(cli, cal_id, output_opts).await
                }
                Some(AclCommands::Get { .. })
                | Some(AclCommands::Insert { .. })
                | Some(AclCommands::Delete { .. }) => {
                    Err(read_only_err("calendar acl write ops"))
                }
            }
        }
        CalendarCommands::Time { timezone: _ } => {
            println!("{}", chrono::Utc::now().to_rfc3339());
            Ok(())
        }

        // Write/compound ops not wired yet.
        CalendarCommands::Create { .. }
        | CalendarCommands::Update { .. }
        | CalendarCommands::Respond { .. }
        | CalendarCommands::ProposeTimes { .. }
        | CalendarCommands::Conflicts { .. }
        | CalendarCommands::Team { .. }
        | CalendarCommands::Users { .. }
        | CalendarCommands::FocusTime { .. }
        | CalendarCommands::OutOfOffice { .. }
        | CalendarCommands::WorkingLocation { .. } => Err(cog_core::Error::Other(
            "calendar: only read-only subcommands are wired yet — available: \
             calendars, events, event/get, search, freebusy, colors, acl list, time".into(),
        )),
        #[cfg(feature = "destructive-permanent")]
        CalendarCommands::Delete { .. } => Err(read_only_err("calendar delete")),
    }
}

async fn run_calendar_list(
    cli: &Cli,
    output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    let cal = load_calendar_service(cli).await?;
    let (calendars, next_page_token) = cal.calendars(None).await?;
    output::print_output(
        &serde_json::json!({ "calendars": calendars, "nextPageToken": next_page_token }),
        output_opts,
    )
    .map_err(|e| cog_core::Error::Other(e.to_string()))
}

async fn run_calendar_events(
    cli: &Cli,
    args: &cli::calendar::EventsArgs,
    output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    let cal = load_calendar_service(cli).await?;
    let calendar_id = args.calendar_id.as_deref().unwrap_or("primary");
    let (time_min, time_max) = resolve_event_window(args);
    let (events, next_page_token) = cal
        .events(
            calendar_id,
            time_min.as_deref(),
            time_max.as_deref(),
            args.max,
            None,
        )
        .await?;
    output::print_output(
        &serde_json::json!({ "events": events, "nextPageToken": next_page_token }),
        output_opts,
    )
    .map_err(|e| cog_core::Error::Other(e.to_string()))
}

/// Translate the EventsArgs convenience flags (today/tomorrow/week/days/from/to)
/// into RFC 3339 `timeMin`/`timeMax` strings.
fn resolve_event_window(args: &cli::calendar::EventsArgs) -> (Option<String>, Option<String>) {
    use chrono::{Duration, Utc};

    if args.all {
        return (None, None);
    }
    if let Some(days) = args.days {
        let now = Utc::now();
        return (
            Some(now.to_rfc3339()),
            Some((now + Duration::days(days as i64)).to_rfc3339()),
        );
    }
    if args.today {
        let now = Utc::now();
        let end = now + Duration::days(1);
        return (Some(now.to_rfc3339()), Some(end.to_rfc3339()));
    }
    if args.tomorrow {
        let start = Utc::now() + Duration::days(1);
        let end = start + Duration::days(1);
        return (Some(start.to_rfc3339()), Some(end.to_rfc3339()));
    }
    if args.week {
        let now = Utc::now();
        return (
            Some(now.to_rfc3339()),
            Some((now + Duration::days(7)).to_rfc3339()),
        );
    }

    // Explicit from/to override the conveniences above.
    let from = args.from.clone();
    let to = args.to.clone();
    if from.is_some() || to.is_some() {
        return (from, to);
    }

    // Default: next 7 days from now.
    let now = Utc::now();
    (
        Some(now.to_rfc3339()),
        Some((now + Duration::days(7)).to_rfc3339()),
    )
}

async fn run_calendar_event_get(
    cli: &Cli,
    args: &cli::calendar::EventGetArgs,
    output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    let cal = load_calendar_service(cli).await?;
    let calendar_id = args.calendar_id.as_deref().unwrap_or("primary");
    let event = cal.event_get(calendar_id, &args.id).await?;
    output::print_output(&event, output_opts)
        .map_err(|e| cog_core::Error::Other(e.to_string()))
}

async fn run_calendar_search(
    cli: &Cli,
    query: &str,
    calendar_id: Option<&str>,
    max: Option<u32>,
    output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    let cal = load_calendar_service(cli).await?;
    let cal_id = calendar_id.unwrap_or("primary");
    let events = cal.search(cal_id, query, None, None, max).await?;
    output::print_output(&serde_json::json!({ "events": events }), output_opts)
        .map_err(|e| cog_core::Error::Other(e.to_string()))
}

async fn run_calendar_freebusy(
    cli: &Cli,
    start: &str,
    end: &str,
    calendars: &[String],
    output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    let cal = load_calendar_service(cli).await?;
    let ids: Vec<String> = if calendars.is_empty() {
        vec!["primary".to_string()]
    } else {
        calendars.to_vec()
    };
    let resp = cal.freebusy(start, end, &ids).await?;
    output::print_output(&resp, output_opts)
        .map_err(|e| cog_core::Error::Other(e.to_string()))
}

async fn run_calendar_colors(
    cli: &Cli,
    output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    let cal = load_calendar_service(cli).await?;
    let colors = cal.colors().await?;
    output::print_output(&colors, output_opts)
        .map_err(|e| cog_core::Error::Other(e.to_string()))
}

async fn run_calendar_acl_list(
    cli: &Cli,
    calendar_id: &str,
    output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    let cal = load_calendar_service(cli).await?;
    let (rules, next_page_token) = cal.acl(calendar_id, None).await?;
    output::print_output(
        &serde_json::json!({ "acl": rules, "nextPageToken": next_page_token }),
        output_opts,
    )
    .map_err(|e| cog_core::Error::Other(e.to_string()))
}

async fn run_open(id: &str) -> cog_core::Result<()> {
    // TODO: Generate appropriate URL based on resource type
    println!("https://drive.google.com/file/d/{id}/view");
    Ok(())
}

async fn run_config(
    args: &cli::config::ConfigArgs,
    output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    use cli::config::ConfigCommands;

    match &args.command {
        ConfigCommands::Path => {
            let home = cog_core::cog_home()?;
            println!("{}", home.display());
            Ok(())
        }
        ConfigCommands::List => {
            let config = cog_core::Config::load()?;
            output::print_output(&config, output_opts)
                .map_err(|e| cog_core::Error::Other(e.to_string()))
        }
        ConfigCommands::Keys => {
            let keys = [
                "keyring_backend",
                "default_timezone",
                "account_aliases",
                "account_clients",
                "client_domains",
            ];
            for key in keys {
                println!("{key}");
            }
            Ok(())
        }
        ConfigCommands::Get { key } => {
            let config = cog_core::Config::load()?;
            let value = serde_json::to_value(&config)
                .map_err(|e| cog_core::Error::Other(e.to_string()))?;
            if let Some(v) = value.get(key) {
                println!("{}", serde_json::to_string_pretty(v)
                    .map_err(|e| cog_core::Error::Other(e.to_string()))?);
            } else {
                return Err(cog_core::Error::NotFound(format!("config key not found: {key}")));
            }
            Ok(())
        }
        ConfigCommands::Set { key, value } => {
            let mut config = cog_core::Config::load()?;
            let mut map = serde_json::to_value(&config)
                .map_err(|e| cog_core::Error::Other(e.to_string()))?;
            if let serde_json::Value::Object(ref mut obj) = map {
                obj.insert(key.clone(), serde_json::Value::String(value.clone()));
            }
            config = serde_json::from_value(map)
                .map_err(|e| cog_core::Error::Other(e.to_string()))?;
            config.save()?;
            Ok(())
        }
        ConfigCommands::Unset { key } => {
            let mut config = cog_core::Config::load()?;
            let mut map = serde_json::to_value(&config)
                .map_err(|e| cog_core::Error::Other(e.to_string()))?;
            if let serde_json::Value::Object(ref mut obj) = map {
                obj.remove(key);
            }
            config = serde_json::from_value(map)
                .map_err(|e| cog_core::Error::Other(e.to_string()))?;
            config.save()?;
            Ok(())
        }
    }
}

fn run_completion(shell: &str) -> cog_core::Result<()> {
    use clap::CommandFactory;
    let mut cmd = Cli::command();
    let shell = match shell {
        "bash" => clap_complete::Shell::Bash,
        "zsh" => clap_complete::Shell::Zsh,
        "fish" => clap_complete::Shell::Fish,
        "powershell" => clap_complete::Shell::PowerShell,
        _ => return Err(cog_core::Error::Other(format!("unsupported shell: {shell}"))),
    };
    clap_complete::generate(shell, &mut cmd, "cog", &mut std::io::stdout());
    Ok(())
}

async fn run_schema(output_opts: &OutputOptions) -> cog_core::Result<()> {
    let services = serde_json::json!({
        "services": [
            {"name": "gmail", "scope": "gmail.modify"},
            {"name": "calendar", "scope": "calendar"},
            {"name": "drive", "scope": "drive"},
            {"name": "docs", "scope": "documents"},
            {"name": "sheets", "scope": "spreadsheets"},
            {"name": "slides", "scope": "presentations"},
            {"name": "forms", "scope": "forms.body"},
            {"name": "contacts", "scope": "contacts"},
            {"name": "tasks", "scope": "tasks"},
            {"name": "people", "scope": "directory.readonly"},
            {"name": "chat", "scope": "chat.messages"},
            {"name": "classroom", "scope": "classroom.courses"},
            {"name": "keep", "scope": "keep (workspace only)"},
            {"name": "appscript", "scope": "script.projects"},
            {"name": "groups", "scope": "cloud-identity.groups.readonly"},
        ]
    });
    output::print_output(&services, output_opts)
        .map_err(|e| cog_core::Error::Other(e.to_string()))
}

async fn run_exit_codes(output_opts: &OutputOptions) -> cog_core::Result<()> {
    let codes = serde_json::json!({
        "exit_codes": [
            {"code": 0, "meaning": "ok"},
            {"code": 1, "meaning": "error"},
            {"code": 2, "meaning": "usage"},
            {"code": 3, "meaning": "empty"},
            {"code": 4, "meaning": "auth-required"},
            {"code": 5, "meaning": "not-found"},
            {"code": 6, "meaning": "permission-denied"},
            {"code": 7, "meaning": "rate-limited"},
            {"code": 8, "meaning": "retryable"},
            {"code": 10, "meaning": "config"},
            {"code": 130, "meaning": "cancelled"},
        ]
    });
    output::print_output(&codes, output_opts)
        .map_err(|e| cog_core::Error::Other(e.to_string()))
}

fn command_name(cmd: &Commands) -> &'static str {
    match cmd {
        Commands::Auth(_) => "auth",
        Commands::Gmail(_) => "gmail",
        Commands::Calendar(_) => "calendar",
        Commands::Drive(_) => "drive",
        Commands::Docs(_) => "docs",
        Commands::Sheets(_) => "sheets",
        Commands::Slides(_) => "slides",
        Commands::Forms(_) => "forms",
        Commands::Contacts(_) => "contacts",
        Commands::Tasks(_) => "tasks",
        Commands::People(_) => "people",
        Commands::Chat(_) => "chat",
        Commands::Classroom(_) => "classroom",
        Commands::Groups(_) => "groups",
        Commands::Keep(_) => "keep",
        Commands::AppScript(_) => "appscript",
        Commands::Config(_) => "config",
        Commands::Time(_) => "time",
        Commands::Completion { .. } => "completion",
        Commands::Mcp => "mcp",
        Commands::Version => "version",
        Commands::Schema => "schema",
        Commands::Agent { .. } => "agent",
        Commands::Send(_) => "gmail",
        Commands::Ls(_) => "drive",
        Commands::Search(_) => "drive",
        Commands::Open { .. } => "open",
        Commands::Download(_) => "drive",
        Commands::Upload(_) => "drive",
        Commands::Login(_) => "auth",
        Commands::Logout(_) => "auth",
        Commands::Status => "auth",
        Commands::Me | Commands::WhoAmI => "people",
        #[cfg(feature = "gemini-web")]
        Commands::Gemini(_) => "gemini",
        #[cfg(feature = "notebooklm")]
        Commands::NotebookLm(_) => "notebooklm",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_listen_spec_ip_and_port() {
        let (host, port) = parse_listen_spec("127.0.0.1:44760").unwrap();
        assert_eq!(host, "127.0.0.1");
        assert_eq!(port, 44760);
    }

    #[test]
    fn parse_listen_spec_bare_ip_defaults_port() {
        let (host, port) = parse_listen_spec("100.64.0.5").unwrap();
        assert_eq!(host, "100.64.0.5");
        assert_eq!(port, OAUTH_PORT_BASE);
    }

    #[test]
    fn parse_listen_spec_port_only() {
        let (host, port) = parse_listen_spec(":50000").unwrap();
        assert_eq!(host, "127.0.0.1");
        assert_eq!(port, 50000);
    }

    #[test]
    fn parse_listen_spec_hostname_port() {
        let (host, port) = parse_listen_spec("mini.tailnet.ts.net:8080").unwrap();
        assert_eq!(host, "mini.tailnet.ts.net");
        assert_eq!(port, 8080);
    }

    #[test]
    fn parse_listen_spec_rejects_wildcard() {
        assert!(parse_listen_spec("0.0.0.0:44760").is_err());
        assert!(parse_listen_spec("::").is_err());
        assert!(parse_listen_spec("[::]").is_err());
    }

    #[test]
    fn parse_listen_spec_rejects_empty() {
        assert!(parse_listen_spec("").is_err());
    }

    #[test]
    fn parse_listen_spec_non_numeric_port_treated_as_bare_host() {
        // "bad:port" has a non-digit port component, so the rsplit_once arm
        // falls through and the whole string is treated as a bare host with
        // the default port.
        let (host, port) = parse_listen_spec("bad:port").unwrap();
        assert_eq!(host, "bad:port");
        assert_eq!(port, OAUTH_PORT_BASE);
    }
}
