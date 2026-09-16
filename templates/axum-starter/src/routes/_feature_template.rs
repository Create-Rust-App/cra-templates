//! Template for a new feature module.
//!
//! Copy this file to `<feature>.rs`, declare it in `mod.rs`, and merge
//! `router()` in `app.rs`. Delete this module if unused.

use axum::{routing::get, Json, Router};
use serde::Serialize;

/// Example payload for the new feature.
#[derive(Debug, Serialize)]
pub struct FeatureStatus {
    /// Machine-readable status.
    pub status: &'static str,
}

/// Router for the new feature; nested under the API prefix by the app.
pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new().route("/feature-template", get(feature_status))
}

/// Example handler for the new feature.
async fn feature_status() -> Json<FeatureStatus> {
    Json(FeatureStatus { status: "ok" })
}
