//! OpenTelemetry trace export for the Axum service.
//!
//! When `OTEL_EXPORTER_OTLP_ENDPOINT` is set, spans are exported OTLP/gRPC
//! with the configured service name; otherwise the layer is an inert
//! no-op, so local development needs no collector. Registered through the
//! template's `CUSTOM_TRACING` extension point (no file forks).

use opentelemetry::{trace::TracerProvider, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{trace::SdkTracerProvider, Resource};

use crate::telemetry::{TracingLayer, CUSTOM_TRACING};

/// Service name reported to the collector.
fn service_name() -> String {
    std::env::var("OTEL_SERVICE_NAME").unwrap_or_else(|_| "axum-starter".to_string())
}

/// Inert subscriber layer used when no collector endpoint is configured.
#[derive(Debug, Default)]
struct Noop;

impl<S> tracing_subscriber::Layer<S> for Noop where S: tracing::Subscriber {}

/// Build the export layer, or an inert no-op without an endpoint.
pub fn otel_layer() -> TracingLayer {
    let Ok(endpoint) = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT") else {
        return Box::new(Noop);
    };
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint(endpoint)
        .build()
        .expect("build OTLP exporter");
    let provider = SdkTracerProvider::builder()
        .with_resource(
            Resource::builder_empty()
                .with_attribute(KeyValue::new("service.name", service_name()))
                .build(),
        )
        .with_batch_exporter(exporter)
        .build();
    let tracer = provider.tracer("axum-starter");
    Box::new(tracing_opentelemetry::layer().with_tracer(tracer))
}

#[linkme::distributed_slice(CUSTOM_TRACING)]
static OTEL_LAYER: fn() -> TracingLayer = otel_layer;
