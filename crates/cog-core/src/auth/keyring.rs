use crate::auth::credentials::StoredToken;
use crate::error::{Error, Result};
use std::path::PathBuf;

const SERVICE_NAME: &str = "cog";
const ACCOUNTS_KEY: &str = "__cog_accounts__";

/// Keyring backend for storing OAuth2 tokens.
#[derive(Debug, Clone)]
pub enum KeyringBackend {
    /// OS-native keyring (macOS Keychain, Linux Secret Service).
    Native,
    /// File-based fallback at `$COG_HOME/keyring/`.
    File(PathBuf),
}

impl KeyringBackend {
    /// Determine the keyring backend from config or environment.
    pub fn from_config(backend: Option<&str>) -> Self {
        let backend = backend
            .or_else(|| std::env::var("COG_KEYRING_BACKEND").ok().as_deref().map(|_| "auto"))
            .unwrap_or("auto");

        match backend {
            "file" => {
                let path = crate::config::cog_home()
                    .unwrap_or_else(|_| PathBuf::from("."))
                    .join("keyring");
                Self::File(path)
            }
            "keychain" | "native" => Self::Native,
            _ => Self::Native, // "auto" defaults to native
        }
    }

    /// Store a token for an account.
    pub fn store(&self, token: &StoredToken) -> Result<()> {
        match self {
            Self::Native => self.store_native(token),
            Self::File(dir) => self.store_file(dir, token),
        }
    }

    /// Retrieve a token for an account.
    pub fn get(&self, email: &str) -> Result<Option<StoredToken>> {
        match self {
            Self::Native => self.get_native(email),
            Self::File(dir) => self.get_file(dir, email),
        }
    }

    /// Remove a token for an account.
    pub fn remove(&self, email: &str) -> Result<()> {
        match self {
            Self::Native => self.remove_native(email),
            Self::File(dir) => self.remove_file(dir, email),
        }
    }

    /// List all stored account emails.
    pub fn list(&self) -> Result<Vec<String>> {
        match self {
            Self::Native => self.list_native(),
            Self::File(dir) => self.list_file(dir),
        }
    }

    // -- native (OS keyring) ------------------------------------------------

    fn store_native(&self, token: &StoredToken) -> Result<()> {
        let json = serde_json::to_string(token)
            .map_err(|e| Error::Keyring(format!("serialize token: {e}")))?;
        let entry = keyring::Entry::new(SERVICE_NAME, &token.email)
            .map_err(|e| Error::Keyring(e.to_string()))?;
        entry
            .set_password(&json)
            .map_err(|e| Error::Keyring(e.to_string()))?;

        // Maintain the account index
        let mut accounts = self.list_native().unwrap_or_default();
        if !accounts.contains(&token.email) {
            accounts.push(token.email.clone());
            self.save_account_index(&accounts)?;
        }
        Ok(())
    }

    fn get_native(&self, email: &str) -> Result<Option<StoredToken>> {
        let entry = keyring::Entry::new(SERVICE_NAME, email)
            .map_err(|e| Error::Keyring(e.to_string()))?;
        match entry.get_password() {
            Ok(json) => {
                let token: StoredToken = serde_json::from_str(&json)
                    .map_err(|e| Error::Keyring(format!("deserialize token: {e}")))?;
                Ok(Some(token))
            }
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(Error::Keyring(e.to_string())),
        }
    }

    fn remove_native(&self, email: &str) -> Result<()> {
        let entry = keyring::Entry::new(SERVICE_NAME, email)
            .map_err(|e| Error::Keyring(e.to_string()))?;
        match entry.delete_credential() {
            Ok(()) => {}
            Err(keyring::Error::NoEntry) => {}
            Err(e) => return Err(Error::Keyring(e.to_string())),
        }

        // Update the account index
        let mut accounts = self.list_native().unwrap_or_default();
        accounts.retain(|a| a != email);
        self.save_account_index(&accounts)?;
        Ok(())
    }

    fn list_native(&self) -> Result<Vec<String>> {
        let entry = keyring::Entry::new(SERVICE_NAME, ACCOUNTS_KEY)
            .map_err(|e| Error::Keyring(e.to_string()))?;
        match entry.get_password() {
            Ok(json) => {
                let accounts: Vec<String> = serde_json::from_str(&json)
                    .map_err(|e| Error::Keyring(format!("deserialize index: {e}")))?;
                Ok(accounts)
            }
            Err(keyring::Error::NoEntry) => Ok(Vec::new()),
            Err(e) => Err(Error::Keyring(e.to_string())),
        }
    }

    fn save_account_index(&self, accounts: &[String]) -> Result<()> {
        let json = serde_json::to_string(accounts)
            .map_err(|e| Error::Keyring(format!("serialize index: {e}")))?;
        let entry = keyring::Entry::new(SERVICE_NAME, ACCOUNTS_KEY)
            .map_err(|e| Error::Keyring(e.to_string()))?;
        entry
            .set_password(&json)
            .map_err(|e| Error::Keyring(e.to_string()))?;
        Ok(())
    }

    // -- file-based fallback ------------------------------------------------

    fn store_file(&self, dir: &std::path::Path, token: &StoredToken) -> Result<()> {
        std::fs::create_dir_all(dir)?;
        let path = dir.join(format!("{}.json", token.email));
        let content = serde_json::to_string_pretty(token)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    fn get_file(&self, dir: &std::path::Path, email: &str) -> Result<Option<StoredToken>> {
        let path = dir.join(format!("{email}.json"));
        if !path.exists() {
            return Ok(None);
        }
        let content = std::fs::read_to_string(path)?;
        let token: StoredToken = serde_json::from_str(&content)?;
        Ok(Some(token))
    }

    fn remove_file(&self, dir: &std::path::Path, email: &str) -> Result<()> {
        let path = dir.join(format!("{email}.json"));
        if path.exists() {
            std::fs::remove_file(path)?;
        }
        Ok(())
    }

    fn list_file(&self, dir: &std::path::Path) -> Result<Vec<String>> {
        if !dir.exists() {
            return Ok(Vec::new());
        }
        let mut emails = Vec::new();
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            if let Some(name) = entry.path().file_stem() {
                if let Some(name) = name.to_str() {
                    if entry.path().extension().is_some_and(|e| e == "json") {
                        emails.push(name.to_string());
                    }
                }
            }
        }
        Ok(emails)
    }
}
