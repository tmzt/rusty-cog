use crate::services::*;
use serde::de::{self, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

/// Generates the `RequestPayload` enum plus the three match-arm tables
/// (service_name, serialize_inner, deserialize_service) from a single declaration.
macro_rules! service_payload {
    (
        $(
            $(#[$meta:meta])*
            $variant:ident($inner:ty) => $wire:literal
        ),* $(,)?
    ) => {
        /// Top-level request payload.
        ///
        /// On the wire each variant serializes as `"type": "service.operation"`
        /// with the operation's parameters flattened into the same JSON object.
        #[derive(Debug, Clone)]
        pub enum RequestPayload {
            $(
                $(#[$meta])*
                $variant($inner),
            )*
            Ping,
            Shutdown { reason: Option<String> },
        }

        impl RequestPayload {
            fn service_name(&self) -> &'static str {
                match self {
                    $(
                        $(#[$meta])*
                        Self::$variant(_) => $wire,
                    )*
                    Self::Ping => "ping",
                    Self::Shutdown { .. } => "shutdown",
                }
            }

            /// Serialize the inner (op-tagged) enum to a `serde_json::Value`.
            /// Returns `None` for standalone variants (Ping, Shutdown).
            fn serialize_inner(&self) -> Option<serde_json::Result<serde_json::Value>> {
                match self {
                    $(
                        $(#[$meta])*
                        Self::$variant(r) => Some(serde_json::to_value(r)),
                    )*
                    Self::Ping | Self::Shutdown { .. } => None,
                }
            }

            /// Dispatch a service name + JSON object to the correct variant.
            fn deserialize_service(
                service: &str,
                value: serde_json::Value,
            ) -> Result<Self, String> {
                match service {
                    $(
                        $(#[$meta])*
                        $wire => serde_json::from_value(value)
                            .map(Self::$variant)
                            .map_err(|e| format!("invalid {}.* request: {e}", $wire)),
                    )*
                    other => Err(format!("unknown service: {other}")),
                }
            }
        }
    };
}

service_payload! {
    Gmail(GmailRequest)             => "gmail",
    Calendar(CalendarRequest)       => "calendar",
    Drive(DriveRequest)             => "drive",
    Docs(DocsRequest)               => "docs",
    Sheets(SheetsRequest)           => "sheets",
    Slides(SlidesRequest)           => "slides",
    Forms(FormsRequest)             => "forms",
    Contacts(ContactsRequest)       => "contacts",
    Tasks(TasksRequest)             => "tasks",
    People(PeopleRequest)           => "people",
    Chat(ChatRequest)               => "chat",
    Classroom(ClassroomRequest)     => "classroom",
    Groups(GroupsRequest)           => "groups",
    Keep(KeepRequest)               => "keep",
    AppScript(AppScriptRequest)     => "appscript",
    #[cfg(feature = "gemini-web")]
    Gemini(GeminiRequest)           => "gemini",
    #[cfg(feature = "notebooklm")]
    NotebookLm(NotebookLmRequest)   => "notebooklm",
    Auth(AuthRequest)               => "auth",
    Monitor(MonitorRequest)         => "monitor",
    Index(IndexRequest)             => "index",
}

// -- Shared serialization helpers (not per-variant, so outside the macro) --

impl RequestPayload {
    /// Build `("service.op", {params})` for wire serialization.
    fn to_type_and_params(&self) -> (String, serde_json::Map<String, serde_json::Value>) {
        let service = self.service_name();

        // Standalone types (no inner enum, no dot)
        match self {
            Self::Ping => return ("ping".into(), serde_json::Map::new()),
            Self::Shutdown { reason } => {
                let mut m = serde_json::Map::new();
                if let Some(r) = reason {
                    m.insert("reason".into(), serde_json::Value::String(r.clone()));
                }
                return ("shutdown".into(), m);
            }
            _ => {}
        }

        // Service types: inner enum → JSON object → extract "op" → build type string
        let value = self
            .serialize_inner()
            .expect("non-standalone variant")
            .expect("request serialization should not fail");

        let mut map = match value {
            serde_json::Value::Object(m) => m,
            _ => serde_json::Map::new(),
        };

        let op = map
            .remove("op")
            .and_then(|v| match v {
                serde_json::Value::String(s) => Some(s),
                _ => None,
            })
            .unwrap_or_default();

        (format!("{service}.{op}"), map)
    }

    /// Parse a type string + params map back into a typed payload.
    fn from_type_and_params(
        type_str: &str,
        params: serde_json::Map<String, serde_json::Value>,
    ) -> Result<Self, String> {
        // Standalone types (no dot)
        match type_str {
            "ping" => return Ok(Self::Ping),
            "shutdown" => {
                let reason = params
                    .get("reason")
                    .and_then(|v| v.as_str())
                    .map(String::from);
                return Ok(Self::Shutdown { reason });
            }
            _ => {}
        }

        // Service types: split on first dot, re-inject "op"
        let (service, op) = type_str
            .split_once('.')
            .ok_or_else(|| format!("invalid request type: {type_str}"))?;

        let mut map = params;
        map.insert("op".into(), serde_json::Value::String(op.into()));

        Self::deserialize_service(service, serde_json::Value::Object(map))
    }
}

// -- Serde impls --

impl Serialize for RequestPayload {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let (type_str, params) = self.to_type_and_params();
        let mut map = serializer.serialize_map(Some(1 + params.len()))?;
        map.serialize_entry("type", &type_str)?;
        for (k, v) in &params {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for RequestPayload {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct PayloadVisitor;

        impl<'de> Visitor<'de> for PayloadVisitor {
            type Value = RequestPayload;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a JSON object with a \"type\" field")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut type_str: Option<String> = None;
                let mut params = serde_json::Map::new();

                while let Some(key) = map.next_key::<String>()? {
                    if key == "type" {
                        type_str = Some(map.next_value()?);
                    } else {
                        params.insert(key, map.next_value()?);
                    }
                }

                let type_str = type_str.ok_or_else(|| de::Error::missing_field("type"))?;
                RequestPayload::from_type_and_params(&type_str, params)
                    .map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_map(PayloadVisitor)
    }
}

// -- Protocol-level request enums (Auth, Monitor, Index) --

/// Authentication requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum AuthRequest {
    Login {
        email: String,
        #[serde(default)]
        services: Vec<String>,
        #[serde(default)]
        readonly: bool,
        #[serde(default)]
        manual: bool,
    },
    Status,
    List {
        #[serde(default)]
        check: bool,
    },
    Remove {
        email: String,
    },
    Credentials {
        path: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        client_name: Option<String>,
    },
    CredentialsList,
    ServiceAccountSet {
        email: String,
        key_path: String,
    },
    ServiceAccountStatus {
        email: String,
    },
    ServiceAccountUnset {
        email: String,
    },
    KeyringGet,
    KeyringSet {
        backend: String,
    },
    AliasSet {
        alias: String,
        email: String,
    },
    AliasList,
    AliasUnset {
        alias: String,
    },
}

/// Monitor requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum MonitorRequest {
    Subscribe {
        services: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        interval_secs: Option<u64>,
    },
    Unsubscribe {
        #[serde(default)]
        services: Vec<String>,
    },
    Status,
}

/// Index requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum IndexRequest {
    Query {
        namespace: String,
        query: String,
        #[serde(default)]
        max_results: Option<usize>,
    },
    Refresh {
        #[serde(default)]
        namespaces: Vec<String>,
    },
    Status,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_roundtrip() {
        let payload = RequestPayload::Ping;
        let json = serde_json::to_string(&payload).unwrap();
        assert!(json.contains("\"type\":\"ping\""));
        let parsed: RequestPayload = serde_json::from_str(&json).unwrap();
        assert!(matches!(parsed, RequestPayload::Ping));
    }

    #[test]
    fn shutdown_roundtrip() {
        let payload = RequestPayload::Shutdown {
            reason: Some("user requested".into()),
        };
        let json = serde_json::to_string(&payload).unwrap();
        assert!(json.contains("\"type\":\"shutdown\""));
        assert!(json.contains("user requested"));
        let parsed: RequestPayload = serde_json::from_str(&json).unwrap();
        match parsed {
            RequestPayload::Shutdown { reason } => {
                assert_eq!(reason.as_deref(), Some("user requested"));
            }
            _ => panic!("expected shutdown"),
        }
    }

    #[test]
    fn auth_login_type_string() {
        let payload = RequestPayload::Auth(AuthRequest::Login {
            email: "test@example.com".into(),
            services: vec!["gmail".into()],
            readonly: false,
            manual: false,
        });
        let json = serde_json::to_string(&payload).unwrap();
        assert!(json.contains("\"type\":\"auth.login\""));
        assert!(json.contains("test@example.com"));
        assert!(!json.contains("\"op\""));

        let parsed: RequestPayload = serde_json::from_str(&json).unwrap();
        assert!(matches!(
            parsed,
            RequestPayload::Auth(AuthRequest::Login { .. })
        ));
    }

    #[test]
    fn gmail_search_type_string() {
        let payload = RequestPayload::Gmail(GmailRequest::Search {
            query: "from:alice".into(),
            max: Some(10),
        });
        let json = serde_json::to_string(&payload).unwrap();
        assert!(json.contains("\"type\":\"gmail.search\""));
        assert!(json.contains("from:alice"));
        assert!(!json.contains("\"op\""));

        let parsed: RequestPayload = serde_json::from_str(&json).unwrap();
        match parsed {
            RequestPayload::Gmail(GmailRequest::Search { query, max }) => {
                assert_eq!(query, "from:alice");
                assert_eq!(max, Some(10));
            }
            _ => panic!("expected gmail search"),
        }
    }

    #[test]
    fn monitor_subscribe_type_string() {
        let payload = RequestPayload::Monitor(MonitorRequest::Subscribe {
            services: vec!["gmail".into(), "drive".into()],
            interval_secs: Some(30),
        });
        let json = serde_json::to_string(&payload).unwrap();
        assert!(json.contains("\"type\":\"monitor.subscribe\""));
    }
}
