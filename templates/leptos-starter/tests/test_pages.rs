//! Integration tests for the rendered pages and fragment endpoints.

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use leptos_starter::{app, config::Config, pages::home::Home, pages::not_found::NotFound};
use tower::ServiceExt;

mod support {
    use super::*;
    use axum::{
        response::{Html, IntoResponse},
        routing::{get, post},
        Router,
    };
    use leptos::*;
    use leptos_starter::api::{count, health};

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

    pub fn app() -> Router {
        Router::new()
            .route("/", get(home))
            .route("/api/healthz", get(health::healthz))
            .route("/api/count", post(count::increment))
            .fallback(not_found)
    }

    pub async fn get_page(path: &str) -> (StatusCode, String) {
        let response = app()
            .oneshot(Request::builder().uri(path).body(Body::empty()).unwrap())
            .await
            .unwrap();
        let status = response.status();
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        (status, String::from_utf8(body.to_vec()).unwrap())
    }
}

#[tokio::test]
async fn home_renders_shell_and_counter() {
    let (status, body) = support::get_page("/").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("<!DOCTYPE html>"), "full document");
    assert!(body.contains("htmx.min.js"), "htmx wired");
    assert!(body.contains("Leptos Starter"), "title and heading");
    assert!(body.contains("hx-post=\"/api/count\""), "counter posts");
    assert!(body.contains(">0<"), "initial count");
}

#[tokio::test]
async fn healthz_returns_ok_json() {
    let (status, body) = support::get_page("/api/healthz").await;
    assert_eq!(status, StatusCode::OK);
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(json["status"], "ok");
}

#[tokio::test]
async fn count_returns_incremented_fragment() {
    let response = support::app()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/count")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"count": 41}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let html = String::from_utf8(body.to_vec()).unwrap();
    assert!(html.contains(">42<"), "incremented, got: {html}");
    assert!(html.contains("hx-post"), "widget rewired");
}

#[tokio::test]
async fn unknown_route_renders_404_page() {
    let (status, body) = support::get_page("/nope").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert!(body.contains("Not found"), "404 page, got: {body}");
}

#[test]
fn default_config_matches_env_example() {
    let config = Config::default();
    assert_eq!(config.addr(), "0.0.0.0:3000");
}
