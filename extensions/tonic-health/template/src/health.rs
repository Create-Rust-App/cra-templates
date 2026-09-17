//! Standard gRPC health probing for the tonic service.
//!
//! Serves `grpc.health.v1.Health` with the Greeter marked `SERVING`, so
//! orchestrators can probe liveness without touching business RPCs.
//! Registered through the template's `CUSTOM_SERVICES` extension point (no
//! file forks).

use std::sync::OnceLock;

use tonic::transport::server::Router;
use tonic_health::server::{health_reporter, HealthReporter};

use crate::{
    proto::hello::greeter_server::GreeterServer, server::CUSTOM_SERVICES, service::GreeterService,
};

/// Reporter handle shared with tests so they can flip statuses.
static REPORTER: OnceLock<HealthReporter> = OnceLock::new();

/// Reporter for the served health service; set once `add_health` runs.
pub fn reporter() -> &'static HealthReporter {
    REPORTER.get().expect("health service registered")
}

/// Serve the standard health probe with the Greeter marked `SERVING`.
pub fn add_health(router: Router) -> Router {
    let (reporter, health_service) = health_reporter();
    // `add_health` runs synchronously inside server assembly while a runtime
    // is already driving it, so marking serving is spawned, not awaited.
    if let Ok(handle) = tokio::runtime::Handle::try_current() {
        let mut reporter = reporter.clone();
        handle.spawn(async move {
            reporter
                .set_serving::<GreeterServer<GreeterService>>()
                .await;
        });
    }
    REPORTER.set(reporter).ok();
    router.add_service(health_service)
}

#[linkme::distributed_slice(CUSTOM_SERVICES)]
static HEALTH_SERVICE: fn(Router) -> Router = add_health;
