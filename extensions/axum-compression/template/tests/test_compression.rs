//! Compression end-to-end test: gzip is served when accepted and skipped
//! otherwise.

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use axum_starter::{app::create_app, config::Config};
use tower::ServiceExt;

async fn get_with_encoding(
    app: axum::Router,
    uri: &str,
    accept_encoding: Option<&str>,
) -> axum::response::Response {
    let mut builder = Request::builder().uri(uri);
    if let Some(encoding) = accept_encoding {
        builder = builder.header("accept-encoding", encoding);
    }
    app.oneshot(builder.body(Body::empty()).unwrap())
        .await
        .unwrap()
}

#[tokio::test]
async fn gzip_served_when_accepted() {
    // `/` answers a body above the 32-byte minimum of the default predicate;
    // tiny bodies like `/ping` are intentionally left uncompressed.
    let app = create_app(&Config::default());
    let response = get_with_encoding(app, "/", Some("gzip")).await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response
            .headers()
            .get("content-encoding")
            .map(|value| value.to_str().expect("value")),
        Some("gzip"),
    );
}

#[tokio::test]
async fn identity_served_without_accept_encoding() {
    let app = create_app(&Config::default());
    let response = get_with_encoding(app, "/ping", None).await;
    assert_eq!(response.status(), StatusCode::OK);
    assert!(response.headers().get("content-encoding").is_none());
}
