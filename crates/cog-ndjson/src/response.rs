use serde::{Deserialize, Serialize};

/// The result of a request — either success or error.
///
/// Uses `#[serde(untagged)]` so that when flattened into CogResponse
/// we get `{"id": N, "result": ...}` or `{"id": N, "error": {...}}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponseResult {
    Ok(OkEnvelope),
    Err(ErrEnvelope),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OkEnvelope {
    pub result: ResponsePayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrEnvelope {
    pub error: ErrorResponse,
}

impl ResponseResult {
    pub fn ok(payload: ResponsePayload) -> Self {
        Self::Ok(OkEnvelope { result: payload })
    }

    pub fn err(error: ErrorResponse) -> Self {
        Self::Err(ErrEnvelope { error })
    }
}

// Convenience constructors (keep the short names used everywhere)
impl From<ResponsePayload> for ResponseResult {
    fn from(p: ResponsePayload) -> Self { Self::ok(p) }
}
impl From<ErrorResponse> for ResponseResult {
    fn from(e: ErrorResponse) -> Self { Self::err(e) }
}

/// Successful response payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponsePayload {
    /// Arbitrary JSON (most API responses).
    Json(serde_json::Value),
    /// Binary data — base64-encoded within JSON.
    Binary {
        content_type: String,
        #[serde(with = "base64_bytes")]
        data: Vec<u8>,
    },
    /// Pong.
    Pong,
    /// Shutdown acknowledgment.
    ShutdownAck,
    /// Empty success.
    Empty,
    /// Monitor subscription confirmation.
    MonitorSubscribed { services: Vec<String> },
    /// Monitor unsubscription confirmation.
    MonitorUnsubscribed { services: Vec<String> },
    /// Monitor status.
    MonitorStatus { subscriptions: Vec<MonitorSubscription> },
    /// Index query results.
    IndexResults {
        namespace: String,
        results: Vec<serde_json::Value>,
        total: usize,
    },
    /// Index refresh status.
    IndexRefreshStatus { namespaces: Vec<IndexNamespaceStatus> },
    /// Index overall status.
    IndexStatus { namespaces: Vec<IndexNamespaceStatus> },
    /// Auth status.
    AuthStatus { accounts: Vec<AccountStatus> },
}

/// Error response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: u32,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

pub mod error_codes {
    pub const INTERNAL: u32 = 1;
    pub const INVALID_REQUEST: u32 = 2;
    pub const NOT_FOUND: u32 = 3;
    pub const AUTH_REQUIRED: u32 = 4;
    pub const PERMISSION_DENIED: u32 = 5;
    pub const RATE_LIMITED: u32 = 6;
    pub const DESTRUCTIVE_DENIED: u32 = 7;
    pub const BULK_TRASH_DENIED: u32 = 8;
    pub const FEATURE_DISABLED: u32 = 9;
    pub const SHUTDOWN_IN_PROGRESS: u32 = 10;
}

impl ErrorResponse {
    pub fn internal(message: impl Into<String>) -> Self {
        Self { code: error_codes::INTERNAL, message: message.into(), details: None }
    }
    pub fn invalid_request(message: impl Into<String>) -> Self {
        Self { code: error_codes::INVALID_REQUEST, message: message.into(), details: None }
    }
    pub fn not_found(message: impl Into<String>) -> Self {
        Self { code: error_codes::NOT_FOUND, message: message.into(), details: None }
    }
    pub fn auth_required(message: impl Into<String>) -> Self {
        Self { code: error_codes::AUTH_REQUIRED, message: message.into(), details: None }
    }
    pub fn permission_denied(message: impl Into<String>) -> Self {
        Self { code: error_codes::PERMISSION_DENIED, message: message.into(), details: None }
    }
    pub fn rate_limited(message: impl Into<String>) -> Self {
        Self { code: error_codes::RATE_LIMITED, message: message.into(), details: None }
    }
    pub fn destructive_denied(message: impl Into<String>) -> Self {
        Self { code: error_codes::DESTRUCTIVE_DENIED, message: message.into(), details: None }
    }
    pub fn bulk_trash_denied(message: impl Into<String>) -> Self {
        Self { code: error_codes::BULK_TRASH_DENIED, message: message.into(), details: None }
    }
    pub fn feature_disabled(message: impl Into<String>) -> Self {
        Self { code: error_codes::FEATURE_DISABLED, message: message.into(), details: None }
    }
}

impl From<cog_core::Error> for ErrorResponse {
    fn from(e: cog_core::Error) -> Self {
        match &e {
            cog_core::Error::NotFound(_) => Self::not_found(e.to_string()),
            cog_core::Error::AuthRequired(_) => Self::auth_required(e.to_string()),
            cog_core::Error::PermissionDenied(_) => Self::permission_denied(e.to_string()),
            cog_core::Error::RateLimited { .. } => Self::rate_limited(e.to_string()),
            cog_core::Error::PermanentDeleteDenied => Self::destructive_denied(e.to_string()),
            cog_core::Error::BulkTrashDenied { .. } => Self::bulk_trash_denied(e.to_string()),
            cog_core::Error::FeatureDisabled(_) => Self::feature_disabled(e.to_string()),
            _ => Self::internal(e.to_string()),
        }
    }
}

// -- Supporting types --

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorSubscription {
    pub service: String,
    pub interval_secs: u64,
    pub last_check: Option<String>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexNamespaceStatus {
    pub namespace: String,
    pub document_count: usize,
    pub last_refresh: Option<String>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountStatus {
    pub email: String,
    pub scopes: Vec<String>,
    pub token_valid: bool,
    pub client_name: Option<String>,
}

mod base64_bytes {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(bytes: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use base64::Engine;
        base64::engine::general_purpose::STANDARD.encode(bytes).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        use base64::Engine;
        let encoded = String::deserialize(deserializer)?;
        base64::engine::general_purpose::STANDARD
            .decode(&encoded)
            .map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_response_roundtrip() {
        let err = ErrorResponse::not_found("file not found");
        let json = serde_json::to_string(&err).unwrap();
        let parsed: ErrorResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.code, error_codes::NOT_FOUND);
        assert!(parsed.message.contains("not found"));
    }

    #[test]
    fn result_ok_has_result_key() {
        let result = ResponseResult::ok(ResponsePayload::Pong);
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"result\""));
    }

    #[test]
    fn result_err_has_error_key() {
        let result = ResponseResult::err(ErrorResponse::internal("oops"));
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"error\""));
        assert!(json.contains("oops"));
    }
}
