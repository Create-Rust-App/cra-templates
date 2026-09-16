//! Router assembly and shared application state.

use axum::{extract::State, routing::get, Json, Router};
use serde_json::{json, Value};

use crate::{config::Config, routes::health};

/// Shared state available to every handler.
#[derive(Clone, Debug)]
pub struct AppState {
    /// URL prefix for versioned API routes, e.g. `/api/v1`.
    pub api_prefix: String,
}

/// Build the application router.
///
/// `/ping` and `/` are infrastructure routes; domain routers are nested
/// under the configured API prefix.
pub fn create_app(config: &Config) -> Router {
    let state = AppState {
        api_prefix: config.api_prefix.clone(),
    };
    let api = Router::new().merge(health::router());
    Router::new()
        .route("/ping", get(ping))
        .route("/", get(root))
        .nest(&config.api_prefix, api)
        .with_state(state)
}

/// Minimal health probe for load balancers (not versioned).
async fn ping() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

/// Root endpoint with navigation hints.
async fn root(State(state): State<AppState>) -> Json<Value> {
    Json(json!({
        "message": "Welcome to axum-starter",
        "health": format!("{}/healthz", state.api_prefix),
    }))
}
