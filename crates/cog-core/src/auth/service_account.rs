use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Google service account credentials.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceAccountKey {
    #[serde(rename = "type")]
    pub key_type: String,
    pub project_id: String,
    pub private_key_id: String,
    pub private_key: String,
    pub client_email: String,
    pub client_id: String,
    pub auth_uri: String,
    pub token_uri: String,
    pub auth_provider_x509_cert_url: String,
    pub client_x509_cert_url: String,
}

impl ServiceAccountKey {
    /// Load a service account key from a JSON file.
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        serde_json::from_str(&content).map_err(|e| {
            Error::Config(format!(
                "invalid service account key at {}: {e}",
                path.display()
            ))
        })
    }

    /// Get the service account email.
    pub fn email(&self) -> &str {
        &self.client_email
    }
}

/// Service account authentication with domain-wide delegation.
#[derive(Debug)]
pub struct ServiceAccountAuth {
    key: ServiceAccountKey,
    subject: Option<String>,
}

impl ServiceAccountAuth {
    /// Create a new service account authenticator.
    pub fn new(key: ServiceAccountKey) -> Self {
        Self { key, subject: None }
    }

    /// Set the subject (impersonated user) for domain-wide delegation.
    pub fn with_subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }

    /// Get an access token for the given scopes.
    pub async fn get_token(&self, _scopes: &[&str]) -> Result<String> {
        // TODO: Implement JWT creation and token exchange
        // 1. Create JWT with claims (iss, scope, aud, sub, iat, exp)
        // 2. Sign with private key
        // 3. Exchange JWT for access token at token_uri
        let _ = &self.key;
        let _ = &self.subject;
        Err(Error::Other(
            "service account auth not yet implemented".into(),
        ))
    }
}
