use crate::session::SessionManager;
use cog_core::config;
use cog_ndjson::server::UdsServer;

/// The cog-api daemon process.
///
/// Long-lived process that listens on a UDS socket, manages sessions
/// (one per authenticated account), and dispatches protocol requests.
pub struct Daemon {
    session_manager: SessionManager,
}

impl Daemon {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            session_manager: SessionManager::new()?,
        })
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let socket_path = config::socket_path()?;

        tracing::info!("starting cog daemon on {}", socket_path.display());

        let server = UdsServer::new(self.session_manager.handler());
        server.listen(&socket_path).await?;

        Ok(())
    }
}
