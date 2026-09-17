//! Binary entry point: tracing setup plus tonic serve loop with reflection
//! and graceful shutdown.

use tonic::transport::Server;
use tonic_reflection::server::Builder as ReflectionBuilder;
use tonic_starter::{
    config::Config,
    proto::{hello::greeter_server::GreeterServer, FILE_DESCRIPTOR_SET},
    service::GreeterService,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "tonic_starter=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env();
    let addr = config.addr().parse()?;
    let reflection = ReflectionBuilder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build_v1()?;

    tracing::info!("serving Greeter on {addr}");
    Server::builder()
        .add_service(GreeterServer::new(GreeterService))
        .add_service(reflection)
        .serve_with_shutdown(addr, shutdown_signal())
        .await?;
    Ok(())
}

/// Resolve when SIGINT arrives (plus SIGTERM on Unix) so in-flight RPCs drain.
async fn shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};
        let mut terminate = signal(SignalKind::terminate()).expect("install handler");
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {},
            _ = terminate.recv() => {},
        }
    }
    #[cfg(not(unix))]
    {
        tokio::signal::ctrl_c()
            .await
            .expect("install shutdown handler");
    }
    tracing::info!("shutting down");
}
