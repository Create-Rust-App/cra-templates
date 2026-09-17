//! Tracing subscriber assembly with extension layers.
//!
//! The binary installs `fmt` plus an environment filter; extensions
//! contribute extra subscriber layers (e.g. OpenTelemetry export from the
//! `axum-otel` extension) through [`CUSTOM_TRACING`] without forking the
//! entry point.

use linkme::distributed_slice;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer, Registry};

/// A boxed subscriber layer contributed by an extension.
pub type TracingLayer = Box<dyn Layer<Registry> + Send + Sync>;

/// Extension point for subscriber layers contributed by extensions.
///
/// Each entry builds a boxed layer installed alongside the default `fmt`
/// layer. Entries are collected at link time, so extensions register
/// without forking the entry point.
#[distributed_slice]
pub static CUSTOM_TRACING: [fn() -> TracingLayer] = [..];

/// Install the global subscriber: extension layers innermost (boxed layers
/// are pinned to [`Registry`]), then the environment filter and `fmt`.
pub fn init() {
    let extra: Vec<TracingLayer> = CUSTOM_TRACING.iter().map(|build| build()).collect();
    tracing_subscriber::registry()
        .with(extra)
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axum_starter=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
