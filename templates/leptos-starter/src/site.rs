//! Axum router assembly for the Leptos site.

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use leptos::*;
use linkme::distributed_slice;

use crate::{
    api::{count, health},
    app,
    pages::{home::Home, not_found::NotFound},
};

/// Extension point for routes contributed by extensions.
///
/// Each function returns a router that is merged into the site router before
/// the fallback is installed. Entries are collected at link time, so
/// extensions can add routes without changing this module.
#[distributed_slice]
pub static CUSTOM_ROUTES: [fn() -> Router] = [..];

/// Build the site router, including routes registered by extensions.
pub fn create_app() -> Router {
    let mut router = Router::new()
        .route("/", get(home))
        .route("/api/healthz", get(health::healthz))
        .route("/api/count", post(count::increment));

    for custom_router in CUSTOM_ROUTES.iter() {
        router = router.merge(custom_router());
    }

    router.fallback(not_found)
}

async fn home() -> Html<String> {
    Html(app::shell(
        "Leptos Starter",
        &leptos::ssr::render_to_string(|| view! { <Home /> }),
    ))
}

async fn not_found() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Html(app::shell(
            "Not found",
            &leptos::ssr::render_to_string(|| view! { <NotFound /> }),
        )),
    )
}
