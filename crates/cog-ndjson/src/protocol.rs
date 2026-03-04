use crate::request::RequestPayload;
use crate::response::ResponseResult;
use serde::de::{self, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

/// A request message in the COG protocol.
///
/// Wire format: `{"id": N, "type": "service.op", ...params}\n`
#[derive(Debug, Clone)]
pub struct CogRequest {
    pub id: u64,
    pub payload: RequestPayload,
}

/// A response message in the COG protocol.
///
/// Wire format:
///   success: `{"id": N, "result": ...}\n`
///   error:   `{"id": N, "error": {"code": N, "message": "..."}}\n`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CogResponse {
    pub id: u64,
    #[serde(flatten)]
    pub result: ResponseResult,
}

/// A server-initiated event (monitor mode).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CogEvent {
    pub event_type: String,
    pub service: String,
    pub payload: serde_json::Value,
    pub timestamp: String,
}

// -- CogRequest: flatten "id" + RequestPayload into one JSON object --

impl Serialize for CogRequest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // RequestPayload serializes as {"type": "...", ...params}
        // We want {"id": N, "type": "...", ...params}
        let payload_value = serde_json::to_value(&self.payload)
            .map_err(serde::ser::Error::custom)?;
        let payload_map = payload_value
            .as_object()
            .ok_or_else(|| serde::ser::Error::custom("payload must serialize to object"))?;

        let mut map = serializer.serialize_map(Some(1 + payload_map.len()))?;
        map.serialize_entry("id", &self.id)?;
        for (k, v) in payload_map {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for CogRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ReqVisitor;

        impl<'de> Visitor<'de> for ReqVisitor {
            type Value = CogRequest;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a JSON object with \"id\" and \"type\" fields")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut id: Option<u64> = None;
                let mut rest = serde_json::Map::new();

                while let Some(key) = map.next_key::<String>()? {
                    if key == "id" {
                        id = Some(map.next_value()?);
                    } else {
                        rest.insert(key, map.next_value()?);
                    }
                }

                let id = id.ok_or_else(|| de::Error::missing_field("id"))?;
                let payload: RequestPayload =
                    serde_json::from_value(serde_json::Value::Object(rest))
                        .map_err(de::Error::custom)?;

                Ok(CogRequest { id, payload })
            }
        }

        deserializer.deserialize_map(ReqVisitor)
    }
}

// -- Constructors --

impl CogRequest {
    pub fn new(id: u64, payload: RequestPayload) -> Self {
        Self { id, payload }
    }

    pub fn ping(id: u64) -> Self {
        Self { id, payload: RequestPayload::Ping }
    }

    pub fn shutdown(id: u64, reason: Option<String>) -> Self {
        Self { id, payload: RequestPayload::Shutdown { reason } }
    }
}

impl CogResponse {
    pub fn ok(id: u64, payload: crate::response::ResponsePayload) -> Self {
        Self { id, result: ResponseResult::ok(payload) }
    }

    pub fn error(id: u64, error: crate::response::ErrorResponse) -> Self {
        Self { id, result: ResponseResult::err(error) }
    }

    pub fn pong(id: u64) -> Self {
        Self::ok(id, crate::response::ResponsePayload::Pong)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_ping_wire_format() {
        let req = CogRequest::ping(42);
        let json = serde_json::to_string(&req).unwrap();
        // Should be flat: {"id":42,"type":"ping"}
        assert!(json.contains("\"id\":42"));
        assert!(json.contains("\"type\":\"ping\""));
        // No nested "payload" key
        assert!(!json.contains("\"payload\""));

        let parsed: CogRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.id, 42);
        assert!(matches!(parsed.payload, RequestPayload::Ping));
    }

    #[test]
    fn request_shutdown_wire_format() {
        let req = CogRequest::shutdown(7, Some("done".into()));
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"id\":7"));
        assert!(json.contains("\"type\":\"shutdown\""));
        assert!(json.contains("\"reason\":\"done\""));
    }

    #[test]
    fn response_pong_roundtrip() {
        let resp = CogResponse::pong(42);
        let json = serde_json::to_string(&resp).unwrap();
        let parsed: CogResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.id, 42);
    }

    #[test]
    fn event_serialization() {
        let event = CogEvent {
            event_type: "new_message".into(),
            service: "gmail".into(),
            payload: serde_json::json!({"thread_id": "abc123"}),
            timestamp: "2026-01-01T00:00:00Z".into(),
        };
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("new_message"));
    }
}
