//! CORS layer end-to-end test: cross-origin responses carry ACAO headers.

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use axum_starter::{app::create_app, config::Config};
use tower::ServiceExt;

#[tokio::test]
async fn cross_origin_responses_allow_any_origin() {
    let app = create_app(&Config::default());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/ping")
                .header("origin", "http://localhost:3000")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let allow_origin = response
        .headers()
        .get("access-control-allow-origin")
        .expect("ACAO header");
    assert_eq!(allow_origin, "*");
}
