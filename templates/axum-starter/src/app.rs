//! Router assembly and shared application state.

use axum::{extract::State, routing::get, Json, Router};
use linkme::distributed_slice;
use serde_json::{json, Value};

use crate::{config::Config, routes::health};

/// Shared state available to every handler.
#[derive(Clone, Debug)]
pub struct AppState {
    /// URL prefix for versioned API routes, e.g. `/api/v1`.
    pub api_prefix: String,
}

/// Extension point for router layers contributed by extensions.
///
/// Each entry maps `Router<AppState>` to itself (e.g. the CORS layer from
/// the `axum-cors` extension). Entries are collected at link time, so
/// extensions register without forking this file.
#[distributed_slice]
pub static CUSTOM_LAYERS: [fn(Router<AppState>) -> Router<AppState>] = [..];

/// Extension point for routers contributed by extensions.
///
/// Each entry builds a `Router<AppState>` merged under the configured API
/// prefix (e.g. the demo `/me` route from the `axum-jwt` extension).
/// Entries are collected at link time, so extensions register without
/// forking this file.
#[distributed_slice]
pub static CUSTOM_ROUTERS: [fn() -> Router<AppState>] = [..];

/// Build the application router.
///
/// `/ping` and `/` are infrastructure routes; domain routers are nested
/// under the configured API prefix.
pub fn create_app(config: &Config) -> Router {
    let state = AppState {
        api_prefix: config.api_prefix.clone(),
    };
    let mut api = Router::new().merge(health::router());
    for router in CUSTOM_ROUTERS.iter() {
        api = api.merge(router());
    }
    let router = Router::new()
        .route("/ping", get(ping))
        .route("/", get(root))
        .nest(&config.api_prefix, api);
    let router = CUSTOM_LAYERS
        .iter()
        .fold(router, |router, layer| layer(router));
    router.with_state(state)
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
