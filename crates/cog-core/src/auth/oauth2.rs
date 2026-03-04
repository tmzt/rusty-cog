use crate::auth::credentials::{ClientCredentials, StoredToken};
use crate::error::{Error, Result};
use crate::http::HttpClient;
use serde::Deserialize;

const USERINFO_URL: &str = "https://www.googleapis.com/oauth2/v2/userinfo";

/// Google API OAuth2 scopes.
pub mod scopes {
    pub const GMAIL: &str = "https://www.googleapis.com/auth/gmail.modify";
    pub const GMAIL_READONLY: &str = "https://www.googleapis.com/auth/gmail.readonly";
    pub const GMAIL_SEND: &str = "https://www.googleapis.com/auth/gmail.send";
    pub const CALENDAR: &str = "https://www.googleapis.com/auth/calendar";
    pub const CALENDAR_READONLY: &str = "https://www.googleapis.com/auth/calendar.readonly";
    pub const DRIVE: &str = "https://www.googleapis.com/auth/drive";
    pub const DRIVE_READONLY: &str = "https://www.googleapis.com/auth/drive.readonly";
    pub const DRIVE_FILE: &str = "https://www.googleapis.com/auth/drive.file";
    pub const DOCS: &str = "https://www.googleapis.com/auth/documents";
    pub const DOCS_READONLY: &str = "https://www.googleapis.com/auth/documents.readonly";
    pub const SHEETS: &str = "https://www.googleapis.com/auth/spreadsheets";
    pub const SHEETS_READONLY: &str = "https://www.googleapis.com/auth/spreadsheets.readonly";
    pub const SLIDES: &str = "https://www.googleapis.com/auth/presentations";
    pub const SLIDES_READONLY: &str = "https://www.googleapis.com/auth/presentations.readonly";
    pub const FORMS: &str = "https://www.googleapis.com/auth/forms.body";
    pub const FORMS_READONLY: &str = "https://www.googleapis.com/auth/forms.body.readonly";
    pub const FORMS_RESPONSES: &str = "https://www.googleapis.com/auth/forms.responses.readonly";
    pub const CONTACTS: &str = "https://www.googleapis.com/auth/contacts";
    pub const CONTACTS_READONLY: &str = "https://www.googleapis.com/auth/contacts.readonly";
    pub const TASKS: &str = "https://www.googleapis.com/auth/tasks";
    pub const TASKS_READONLY: &str = "https://www.googleapis.com/auth/tasks.readonly";
    pub const PEOPLE: &str = "https://www.googleapis.com/auth/directory.readonly";
    pub const CHAT: &str = "https://www.googleapis.com/auth/chat.messages";
    pub const CHAT_READONLY: &str = "https://www.googleapis.com/auth/chat.messages.readonly";
    pub const CHAT_SPACES: &str = "https://www.googleapis.com/auth/chat.spaces";
    pub const CLASSROOM: &str = "https://www.googleapis.com/auth/classroom.courses";
    pub const CLASSROOM_READONLY: &str =
        "https://www.googleapis.com/auth/classroom.courses.readonly";
    pub const KEEP: &str = "https://www.googleapis.com/auth/keep";
    pub const KEEP_READONLY: &str = "https://www.googleapis.com/auth/keep.readonly";
    pub const APPSCRIPT: &str = "https://www.googleapis.com/auth/script.projects";
    pub const GROUPS: &str = "https://www.googleapis.com/auth/cloud-identity.groups.readonly";
    pub const USERINFO: &str = "https://www.googleapis.com/auth/userinfo.email";
}

/// Response from Google's OAuth2 token endpoint.
#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    #[serde(default)]
    refresh_token: Option<String>,
    expires_in: i64,
    #[serde(default)]
    scope: Option<String>,
    token_type: String,
}

/// Minimal userinfo response for extracting the account email.
#[derive(Debug, Deserialize)]
struct UserInfo {
    email: String,
}

/// OAuth2 client for Google API authorization.
#[derive(Debug)]
pub struct OAuth2Client {
    credentials: ClientCredentials,
    http: HttpClient,
}

impl OAuth2Client {
    /// Create a new OAuth2 client from credentials.
    pub fn new(credentials: ClientCredentials, http: HttpClient) -> Self {
        Self { credentials, http }
    }

    /// Generate the authorization URL for the browser-based flow.
    pub fn authorization_url(&self, scopes: &[&str], redirect_uri: &str) -> Result<String> {
        let client_id = self
            .credentials
            .client_id()
            .ok_or_else(|| Error::OAuth2("missing client_id".into()))?;

        let scope_str = scopes.join(" ");
        Ok(format!(
            "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope={}&access_type=offline&prompt=consent",
            urlencoding(client_id),
            urlencoding(redirect_uri),
            urlencoding(&scope_str),
        ))
    }

    /// Exchange an authorization code for tokens.
    pub async fn exchange_code(
        &self,
        code: &str,
        redirect_uri: &str,
        scopes: &[&str],
        client_name: Option<&str>,
    ) -> Result<StoredToken> {
        let client_id = self
            .credentials
            .client_id()
            .ok_or_else(|| Error::OAuth2("missing client_id".into()))?;
        let client_secret = self
            .credentials
            .client_secret()
            .ok_or_else(|| Error::OAuth2("missing client_secret".into()))?;
        let token_uri = self
            .credentials
            .token_uri()
            .unwrap_or("https://oauth2.googleapis.com/token");

        let params = [
            ("grant_type", "authorization_code"),
            ("code", code),
            ("client_id", client_id),
            ("client_secret", client_secret),
            ("redirect_uri", redirect_uri),
        ];

        let resp_bytes = self.http.post_form(token_uri, &params).await?;
        let token_resp: TokenResponse = serde_json::from_slice(&resp_bytes)
            .map_err(|e| Error::OAuth2(format!("invalid token response: {e}")))?;

        let refresh_token = token_resp
            .refresh_token
            .ok_or_else(|| Error::OAuth2("no refresh_token in response".into()))?;

        // Fetch the user's email via userinfo
        let email = self.fetch_email(&token_resp.access_token).await?;

        let expires_at = chrono::Utc::now()
            + chrono::Duration::seconds(token_resp.expires_in);

        Ok(StoredToken {
            email,
            refresh_token,
            scopes: scopes.iter().map(|s| s.to_string()).collect(),
            client_name: client_name.map(String::from),
            access_token: Some(token_resp.access_token),
            expires_at: Some(expires_at),
        })
    }

    /// Refresh an access token using a refresh token.
    pub async fn refresh_token(&self, token: &mut StoredToken) -> Result<()> {
        let client_id = self
            .credentials
            .client_id()
            .ok_or_else(|| Error::OAuth2("missing client_id".into()))?;
        let client_secret = self
            .credentials
            .client_secret()
            .ok_or_else(|| Error::OAuth2("missing client_secret".into()))?;
        let token_uri = self
            .credentials
            .token_uri()
            .unwrap_or("https://oauth2.googleapis.com/token");

        let params = [
            ("grant_type", "refresh_token"),
            ("refresh_token", &token.refresh_token),
            ("client_id", client_id),
            ("client_secret", client_secret),
        ];

        let resp_bytes = self.http.post_form(token_uri, &params).await?;
        let token_resp: TokenResponse = serde_json::from_slice(&resp_bytes)
            .map_err(|e| Error::OAuth2(format!("invalid token response: {e}")))?;

        token.access_token = Some(token_resp.access_token);
        token.expires_at = Some(
            chrono::Utc::now() + chrono::Duration::seconds(token_resp.expires_in),
        );

        // Google may issue a new refresh token
        if let Some(new_refresh) = token_resp.refresh_token {
            token.refresh_token = new_refresh;
        }

        Ok(())
    }

    /// Get a valid access token, refreshing if needed.
    pub async fn get_access_token(&self, token: &mut StoredToken) -> Result<String> {
        if !token.is_valid() {
            self.refresh_token(token).await?;
        }
        token
            .access_token
            .clone()
            .ok_or_else(|| Error::OAuth2("no access token after refresh".into()))
    }

    async fn fetch_email(&self, access_token: &str) -> Result<String> {
        let resp = self.http.get(USERINFO_URL, access_token).await?;
        let info: UserInfo = serde_json::from_slice(&resp)
            .map_err(|e| Error::OAuth2(format!("userinfo parse error: {e}")))?;
        Ok(info.email)
    }

    /// Get scopes for a set of service names.
    pub fn scopes_for_services(services: &[&str], readonly: bool) -> Vec<&'static str> {
        let mut result = vec![scopes::USERINFO];

        for service in services {
            let scope = match (*service, readonly) {
                ("gmail", true) => scopes::GMAIL_READONLY,
                ("gmail", false) => scopes::GMAIL,
                ("calendar", true) => scopes::CALENDAR_READONLY,
                ("calendar", false) => scopes::CALENDAR,
                ("drive", true) => scopes::DRIVE_READONLY,
                ("drive", false) => scopes::DRIVE,
                ("docs", true) => scopes::DOCS_READONLY,
                ("docs", false) => scopes::DOCS,
                ("sheets", true) => scopes::SHEETS_READONLY,
                ("sheets", false) => scopes::SHEETS,
                ("slides", true) => scopes::SLIDES_READONLY,
                ("slides", false) => scopes::SLIDES,
                ("forms", true) => scopes::FORMS_READONLY,
                ("forms", false) => scopes::FORMS,
                ("contacts", true) => scopes::CONTACTS_READONLY,
                ("contacts", false) => scopes::CONTACTS,
                ("tasks", true) => scopes::TASKS_READONLY,
                ("tasks", false) => scopes::TASKS,
                ("people", _) => scopes::PEOPLE,
                ("chat", true) => scopes::CHAT_READONLY,
                ("chat", false) => scopes::CHAT,
                ("classroom", true) => scopes::CLASSROOM_READONLY,
                ("classroom", false) => scopes::CLASSROOM,
                ("keep", true) => scopes::KEEP_READONLY,
                ("keep", false) => scopes::KEEP,
                ("appscript", _) => scopes::APPSCRIPT,
                ("groups", _) => scopes::GROUPS,
                _ => continue,
            };
            if !result.contains(&scope) {
                result.push(scope);
            }
        }

        result
    }
}

fn urlencoding(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scopes_for_gmail_readonly() {
        let scopes = OAuth2Client::scopes_for_services(&["gmail"], true);
        assert!(scopes.contains(&scopes::GMAIL_READONLY));
        assert!(!scopes.contains(&scopes::GMAIL));
    }

    #[test]
    fn scopes_for_multiple_services() {
        let scopes = OAuth2Client::scopes_for_services(&["gmail", "drive", "docs"], false);
        assert!(scopes.contains(&scopes::GMAIL));
        assert!(scopes.contains(&scopes::DRIVE));
        assert!(scopes.contains(&scopes::DOCS));
        assert!(scopes.contains(&scopes::USERINFO));
    }
}
