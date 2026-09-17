//! Gzip response compression for the Axum service.
//!
//! Clients advertising `Accept-Encoding: gzip` receive compressed responses,
//! cutting JSON payload sizes substantially. Registered through the
//! template's `CUSTOM_LAYERS` extension point (no file forks).

use axum::Router;
use tower_http::compression::CompressionLayer;

use crate::app::{AppState, CUSTOM_LAYERS};

/// Compress responses for clients that accept gzip.
/// Uses the default predicate: bodies under 32 bytes, images, gRPC, and
/// Server-Sent Events pass through uncompressed.
pub fn apply_compression(router: Router<AppState>) -> Router<AppState> {
    router.layer(CompressionLayer::new().gzip(true))
}

#[linkme::distributed_slice(CUSTOM_LAYERS)]
static COMPRESSION_LAYER: fn(Router<AppState>) -> Router<AppState> = apply_compression;
