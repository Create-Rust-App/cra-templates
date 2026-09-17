//! Server assembly and graceful shutdown.
//!
//! The Greeter plus reflection are always served; extensions contribute
//! additional services through [`CUSTOM_SERVICES`] without forking this
//! file (e.g. the gRPC health probe from the `tonic-health` extension).

use std::net::SocketAddr;

use linkme::distributed_slice;
use tonic::transport::{server::Router, Server};
use tonic_reflection::server::Builder as ReflectionBuilder;

use crate::{
    config::Config,
    proto::{hello::greeter_server::GreeterServer, FILE_DESCRIPTOR_SET},
    service::GreeterService,
};

/// Extension point for services contributed by extensions.
///
/// Each entry maps the assembled [`Router`] to itself. Entries are
/// collected at link time, so extensions register without forking this
/// file.
#[distributed_slice]
pub static CUSTOM_SERVICES: [fn(Router) -> Router] = [..];

/// Serve the configured address until the shutdown signal resolves.
pub async fn run(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = config.addr().parse()?;
    let reflection = ReflectionBuilder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build_v1()?;

    tracing::info!("serving Greeter on {addr}");
    let router = CUSTOM_SERVICES.iter().fold(
        Server::builder()
            .add_service(GreeterServer::new(GreeterService))
            .add_service(reflection),
        |router, add| add(router),
    );
    router.serve_with_shutdown(addr, shutdown_signal()).await?;
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
