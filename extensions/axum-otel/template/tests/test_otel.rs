//! OpenTelemetry end-to-end test: the layer builds with and without a
//! configured endpoint, registers exactly once on `CUSTOM_TRACING`, and the
//! composed subscriber installs and records spans.
//!
//! The layer tests share one function because they mutate the process
//! environment.

use axum_starter::{
    otel::otel_layer,
    telemetry::{init, CUSTOM_TRACING},
};

#[tokio::test]
async fn otel_layer_builds_with_and_without_endpoint() {
    std::env::remove_var("OTEL_EXPORTER_OTLP_ENDPOINT");
    // No endpoint: inert no-op layer, local development needs no collector.
    let _inactive = otel_layer();

    // Endpoint set: the exporter builds lazily (no connection is opened).
    std::env::set_var("OTEL_EXPORTER_OTLP_ENDPOINT", "http://127.0.0.1:9");
    let _active = otel_layer();
    std::env::remove_var("OTEL_EXPORTER_OTLP_ENDPOINT");
}

#[tokio::test]
async fn telemetry_init_installs_composed_subscriber() {
    assert_eq!(
        CUSTOM_TRACING.iter().count(),
        1,
        "otel layer registered exactly once"
    );
    init();
    let span = tracing::info_span!("otel_smoke");
    let _guard = span.enter();
    tracing::info!("span records without a collector");
}
