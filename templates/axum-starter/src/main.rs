//! Binary entry point: tracing setup plus Axum serve loop.

use axum_starter::{app::create_app, config::Config};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axum_starter=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env();
    let app = create_app(&config);
    let listener = tokio::net::TcpListener::bind(config.addr())
        .await
        .expect("failed to bind listener");
    tracing::info!(
        "listening on {}",
        listener.local_addr().expect("local addr")
    );
    axum::serve(listener, app).await.expect("server error");
}
