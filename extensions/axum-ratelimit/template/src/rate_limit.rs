//! Per-key rate limiting for the Axum service.
//!
//! Quotas are keyed by the `x-api-key` header (requests without one share a
//! single anonymous bucket), so one caller cannot starve the rest. The
//! default allows a burst of [`BURST`] requests replenished at [`RATE`].
//! Registered through the template's `CUSTOM_LAYERS` extension point (no
//! file forks).

use std::sync::Arc;

use axum::Router;
use http::Request;
use tower_governor::{
    errors::GovernorError, governor::GovernorConfigBuilder, key_extractor::KeyExtractor,
    GovernorLayer,
};

use crate::app::{AppState, CUSTOM_LAYERS};

/// Sustained rate: requests per second per key.
pub const RATE: u32 = 2;
/// Burst capacity per key.
pub const BURST: u32 = 2;

/// Quota key: the `x-api-key` header, or one shared anonymous bucket.
#[derive(Debug, Clone, Copy)]
pub struct ApiKeyExtractor;

impl KeyExtractor for ApiKeyExtractor {
    type Key = String;

    fn extract<T>(&self, req: &Request<T>) -> Result<Self::Key, GovernorError> {
        Ok(req
            .headers()
            .get("x-api-key")
            .and_then(|value| value.to_str().ok())
            .unwrap_or("anonymous")
            .to_string())
    }
}

/// Reject over-quota callers with `429 Too Many Requests`.
pub fn apply_rate_limit(router: Router<AppState>) -> Router<AppState> {
    let config = GovernorConfigBuilder::default()
        .per_second(RATE as u64)
        .burst_size(BURST)
        .key_extractor(ApiKeyExtractor)
        .finish()
        .expect("valid governor config");
    router.layer(GovernorLayer {
        config: Arc::new(config),
    })
}

#[linkme::distributed_slice(CUSTOM_LAYERS)]
static RATE_LIMIT_LAYER: fn(Router<AppState>) -> Router<AppState> = apply_rate_limit;
