//! Request timeout middleware for the Axum service.
//!
//! Handlers that exceed [`TIMEOUT`] fail fast with `408 Request Timeout`
//! instead of holding connections open. Registered through the template's
//! `CUSTOM_LAYERS` extension point (no file forks).

use std::time::Duration;

use axum::Router;
use tower_http::timeout::TimeoutLayer;

use crate::app::{AppState, CUSTOM_LAYERS};

/// Per-request timeout applied to every route.
pub const TIMEOUT: Duration = Duration::from_secs(10);

/// Fail handlers that exceed [`TIMEOUT`] with `408 Request Timeout`.
pub fn apply_timeout(router: Router<AppState>) -> Router<AppState> {
    router.layer(TimeoutLayer::new(TIMEOUT))
}

#[linkme::distributed_slice(CUSTOM_LAYERS)]
static TIMEOUT_LAYER: fn(Router<AppState>) -> Router<AppState> = apply_timeout;
