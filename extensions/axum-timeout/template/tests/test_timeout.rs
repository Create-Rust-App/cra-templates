//! Timeout end-to-end test: slow handlers fail fast, fast ones pass.
//!
//! The test builds its own routers with short timeouts so the suite stays
//! fast; the shipped [`axum_starter::timeout::TIMEOUT`] default (10s) only
//! guards production handlers.

use std::time::Duration;

use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::get,
    Router,
};
use tower::ServiceExt;
use tower_http::timeout::TimeoutLayer;

async fn slow() -> &'static str {
    tokio::time::sleep(Duration::from_secs(5)).await;
    "too late"
}

#[tokio::test]
async fn slow_handler_times_out() {
    let app = Router::new()
        .route("/slow", get(slow))
        .layer(TimeoutLayer::new(Duration::from_millis(50)));
    let response = app
        .oneshot(Request::builder().uri("/slow").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::REQUEST_TIMEOUT);
}

#[tokio::test]
async fn fast_handler_passes_through_timeout() {
    use axum_starter::{app::create_app, config::Config};
    // The composed service (with the 10s extension timeout) answers fast
    // routes normally.
    let app = create_app(&Config::default());
    let response = app
        .oneshot(Request::builder().uri("/ping").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
