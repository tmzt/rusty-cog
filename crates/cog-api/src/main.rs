use cog_api::daemon::Daemon;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    smol::block_on(async {
        let daemon = Daemon::new()?;
        daemon.run().await
    })
}
