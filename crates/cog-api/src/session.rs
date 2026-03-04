use cog_ndjson::handler::RequestHandler;
use cog_ndjson::protocol::{CogRequest, CogResponse};
use cog_ndjson::request::RequestPayload;
use cog_ndjson::response::{ErrorResponse, ResponsePayload};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

/// Manages authenticated sessions for different accounts.
pub struct SessionManager {
    sessions: async_lock::Mutex<HashMap<String, Session>>,
}

struct Session {
    email: String,
    // TODO: Add service clients, token management
}

impl SessionManager {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            sessions: async_lock::Mutex::new(HashMap::new()),
        })
    }

    /// Get a request handler that dispatches to this session manager.
    pub fn handler(&self) -> SessionHandler {
        SessionHandler {
            // The handler will reference the session manager through
            // a shared reference in the actual implementation
        }
    }
}

/// Request handler backed by the session manager.
pub struct SessionHandler;

impl RequestHandler for SessionHandler {
    fn handle(
        &self,
        request: CogRequest,
    ) -> Pin<Box<dyn Future<Output = CogResponse> + Send + '_>> {
        Box::pin(async move {
            let id = request.id;

            match request.payload {
                RequestPayload::Ping => CogResponse::pong(id),
                RequestPayload::Shutdown { reason } => {
                    if let Some(reason) = &reason {
                        tracing::info!("shutdown requested: {reason}");
                    }
                    CogResponse::ok(id, ResponsePayload::ShutdownAck)
                }
                RequestPayload::Auth(_auth_req) => {
                    // TODO: Implement auth request handling
                    CogResponse::error(
                        id,
                        ErrorResponse::internal("auth handling not yet implemented"),
                    )
                }
                RequestPayload::Monitor(_monitor_req) => {
                    // TODO: Implement monitor request handling
                    CogResponse::error(
                        id,
                        ErrorResponse::internal("monitor handling not yet implemented"),
                    )
                }
                RequestPayload::Index(_index_req) => {
                    // TODO: Implement index request handling
                    CogResponse::error(
                        id,
                        ErrorResponse::internal("index handling not yet implemented"),
                    )
                }
                _ => {
                    // TODO: Dispatch to appropriate service client
                    CogResponse::error(
                        id,
                        ErrorResponse::internal("service dispatch not yet implemented"),
                    )
                }
            }
        })
    }
}
