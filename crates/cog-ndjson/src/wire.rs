use crate::protocol::{CogRequest, CogResponse, CogEvent};
use serde::{de::DeserializeOwned, Serialize};
use std::io;

/// Maximum NDJSON line size: 16 MiB.
pub const MAX_LINE_SIZE: usize = 16 * 1024 * 1024;

/// Encode a message as a newline-terminated JSON line.
pub fn encode<T: Serialize>(msg: &T) -> io::Result<Vec<u8>> {
    let mut bytes = serde_json::to_vec(msg)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    bytes.push(b'\n');

    if bytes.len() > MAX_LINE_SIZE {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("message too large: {} bytes (max {MAX_LINE_SIZE})", bytes.len()),
        ));
    }

    Ok(bytes)
}

/// Decode a message from a JSON line (trailing newline optional).
pub fn decode<T: DeserializeOwned>(data: &[u8]) -> io::Result<T> {
    let trimmed = data.strip_suffix(b"\n").unwrap_or(data);
    serde_json::from_slice(trimmed)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

/// Convenience wrappers for specific message types.
pub fn encode_request(req: &CogRequest) -> io::Result<Vec<u8>> { encode(req) }
pub fn decode_request(data: &[u8]) -> io::Result<CogRequest> { decode(data) }
pub fn encode_response(resp: &CogResponse) -> io::Result<Vec<u8>> { encode(resp) }
pub fn decode_response(data: &[u8]) -> io::Result<CogResponse> { decode(data) }
pub fn encode_event(event: &CogEvent) -> io::Result<Vec<u8>> { encode(event) }
pub fn decode_event(data: &[u8]) -> io::Result<CogEvent> { decode(data) }

/// Read a single NDJSON line from an async reader.
pub async fn read_line(
    reader: &mut (impl futures_lite::AsyncBufRead + Unpin),
) -> io::Result<Option<Vec<u8>>> {
    use futures_lite::io::AsyncBufReadExt;

    let mut line = Vec::new();
    let n = reader.read_until(b'\n', &mut line).await?;

    if n == 0 {
        return Ok(None); // EOF
    }

    if line.len() > MAX_LINE_SIZE {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("NDJSON line too large: {} bytes", line.len()),
        ));
    }

    Ok(Some(line))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_roundtrip() {
        let req = CogRequest::ping(1);
        let encoded = encode_request(&req).unwrap();
        let decoded = decode_request(&encoded).unwrap();
        assert_eq!(decoded.id, 1);
    }

    #[test]
    fn response_roundtrip() {
        let resp = CogResponse::pong(42);
        let encoded = encode_response(&resp).unwrap();
        let decoded = decode_response(&encoded).unwrap();
        assert_eq!(decoded.id, 42);
    }

    #[test]
    fn too_short_is_error() {
        let result = decode_request(&[]);
        assert!(result.is_err());
    }
}
