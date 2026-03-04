mod cli;
mod confirm;
mod exit;
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
        Commands::Gmail(_args) => {
            // TODO: Dispatch to gmail subcommands
            Err(cog_core::Error::Other("gmail commands not yet implemented".into()))
        }
        Commands::Calendar(_args) => {
            Err(cog_core::Error::Other("calendar commands not yet implemented".into()))
        }
        Commands::Drive(_args) => {
            Err(cog_core::Error::Other("drive commands not yet implemented".into()))
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

/// Default services to authorize if none specified.
const DEFAULT_LOGIN_SERVICES: &[&str] = &[
    "gmail", "calendar", "drive", "docs", "sheets", "slides",
    "forms", "contacts", "tasks", "people", "keep",
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

    // Determine redirect strategy based on credential type:
    //   - "installed" (desktop): loopback redirect on our port range
    //   - "web": use exact redirect URIs from credentials file
    let (listener, redirect_uri) = if credentials.is_web() {
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

async fn run_drive_ls(
    _cli: &Cli,
    _args: &cli::drive::LsArgs,
    _output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    Err(cog_core::Error::Other("drive ls not yet implemented".into()))
}

async fn run_drive_search(
    _cli: &Cli,
    _args: &cli::drive::SearchArgs,
    _output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    Err(cog_core::Error::Other("drive search not yet implemented".into()))
}

async fn run_drive_download(
    _cli: &Cli,
    _args: &cli::drive::DownloadArgs,
    _output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    Err(cog_core::Error::Other("drive download not yet implemented".into()))
}

async fn run_drive_upload(
    _cli: &Cli,
    _args: &cli::drive::UploadArgs,
    _output_opts: &OutputOptions,
) -> cog_core::Result<()> {
    Err(cog_core::Error::Other("drive upload not yet implemented".into()))
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
