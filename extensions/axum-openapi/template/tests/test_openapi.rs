//! OpenAPI end-to-end test: the served document is valid JSON describing
//! the endpoints, and the Redoc UI renders.

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use axum_starter::{app::create_app, config::Config};
use serde_json::Value;
use tower::ServiceExt;

#[tokio::test]
async fn openapi_document_lists_endpoints() {
    let app = create_app(&Config::default());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/openapi.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), 256 * 1024)
        .await
        .expect("read body");
    let json: Value = serde_json::from_slice(&body).expect("parse document");
    assert_eq!(json["openapi"], "3.1.0");
    assert!(json["paths"]["/api/v1/openapi.json"].is_object());
}

#[tokio::test]
async fn redoc_ui_renders() {
    let app = create_app(&Config::default());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/docs")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), 256 * 1024)
        .await
        .expect("read body");
    let html = String::from_utf8(body.to_vec()).expect("html body");
    assert!(html.contains("redoc"), "Redoc UI markers");
}
