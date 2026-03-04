use thiserror::Error;

/// Top-level error type for cog-core operations.
#[derive(Debug, Error)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Http(#[from] HttpError),

    #[error("authentication required: {0}")]
    AuthRequired(String),

    #[error("OAuth2 error: {0}")]
    OAuth2(String),

    #[error("API error {status}: {message}")]
    Api {
        status: u16,
        message: String,
        details: Option<serde_json::Value>,
    },

    #[error("not found: {0}")]
    NotFound(String),

    #[error("permission denied: {0}")]
    PermissionDenied(String),

    #[error("rate limited: retry after {retry_after_secs:?}s")]
    RateLimited { retry_after_secs: Option<u64> },

    #[error("retryable error: {0}")]
    Retryable(String),

    #[error("configuration error: {0}")]
    Config(String),

    #[error("permanent delete denied: feature 'destructive-permanent' is not enabled")]
    PermanentDeleteDenied,

    #[error("bulk trash denied: operating on {count} items (>50) requires feature 'destructive-bulk-trash'")]
    BulkTrashDenied { count: usize },

    #[error("feature not enabled: {0}")]
    FeatureDisabled(String),

    #[error("serialization error: {0}")]
    Serialization(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("keyring error: {0}")]
    Keyring(String),

    #[error("circuit breaker open for {service}")]
    CircuitBreakerOpen { service: String },

    #[error("cancelled")]
    Cancelled,

    #[error("{0}")]
    Other(String),
}

/// HTTP-specific errors.
#[derive(Debug, Error)]
pub enum HttpError {
    #[error("connection error: {0}")]
    Connection(String),

    #[error("timeout after {0:?}")]
    Timeout(std::time::Duration),

    #[error("TLS error: {0}")]
    Tls(String),

    #[error("hyper error: {0}")]
    Hyper(String),

    #[error("invalid URI: {0}")]
    InvalidUri(String),
}

impl Error {
    /// Map this error to a CLI exit code.
    pub fn exit_code(&self) -> i32 {
        match self {
            Error::Api { .. } | Error::Http(_) | Error::Other(_) => 1,
            Error::AuthRequired(_) | Error::OAuth2(_) => 4,
            Error::NotFound(_) => 5,
            Error::PermissionDenied(_) => 6,
            Error::RateLimited { .. } => 7,
            Error::Retryable(_) | Error::CircuitBreakerOpen { .. } => 8,
            Error::Config(_) => 10,
            Error::Cancelled => 130,
            Error::PermanentDeleteDenied | Error::BulkTrashDenied { .. } => 1,
            Error::FeatureDisabled(_) => 1,
            _ => 1,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
