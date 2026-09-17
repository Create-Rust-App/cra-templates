//! Binary entry point: tracing setup plus Axum serve loop.
//!
//! Pages render server-side through Leptos; the counter widget stays
//! interactive via htmx fragment swaps (no WASM build step).

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use leptos::*;
use leptos_starter::{
    api::{count, health},
    app,
    config::Config,
    pages::{home::Home, not_found::NotFound},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "leptos_starter=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env();
    let app = Router::new()
        .route("/", get(home))
        .route("/api/healthz", get(health::healthz))
        .route("/api/count", post(count::increment))
        .fallback(not_found);
    let listener = tokio::net::TcpListener::bind(config.addr())
        .await
        .expect("failed to bind listener");
    tracing::info!(
        "listening on {}",
        listener.local_addr().expect("local addr")
    );
    axum::serve(listener, app).await.expect("server error");
}
