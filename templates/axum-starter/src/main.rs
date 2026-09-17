//! Binary entry point: tracing setup plus Axum serve loop.

use axum_starter::{app::create_app, config::Config, telemetry};

#[tokio::main]
async fn main() {
    telemetry::init();

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
