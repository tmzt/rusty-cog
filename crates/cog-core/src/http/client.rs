use crate::error::{Error, HttpError, Result};
use crate::http::io::SmolIo;
use crate::http::retry::RetryConfig;
use futures_rustls::TlsConnector;
use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use rustls::pki_types::ServerName;
use std::sync::Arc;
use std::time::Duration;

/// Circuit breaker state for a service endpoint.
#[derive(Debug)]
struct CircuitBreaker {
    failures: std::sync::atomic::AtomicU32,
    state: async_lock::Mutex<CircuitState>,
}

#[derive(Debug, Clone)]
enum CircuitState {
    Closed,
    Open {
        until: std::time::Instant,
    },
    HalfOpen,
}

impl CircuitBreaker {
    fn new() -> Self {
        Self {
            failures: std::sync::atomic::AtomicU32::new(0),
            state: async_lock::Mutex::new(CircuitState::Closed),
        }
    }
}

/// Async HTTP client built on `hyper` + `smol`.
///
/// Uses `rustls` for TLS. No tokio dependency.
///
/// Features:
/// - Retry with exponential backoff for 429 and 5xx
/// - Circuit breaker (5 failures -> open 30s -> half-open)
/// - Bearer token injection
#[derive(Clone)]
pub struct HttpClient {
    retry_config: RetryConfig,
    circuit_breaker: Arc<CircuitBreaker>,
    user_agent: String,
    tls_config: Arc<rustls::ClientConfig>,
}

impl HttpClient {
    /// Create a new HTTP client with default settings.
    pub fn new() -> Result<Self> {
        let tls_config = Self::make_tls_config();
        Ok(Self {
            retry_config: RetryConfig::default(),
            circuit_breaker: Arc::new(CircuitBreaker::new()),
            user_agent: format!("rusty-cog/{}", env!("CARGO_PKG_VERSION")),
            tls_config,
        })
    }

    /// Create a new HTTP client with custom retry configuration.
    pub fn with_retry(retry_config: RetryConfig) -> Result<Self> {
        let tls_config = Self::make_tls_config();
        Ok(Self {
            retry_config,
            circuit_breaker: Arc::new(CircuitBreaker::new()),
            user_agent: format!("rusty-cog/{}", env!("CARGO_PKG_VERSION")),
            tls_config,
        })
    }

    fn make_tls_config() -> Arc<rustls::ClientConfig> {
        let mut root_store = rustls::RootCertStore::empty();
        root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

        let mut config = rustls::ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        config.alpn_protocols = vec![b"http/1.1".to_vec()];
        Arc::new(config)
    }

    /// Execute a GET request with bearer token authentication.
    pub async fn get(&self, url: &str, bearer_token: &str) -> Result<Vec<u8>> {
        self.request("GET", url, bearer_token, None).await
    }

    /// Execute a POST request with bearer token authentication and JSON body.
    pub async fn post(&self, url: &str, bearer_token: &str, body: &[u8]) -> Result<Vec<u8>> {
        self.request("POST", url, bearer_token, Some(body)).await
    }

    /// Execute a PUT request with bearer token authentication and JSON body.
    pub async fn put(&self, url: &str, bearer_token: &str, body: &[u8]) -> Result<Vec<u8>> {
        self.request("PUT", url, bearer_token, Some(body)).await
    }

    /// Execute a PATCH request with bearer token authentication and JSON body.
    pub async fn patch(&self, url: &str, bearer_token: &str, body: &[u8]) -> Result<Vec<u8>> {
        self.request("PATCH", url, bearer_token, Some(body)).await
    }

    /// Execute a DELETE request with bearer token authentication.
    pub async fn delete(&self, url: &str, bearer_token: &str) -> Result<Vec<u8>> {
        self.request("DELETE", url, bearer_token, None).await
    }

    /// Execute a POST with a custom content type and bearer token.
    /// Used for Drive multipart uploads.
    pub async fn post_multipart(
        &self,
        url: &str,
        bearer_token: &str,
        body: &[u8],
        content_type: &str,
    ) -> Result<Vec<u8>> {
        self.check_circuit_breaker(url).await?;
        match self
            .do_request_inner("POST", url, Some(bearer_token), Some(body), content_type)
            .await
        {
            Ok(resp) => {
                self.record_success().await;
                Ok(resp)
            }
            Err(e) => {
                self.record_failure().await;
                Err(e)
            }
        }
    }

    /// Execute a POST with form-encoded body and no auth header.
    /// Used for OAuth2 token exchange.
    pub async fn post_form(&self, url: &str, params: &[(&str, &str)]) -> Result<Vec<u8>> {
        let body: String = url::form_urlencoded::Serializer::new(String::new())
            .extend_pairs(params)
            .finish();
        self.do_request_inner(
            "POST",
            url,
            None,
            Some(body.as_bytes()),
            "application/x-www-form-urlencoded",
        )
        .await
    }

    // -- circuit breaker ----------------------------------------------------

    async fn check_circuit_breaker(&self, service: &str) -> Result<()> {
        let state = self.circuit_breaker.state.lock().await;
        match &*state {
            CircuitState::Closed | CircuitState::HalfOpen => Ok(()),
            CircuitState::Open { until } => {
                if std::time::Instant::now() >= *until {
                    drop(state);
                    *self.circuit_breaker.state.lock().await = CircuitState::HalfOpen;
                    Ok(())
                } else {
                    Err(Error::CircuitBreakerOpen {
                        service: service.to_string(),
                    })
                }
            }
        }
    }

    async fn record_success(&self) {
        self.circuit_breaker
            .failures
            .store(0, std::sync::atomic::Ordering::Relaxed);
        *self.circuit_breaker.state.lock().await = CircuitState::Closed;
    }

    async fn record_failure(&self) {
        let failures = self
            .circuit_breaker
            .failures
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            + 1;
        if failures >= 5 {
            *self.circuit_breaker.state.lock().await = CircuitState::Open {
                until: std::time::Instant::now() + Duration::from_secs(30),
            };
        }
    }

    // -- retry loop ---------------------------------------------------------

    async fn request(
        &self,
        method: &str,
        url: &str,
        bearer_token: &str,
        body: Option<&[u8]>,
    ) -> Result<Vec<u8>> {
        self.check_circuit_breaker(url).await?;

        let mut last_err = None;
        let max_attempts = self.retry_config.max_retries + 1;

        for attempt in 0..max_attempts {
            if attempt > 0 {
                let delay = self.retry_config.delay_for_attempt(attempt);
                smol::Timer::after(delay).await;
            }

            match self
                .do_request_inner(method, url, Some(bearer_token), body, "application/json")
                .await
            {
                Ok(response) => {
                    self.record_success().await;
                    return Ok(response);
                }
                Err(e) => {
                    let should_retry = match &e {
                        Error::RateLimited { .. } => attempt < self.retry_config.max_429_retries,
                        Error::Api { status, .. } if *status >= 500 => {
                            attempt < self.retry_config.max_5xx_retries
                        }
                        Error::Http(HttpError::Connection(_)) => true,
                        _ => false,
                    };

                    if should_retry {
                        tracing::warn!(
                            attempt = attempt + 1,
                            max_attempts,
                            error = %e,
                            "retrying request"
                        );
                        last_err = Some(e);
                        continue;
                    }

                    self.record_failure().await;
                    return Err(e);
                }
            }
        }

        self.record_failure().await;
        Err(last_err.unwrap_or_else(|| Error::Other("max retries exceeded".into())))
    }

    // -- low-level request --------------------------------------------------

    async fn do_request_inner(
        &self,
        method: &str,
        url: &str,
        bearer_token: Option<&str>,
        body: Option<&[u8]>,
        content_type: &str,
    ) -> Result<Vec<u8>> {
        let uri: hyper::Uri = url
            .parse()
            .map_err(|e: hyper::http::uri::InvalidUri| {
                Error::Http(HttpError::InvalidUri(e.to_string()))
            })?;

        let scheme = uri.scheme_str().unwrap_or("https");
        let host = uri
            .host()
            .ok_or_else(|| Error::Http(HttpError::InvalidUri("missing host".into())))?;
        let port = uri
            .port_u16()
            .unwrap_or(if scheme == "https" { 443 } else { 80 });

        let addr = format!("{host}:{port}");
        let tcp = smol::net::TcpStream::connect(&addr)
            .await
            .map_err(|e| Error::Http(HttpError::Connection(e.to_string())))?;

        let path = uri
            .path_and_query()
            .map(|pq| pq.as_str())
            .unwrap_or("/");

        if scheme == "https" {
            let connector = TlsConnector::from(self.tls_config.clone());
            let server_name: ServerName<'static> = ServerName::try_from(host.to_string())
                .map_err(|e| Error::Http(HttpError::Tls(e.to_string())))?;
            let tls_stream = connector
                .connect(server_name, tcp)
                .await
                .map_err(|e| Error::Http(HttpError::Tls(e.to_string())))?;
            self.send_hyper(SmolIo(tls_stream), method, host, path, bearer_token, body, content_type)
                .await
        } else {
            self.send_hyper(SmolIo(tcp), method, host, path, bearer_token, body, content_type)
                .await
        }
    }

    async fn send_hyper<I>(
        &self,
        io: I,
        method: &str,
        host: &str,
        path: &str,
        bearer_token: Option<&str>,
        body: Option<&[u8]>,
        content_type: &str,
    ) -> Result<Vec<u8>>
    where
        I: hyper::rt::Read + hyper::rt::Write + Unpin + Send + 'static,
    {
        let (mut sender, conn) = hyper::client::conn::http1::Builder::new()
            .handshake::<_, Full<Bytes>>(io)
            .await
            .map_err(|e| Error::Http(HttpError::Hyper(e.to_string())))?;

        // Drive the connection in the background
        smol::spawn(async move {
            if let Err(e) = conn.await {
                tracing::debug!("connection task ended: {e}");
            }
        })
        .detach();

        let http_method: hyper::Method = method
            .parse()
            .map_err(|_| Error::Other(format!("invalid HTTP method: {method}")))?;

        let mut builder = hyper::Request::builder()
            .method(http_method)
            .uri(path)
            .header("Host", host)
            .header("User-Agent", &self.user_agent)
            .header("Accept", "application/json");

        if let Some(token) = bearer_token {
            builder = builder.header("Authorization", format!("Bearer {token}"));
        }

        let req_body = if let Some(data) = body {
            builder = builder.header("Content-Type", content_type);
            Full::new(Bytes::copy_from_slice(data))
        } else {
            Full::new(Bytes::new())
        };

        let req = builder
            .body(req_body)
            .map_err(|e| Error::Http(HttpError::Hyper(e.to_string())))?;

        let response = sender
            .send_request(req)
            .await
            .map_err(|e| Error::Http(HttpError::Hyper(e.to_string())))?;

        let status = response.status().as_u16();

        let retry_after = if status == 429 {
            response
                .headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse::<u64>().ok())
        } else {
            None
        };

        let body_bytes = response
            .into_body()
            .collect()
            .await
            .map_err(|e| Error::Http(HttpError::Hyper(e.to_string())))?
            .to_bytes()
            .to_vec();

        Self::check_response(status, retry_after, &body_bytes)
    }

    // -- response mapping ---------------------------------------------------

    fn check_response(
        status: u16,
        retry_after: Option<u64>,
        body: &[u8],
    ) -> Result<Vec<u8>> {
        match status {
            200..=299 => Ok(body.to_vec()),
            401 => Err(Error::AuthRequired(
                Self::extract_error_message(body)
                    .unwrap_or_else(|| "unauthorized".into()),
            )),
            403 => Err(Error::PermissionDenied(
                Self::extract_error_message(body)
                    .unwrap_or_else(|| "forbidden".into()),
            )),
            404 => Err(Error::NotFound(
                Self::extract_error_message(body)
                    .unwrap_or_else(|| "not found".into()),
            )),
            429 => Err(Error::RateLimited {
                retry_after_secs: retry_after,
            }),
            400..=499 => Err(Error::Api {
                status,
                message: Self::extract_error_message(body)
                    .unwrap_or_else(|| format!("HTTP {status}")),
                details: serde_json::from_slice(body).ok(),
            }),
            500..=599 => Err(Error::Api {
                status,
                message: Self::extract_error_message(body)
                    .unwrap_or_else(|| format!("HTTP {status}")),
                details: serde_json::from_slice(body).ok(),
            }),
            _ => Err(Error::Api {
                status,
                message: format!("unexpected status {status}"),
                details: None,
            }),
        }
    }

    fn extract_error_message(body: &[u8]) -> Option<String> {
        let json: serde_json::Value = serde_json::from_slice(body).ok()?;
        json.get("error")
            .and_then(|e| e.get("message"))
            .and_then(|m| m.as_str())
            .map(String::from)
    }
}

impl std::fmt::Debug for HttpClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HttpClient")
            .field("user_agent", &self.user_agent)
            .field("retry_config", &self.retry_config)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_client() {
        let client = HttpClient::new().unwrap();
        assert!(client.user_agent.starts_with("rusty-cog/"));
    }

    #[test]
    fn circuit_breaker_starts_closed() {
        let cb = CircuitBreaker::new();
        assert_eq!(
            cb.failures.load(std::sync::atomic::Ordering::Relaxed),
            0
        );
    }

    #[test]
    fn tls_config_created() {
        let config = HttpClient::make_tls_config();
        assert_eq!(config.alpn_protocols, vec![b"http/1.1".to_vec()]);
    }

    #[test]
    fn check_response_maps_status_codes() {
        // 200 -> Ok
        assert!(HttpClient::check_response(200, None, b"{}").is_ok());

        // 401 -> AuthRequired
        let err = HttpClient::check_response(401, None, b"{}").unwrap_err();
        assert_eq!(err.exit_code(), 4);

        // 403 -> PermissionDenied
        let err = HttpClient::check_response(403, None, b"{}").unwrap_err();
        assert_eq!(err.exit_code(), 6);

        // 404 -> NotFound
        let err = HttpClient::check_response(404, None, b"{}").unwrap_err();
        assert_eq!(err.exit_code(), 5);

        // 429 -> RateLimited
        let err = HttpClient::check_response(429, Some(30), b"{}").unwrap_err();
        assert_eq!(err.exit_code(), 7);
        match err {
            Error::RateLimited { retry_after_secs } => {
                assert_eq!(retry_after_secs, Some(30));
            }
            _ => panic!("expected RateLimited"),
        }

        // 500 -> Api
        let err = HttpClient::check_response(500, None, b"{}").unwrap_err();
        assert_eq!(err.exit_code(), 1);
    }

    #[test]
    fn extract_google_error_message() {
        let body = br#"{"error":{"message":"Quota exceeded","code":429}}"#;
        assert_eq!(
            HttpClient::extract_error_message(body),
            Some("Quota exceeded".into())
        );
    }
}
