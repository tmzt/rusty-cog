use serde::{Deserialize, Serialize};

/// OAuth2 client credentials (from Google Cloud Console).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientCredentials {
    pub installed: Option<InstalledCredentials>,
    pub web: Option<WebCredentials>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledCredentials {
    pub client_id: String,
    pub client_secret: String,
    pub auth_uri: String,
    pub token_uri: String,
    #[serde(default)]
    pub redirect_uris: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebCredentials {
    pub client_id: String,
    pub client_secret: String,
    pub auth_uri: String,
    pub token_uri: String,
    #[serde(default)]
    pub redirect_uris: Vec<String>,
}

impl ClientCredentials {
    /// Get the client ID regardless of credential type.
    pub fn client_id(&self) -> Option<&str> {
        self.installed
            .as_ref()
            .map(|c| c.client_id.as_str())
            .or_else(|| self.web.as_ref().map(|c| c.client_id.as_str()))
    }

    /// Get the client secret regardless of credential type.
    pub fn client_secret(&self) -> Option<&str> {
        self.installed
            .as_ref()
            .map(|c| c.client_secret.as_str())
            .or_else(|| self.web.as_ref().map(|c| c.client_secret.as_str()))
    }

    /// Get the token URI.
    pub fn token_uri(&self) -> Option<&str> {
        self.installed
            .as_ref()
            .map(|c| c.token_uri.as_str())
            .or_else(|| self.web.as_ref().map(|c| c.token_uri.as_str()))
    }

    /// Get the registered redirect URIs.
    pub fn redirect_uris(&self) -> &[String] {
        self.installed
            .as_ref()
            .map(|c| c.redirect_uris.as_slice())
            .or_else(|| self.web.as_ref().map(|c| c.redirect_uris.as_slice()))
            .unwrap_or(&[])
    }

    /// Whether this is a "web" type credential (vs "installed"/desktop).
    pub fn is_web(&self) -> bool {
        self.web.is_some()
    }
}

/// A stored OAuth2 token (refresh token + metadata).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredToken {
    pub email: String,
    pub refresh_token: String,
    pub scopes: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl StoredToken {
    /// Check if the access token is still valid (with 60s margin).
    pub fn is_valid(&self) -> bool {
        match (&self.access_token, &self.expires_at) {
            (Some(_), Some(expires)) => {
                *expires > chrono::Utc::now() + chrono::Duration::seconds(60)
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_credentials_installed() {
        let json = r#"{"installed":{"client_id":"id","client_secret":"secret","auth_uri":"https://auth","token_uri":"https://token","redirect_uris":["http://localhost"]}}"#;
        let creds: ClientCredentials = serde_json::from_str(json).unwrap();
        assert_eq!(creds.client_id(), Some("id"));
        assert_eq!(creds.client_secret(), Some("secret"));
    }

    #[test]
    fn stored_token_expired() {
        let token = StoredToken {
            email: "test@example.com".into(),
            refresh_token: "refresh".into(),
            scopes: vec!["gmail".into()],
            client_name: None,
            access_token: Some("access".into()),
            expires_at: Some(chrono::Utc::now() - chrono::Duration::seconds(10)),
        };
        assert!(!token.is_valid());
    }
}
