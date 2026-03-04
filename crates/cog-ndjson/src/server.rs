use crate::handler::RequestHandler;
use crate::handshake::server_handshake;
use crate::protocol::CogRequest;
use crate::wire;
use std::io;
use std::path::Path;
use std::sync::Arc;

/// Unix Domain Socket server for the COG protocol (NDJSON).
pub struct UdsServer {
    handler: Arc<dyn RequestHandler>,
}

impl UdsServer {
    pub fn new(handler: impl RequestHandler + 'static) -> Self {
        Self {
            handler: Arc::new(handler),
        }
    }

    /// Listen on the given socket path, accepting connections concurrently.
    pub async fn listen(&self, socket_path: &Path) -> io::Result<()> {
        if socket_path.exists() {
            std::fs::remove_file(socket_path)?;
        }
        if let Some(parent) = socket_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let listener = smol::net::unix::UnixListener::bind(socket_path)?;
        tracing::info!("COG server listening on {}", socket_path.display());

        loop {
            let (stream, _addr) = listener.accept().await?;
            let handler = self.handler.clone();

            smol::spawn(async move {
                if let Err(e) = Self::handle_connection(stream, handler).await {
                    tracing::error!("connection error: {e}");
                }
            })
            .detach();
        }
    }

    async fn handle_connection(
        stream: smol::net::unix::UnixStream,
        handler: Arc<dyn RequestHandler>,
    ) -> io::Result<()> {
        use futures_lite::io::{AsyncWriteExt, BufReader};

        let (reader, mut writer) = futures_lite::io::split(stream);
        let mut reader = BufReader::new(reader);

        // Handshake
        server_handshake(&mut reader, &mut writer).await?;
        tracing::debug!("client connected");

        // NDJSON request/response loop
        loop {
            let line = match wire::read_line(&mut reader).await? {
                Some(l) => l,
                None => {
                    tracing::debug!("client disconnected");
                    break;
                }
            };

            let request: CogRequest = wire::decode(&line)?;
            let is_shutdown = matches!(
                request.payload,
                crate::request::RequestPayload::Shutdown { .. }
            );

            let response = handler.handle(request).await;
            let bytes = wire::encode(&response)?;
            writer.write_all(&bytes).await?;
            writer.flush().await?;

            if is_shutdown {
                tracing::info!("shutdown requested, closing connection");
                break;
            }
        }

        Ok(())
    }
}
