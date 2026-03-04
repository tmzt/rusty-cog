use serde::{Deserialize, Serialize};

pub const PROTOCOL_VERSION: &str = "cog/1";

/// Client handshake message (first line on connection).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandshakeRequest {
    pub protocol: String,
}

/// Server handshake response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandshakeResponse {
    pub protocol: String,
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl HandshakeRequest {
    pub fn new() -> Self {
        Self {
            protocol: PROTOCOL_VERSION.into(),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.protocol != PROTOCOL_VERSION {
            return Err(format!(
                "unsupported protocol: {} (expected {PROTOCOL_VERSION})",
                self.protocol,
            ));
        }
        Ok(())
    }
}

impl HandshakeResponse {
    pub fn ok() -> Self {
        Self {
            protocol: PROTOCOL_VERSION.into(),
            status: "ok".into(),
            error: None,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            protocol: PROTOCOL_VERSION.into(),
            status: "error".into(),
            error: Some(message.into()),
        }
    }
}

/// Server side: read handshake, validate, respond.
pub async fn server_handshake(
    reader: &mut (impl futures_lite::AsyncBufRead + Unpin),
    writer: &mut (impl futures_lite::AsyncWrite + Unpin),
) -> std::io::Result<()> {
    use futures_lite::io::{AsyncBufReadExt, AsyncWriteExt};

    let mut line = String::new();
    reader.read_line(&mut line).await?;

    let request: HandshakeRequest = serde_json::from_str(line.trim()).map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::InvalidData, format!("invalid handshake: {e}"))
    })?;

    match request.validate() {
        Ok(()) => {
            let resp = serde_json::to_string(&HandshakeResponse::ok()).unwrap();
            writer.write_all(format!("{resp}\n").as_bytes()).await?;
            writer.flush().await?;
            Ok(())
        }
        Err(msg) => {
            let resp = serde_json::to_string(&HandshakeResponse::error(&msg)).unwrap();
            writer.write_all(format!("{resp}\n").as_bytes()).await?;
            writer.flush().await?;
            Err(std::io::Error::new(std::io::ErrorKind::InvalidData, msg))
        }
    }
}

/// Client side: send handshake, read response.
pub async fn client_handshake(
    reader: &mut (impl futures_lite::AsyncBufRead + Unpin),
    writer: &mut (impl futures_lite::AsyncWrite + Unpin),
) -> std::io::Result<()> {
    use futures_lite::io::{AsyncBufReadExt, AsyncWriteExt};

    let req = serde_json::to_string(&HandshakeRequest::new()).unwrap();
    writer.write_all(format!("{req}\n").as_bytes()).await?;
    writer.flush().await?;

    let mut line = String::new();
    reader.read_line(&mut line).await?;

    let response: HandshakeResponse = serde_json::from_str(line.trim()).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("invalid handshake response: {e}"),
        )
    })?;

    if response.status != "ok" {
        return Err(std::io::Error::new(
            std::io::ErrorKind::ConnectionRefused,
            response.error.unwrap_or_else(|| "handshake rejected".into()),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_ok() {
        let req = HandshakeRequest::new();
        assert!(req.validate().is_ok());
    }

    #[test]
    fn validate_bad_protocol() {
        let req = HandshakeRequest {
            protocol: "cog/2".into(),
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn response_ok() {
        let resp = HandshakeResponse::ok();
        assert_eq!(resp.status, "ok");
        assert!(resp.error.is_none());
    }

    #[test]
    fn response_error() {
        let resp = HandshakeResponse::error("bad version");
        assert_eq!(resp.status, "error");
        assert_eq!(resp.error.as_deref(), Some("bad version"));
    }
}
