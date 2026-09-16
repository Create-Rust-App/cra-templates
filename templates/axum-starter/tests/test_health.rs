//! Integration tests for the HTTP surface (ping, root, health).

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use axum_starter::{app::create_app, config::Config};
use tower::ServiceExt;

fn test_config() -> Config {
    Config {
        host: "127.0.0.1".to_string(),
        port: 0,
        api_prefix: "/api/v1".to_string(),
    }
}

async fn get(path: &str) -> (StatusCode, serde_json::Value) {
    let app = create_app(&test_config());
    let response = app
        .oneshot(Request::builder().uri(path).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    (status, json)
}

#[tokio::test]
async fn ping_returns_ok() {
    let (status, json) = get("/ping").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["status"], "ok");
}

#[tokio::test]
async fn root_links_to_versioned_health() {
    let (status, json) = get("/").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["health"], "/api/v1/healthz");
}

#[tokio::test]
async fn healthz_returns_ok() {
    let (status, json) = get("/api/v1/healthz").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["status"], "ok");
}

#[tokio::test]
async fn unknown_route_is_404() {
    let app = create_app(&test_config());
    let response = app
        .oneshot(Request::builder().uri("/nope").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
