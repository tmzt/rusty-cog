use cog_ndjson::handshake::{HandshakeRequest, HandshakeResponse};
use cog_ndjson::protocol::{CogEvent, CogRequest, CogResponse};
use cog_ndjson::request::RequestPayload;
use cog_ndjson::response::{ErrorResponse, ResponsePayload, ResponseResult};
use cog_ndjson::wire;

#[test]
fn handshake_roundtrip() {
    let req = HandshakeRequest::new();
    let json = serde_json::to_string(&req).unwrap();
    let parsed: HandshakeRequest = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.protocol, "cog/1");
}

#[test]
fn handshake_validates_protocol_version() {
    let req = HandshakeRequest { protocol: "cog/2".into() };
    assert!(req.validate().is_err());
}

#[test]
fn ping_pong_ndjson() {
    let request = CogRequest::ping(1);
    let encoded = wire::encode_request(&request).unwrap();

    // Verify it's a valid JSON line
    assert!(encoded.ends_with(b"\n"));
    let text = std::str::from_utf8(&encoded).unwrap();
    assert!(!text.trim().is_empty());

    let decoded = wire::decode_request(&encoded).unwrap();
    assert_eq!(decoded.id, 1);
    assert!(matches!(decoded.payload, RequestPayload::Ping));

    let response = CogResponse::pong(1);
    let resp_encoded = wire::encode_response(&response).unwrap();
    let resp_decoded = wire::decode_response(&resp_encoded).unwrap();
    assert_eq!(resp_decoded.id, 1);
}

#[test]
fn shutdown_request_ndjson() {
    let request = CogRequest::shutdown(42, Some("test shutdown".into()));
    let encoded = wire::encode_request(&request).unwrap();
    let decoded = wire::decode_request(&encoded).unwrap();

    assert_eq!(decoded.id, 42);
    match decoded.payload {
        RequestPayload::Shutdown { reason } => {
            assert_eq!(reason.as_deref(), Some("test shutdown"));
        }
        _ => panic!("expected Shutdown"),
    }
}

#[test]
fn error_response_ndjson() {
    let response = CogResponse::error(10, ErrorResponse::not_found("message not found"));
    let encoded = wire::encode_response(&response).unwrap();
    let decoded = wire::decode_response(&encoded).unwrap();

    assert_eq!(decoded.id, 10);
    match decoded.result {
        ResponseResult::Err(envelope) => {
            assert_eq!(envelope.error.code, 3); // NOT_FOUND
            assert!(envelope.error.message.contains("not found"));
        }
        _ => panic!("expected error response"),
    }
}

#[test]
fn event_serialization() {
    let event = CogEvent {
        event_type: "new_message".into(),
        service: "gmail".into(),
        payload: serde_json::json!({
            "thread_id": "abc123",
            "subject": "Test Subject",
        }),
        timestamp: "2026-01-15T10:30:00Z".into(),
    };

    let json = serde_json::to_string(&event).unwrap();
    let parsed: CogEvent = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.event_type, "new_message");
    assert_eq!(parsed.service, "gmail");
}

#[test]
fn auth_request_roundtrip() {
    let request = CogRequest::new(
        5,
        RequestPayload::Auth(cog_ndjson::request::AuthRequest::Login {
            email: "user@example.com".into(),
            services: vec!["gmail".into(), "drive".into()],
            readonly: false,
            manual: false,
        }),
    );

    let encoded = wire::encode_request(&request).unwrap();
    let text = std::str::from_utf8(&encoded).unwrap();
    assert!(text.contains("\"type\":\"auth.login\""));
    assert!(!text.contains("\"op\""));

    let decoded = wire::decode_request(&encoded).unwrap();
    assert_eq!(decoded.id, 5);
}

#[test]
fn monitor_subscribe_roundtrip() {
    let request = CogRequest::new(
        6,
        RequestPayload::Monitor(cog_ndjson::request::MonitorRequest::Subscribe {
            services: vec!["gmail".into(), "drive".into()],
            interval_secs: Some(30),
        }),
    );

    let encoded = wire::encode_request(&request).unwrap();
    let decoded = wire::decode_request(&encoded).unwrap();
    assert_eq!(decoded.id, 6);
}

#[test]
fn index_query_roundtrip() {
    let request = CogRequest::new(
        7,
        RequestPayload::Index(cog_ndjson::request::IndexRequest::Query {
            namespace: "gmail".into(),
            query: "from:alice subject:meeting".into(),
            max_results: Some(10),
        }),
    );

    let encoded = wire::encode_request(&request).unwrap();
    let decoded = wire::decode_request(&encoded).unwrap();
    assert_eq!(decoded.id, 7);
}

#[test]
fn max_line_size_enforced() {
    let large_reason = "x".repeat(20 * 1024 * 1024);
    let request = CogRequest::shutdown(1, Some(large_reason));

    let result = wire::encode_request(&request);
    assert!(result.is_err(), "should reject lines over 16 MiB");
}

#[test]
fn multiple_requests_sequential() {
    let requests = vec![
        CogRequest::ping(1),
        CogRequest::ping(2),
        CogRequest::shutdown(3, None),
    ];

    let mut stream = Vec::new();
    for req in &requests {
        let encoded = wire::encode_request(req).unwrap();
        stream.extend_from_slice(&encoded);
    }

    let text = std::str::from_utf8(&stream).unwrap();
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.len(), 3);

    for (i, line) in lines.iter().enumerate() {
        let req: CogRequest = serde_json::from_str(line).unwrap();
        assert_eq!(req.id, (i + 1) as u64);
    }
}

#[test]
fn wire_format_has_flat_type_string() {
    // Verify the wire format uses flat "type" strings, not nested enums
    let request = CogRequest::ping(1);
    let json = serde_json::to_string(&request).unwrap();
    // Should have "type":"ping" at top level, no "payload" wrapper
    assert!(json.contains("\"type\":\"ping\""));
    assert!(!json.contains("\"payload\""));
    assert!(!json.contains("\"service\""));
}
