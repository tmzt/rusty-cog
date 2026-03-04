use crate::protocol::{CogRequest, CogResponse};
use crate::request::RequestPayload;
use crate::response::{ErrorResponse, ResponsePayload};
use std::future::Future;
use std::pin::Pin;

/// Trait for handling COG protocol requests.
///
/// Implemented by the `cog-api` crate to dispatch requests to service clients.
/// Uses boxed futures to be dyn-compatible.
pub trait RequestHandler: Send + Sync {
    /// Handle a request and return a response.
    fn handle(
        &self,
        request: CogRequest,
    ) -> Pin<Box<dyn Future<Output = CogResponse> + Send + '_>>;
}

/// Default handler that responds to Ping and rejects everything else.
pub struct DefaultHandler;

impl RequestHandler for DefaultHandler {
    fn handle(
        &self,
        request: CogRequest,
    ) -> Pin<Box<dyn Future<Output = CogResponse> + Send + '_>> {
        Box::pin(async move {
            let id = request.id;

            match request.payload {
                RequestPayload::Ping => CogResponse::pong(id),
                RequestPayload::Shutdown { .. } => {
                    CogResponse::ok(id, ResponsePayload::ShutdownAck)
                }
                _ => CogResponse::error(
                    id,
                    ErrorResponse::internal("no handler registered for this service"),
                ),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_handler_ping() {
        smol::block_on(async {
            let handler = DefaultHandler;
            let req = CogRequest::ping(1);
            let resp = handler.handle(req).await;
            assert_eq!(resp.id, 1);
        });
    }

    #[test]
    fn default_handler_shutdown() {
        smol::block_on(async {
            let handler = DefaultHandler;
            let req = CogRequest::shutdown(2, None);
            let resp = handler.handle(req).await;
            assert_eq!(resp.id, 2);
        });
    }
}
