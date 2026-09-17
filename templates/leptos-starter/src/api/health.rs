//! Health endpoint mirroring the Axum starter probe.

use axum::Json;
use serde::Serialize;

/// Health payload returned by `/api/healthz`.
#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct HealthStatus {
    /// Machine-readable status, e.g. `"ok"`.
    pub status: &'static str,
}

/// Service health endpoint.
pub async fn healthz() -> Json<HealthStatus> {
    Json(HealthStatus { status: "ok" })
}
