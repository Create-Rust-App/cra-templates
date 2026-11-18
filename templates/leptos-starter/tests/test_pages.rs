//! Integration tests for the rendered pages and fragment endpoints.

use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::get,
    Router,
};
use leptos_starter::{config::Config, site::create_app};
use tower::ServiceExt;

fn extension_router() -> Router {
    Router::new().route("/__test/extension", get(|| async { "extension route" }))
}

#[linkme::distributed_slice(leptos_starter::site::CUSTOM_ROUTES)]
static TEST_EXTENSION_ROUTES: fn() -> Router = extension_router;

mod support {
    use super::*;

    pub async fn get_page(path: &str) -> (StatusCode, String) {
        let response = create_app()
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
    let response = create_app()
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

#[tokio::test]
async fn extension_routes_are_merged_into_the_site_router() {
    let (status, body) = support::get_page("/__test/extension").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "extension route");
}

#[test]
fn default_config_matches_env_example() {
    let config = Config::default();
    assert_eq!(config.addr(), "0.0.0.0:3000");
}
