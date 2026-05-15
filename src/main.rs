use clap::Parser;
use ipocrate::{Config, ensure_frontend_dist, router, served_urls};
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    init_tracing();

    let config = Config::parse();
    ensure_frontend_dist(&config.frontend_dist)?;

    let address = config.socket_addr();
    let listener = TcpListener::bind(address).await?;

    info!(
        address = %address,
        frontend_dist = %config.frontend_dist.display(),
        "starting ipocrate backend"
    );
    for url in served_urls(address) {
        info!(%url, "serving ipocrate");
    }

    axum::serve(listener, router(config.frontend_dist)).await
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(fmt::layer())
        .init();
}
