use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Resolves the COG_HOME directory.
///
/// Priority:
/// 1. `$COG_HOME` environment variable
/// 2. `~/.config/cog/`
pub fn cog_home() -> Result<PathBuf> {
    if let Ok(home) = std::env::var("COG_HOME") {
        return Ok(PathBuf::from(home));
    }

    dirs::config_dir()
        .map(|d| d.join("cog"))
        .ok_or_else(|| Error::Config("cannot determine config directory".into()))
}

/// Resolves the UDS socket path.
///
/// Priority:
/// 1. `$COG_HOME/cog.sock`
/// 2. `$XDG_RUNTIME_DIR/cog.sock`
/// 3. `/tmp/cog-{uid}.sock`
pub fn socket_path() -> Result<PathBuf> {
    if let Ok(home) = cog_home() {
        return Ok(home.join("cog.sock"));
    }

    if let Ok(runtime) = std::env::var("XDG_RUNTIME_DIR") {
        return Ok(PathBuf::from(runtime).join("cog.sock"));
    }

    #[cfg(unix)]
    {
        let uid = unsafe { libc::getuid() };
        Ok(PathBuf::from(format!("/tmp/cog-{uid}.sock")))
    }

    #[cfg(not(unix))]
    Err(Error::Config("cannot determine socket path".into()))
}

/// Path to external subcommand directory.
pub fn external_commands_dir() -> Result<PathBuf> {
    dirs::home_dir()
        .map(|d| d.join(".rustycog").join("bin"))
        .ok_or_else(|| Error::Config("cannot determine home directory".into()))
}

/// Load OAuth client credentials from `$COG_HOME/credentials.json`
/// or `$COG_HOME/credentials-{client}.json` for a named client.
pub fn load_credentials(client_name: Option<&str>) -> Result<crate::auth::ClientCredentials> {
    let home = cog_home()?;
    let path = match client_name {
        Some(name) => home.join(format!("credentials-{name}.json")),
        None => home.join("credentials.json"),
    };

    if !path.exists() {
        return Err(Error::Config(format!(
            "credentials file not found: {}\nDownload OAuth credentials from Google Cloud Console and save them there.",
            path.display()
        )));
    }

    let content = std::fs::read_to_string(&path)?;
    serde_json::from_str(&content)
        .map_err(|e| Error::Config(format!("invalid credentials file {}: {e}", path.display())))
}

/// Main configuration file.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Keyring backend: "auto", "keychain", "file"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keyring_backend: Option<String>,

    /// Default timezone (IANA, "UTC", or "local")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_timezone: Option<String>,

    /// Account aliases: alias -> email
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub account_aliases: HashMap<String, String>,

    /// Account-specific OAuth client: email -> client name
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub account_clients: HashMap<String, String>,

    /// Domain-specific OAuth client: domain -> client name
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub client_domains: HashMap<String, String>,
}

impl Config {
    /// Load configuration from `$COG_HOME/config.json5`.
    pub fn load() -> Result<Self> {
        let path = cog_home()?.join("config.json5");
        Self::load_from(&path)
    }

    /// Load configuration from a specific path.
    pub fn load_from(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }

        let content = std::fs::read_to_string(path)?;
        json5::from_str(&content).map_err(|e| Error::Config(format!("invalid config: {e}")))
    }

    /// Save configuration to `$COG_HOME/config.json5`.
    pub fn save(&self) -> Result<()> {
        let home = cog_home()?;
        std::fs::create_dir_all(&home)?;
        let path = home.join("config.json5");
        self.save_to(&path)
    }

    /// Save configuration to a specific path.
    pub fn save_to(&self, path: &Path) -> Result<()> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| Error::Config(format!("serialize config: {e}")))?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Resolve an account identifier (alias or email) to an email.
    pub fn resolve_account(&self, account: &str) -> String {
        self.account_aliases
            .get(account)
            .cloned()
            .unwrap_or_else(|| account.to_string())
    }

    /// Get the OAuth client name for an account.
    pub fn client_for_account(&self, email: &str) -> Option<&str> {
        if let Some(client) = self.account_clients.get(email) {
            return Some(client.as_str());
        }

        if let Some(domain) = email.split('@').nth(1) {
            if let Some(client) = self.client_domains.get(domain) {
                return Some(client.as_str());
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_roundtrips() {
        let config = Config::default();
        let json = serde_json::to_string(&config).unwrap();
        let parsed: Config = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.keyring_backend, config.keyring_backend);
    }

    #[test]
    fn resolve_alias() {
        let mut config = Config::default();
        config
            .account_aliases
            .insert("work".into(), "user@company.com".into());
        assert_eq!(config.resolve_account("work"), "user@company.com");
        assert_eq!(config.resolve_account("other@gmail.com"), "other@gmail.com");
    }

    #[test]
    fn client_for_domain() {
        let mut config = Config::default();
        config
            .client_domains
            .insert("company.com".into(), "enterprise".into());
        assert_eq!(
            config.client_for_account("user@company.com"),
            Some("enterprise")
        );
        assert_eq!(config.client_for_account("user@gmail.com"), None);
    }
}
