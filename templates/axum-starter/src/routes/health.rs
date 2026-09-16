//! Health feature HTTP routes.

use axum::{routing::get, Json, Router};
use serde::Serialize;

/// Health payload returned by `/healthz`.
#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct HealthStatus {
    /// Machine-readable status, e.g. `"ok"`.
    pub status: &'static str,
}

/// Router for the health feature; nested under the API prefix by the app.
pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new().route("/healthz", get(health_check))
}

/// Service health endpoint.
async fn health_check() -> Json<HealthStatus> {
    Json(HealthStatus { status: "ok" })
}
