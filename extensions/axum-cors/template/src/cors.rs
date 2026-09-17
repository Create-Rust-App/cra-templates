//! Permissive CORS layer registered on [`crate::app::CUSTOM_LAYERS`].
//!
//! The default allows any origin, method, and header so local frontends work
//! out of the box. Restrict it for production (see README).

use axum::Router;
use tower_http::cors::{Any, CorsLayer};

use crate::app::{AppState, CUSTOM_LAYERS};

/// Apply a permissive CORS layer to the assembled router.
pub fn apply_cors(router: Router<AppState>) -> Router<AppState> {
    router.layer(
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any),
    )
}

#[linkme::distributed_slice(CUSTOM_LAYERS)]
static CORS_LAYER: fn(Router<AppState>) -> Router<AppState> = apply_cors;
