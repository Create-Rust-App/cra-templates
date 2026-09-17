//! Request ID end-to-end test: every response carries a unique id.

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use axum_starter::{app::create_app, config::Config};
use tower::ServiceExt;

async fn request_id_for(app: axum::Router, uri: &str) -> String {
    let response = app
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    response
        .headers()
        .get("x-request-id")
        .expect("request id header")
        .to_str()
        .expect("header value")
        .to_string()
}

#[tokio::test]
async fn responses_carry_unique_request_ids() {
    let app = create_app(&Config::default());
    let first = request_id_for(app.clone(), "/ping").await;
    let second = request_id_for(app, "/ping").await;
    assert!(!first.is_empty());
    assert!(!second.is_empty());
    assert_ne!(first, second, "ids are unique per request");
}
