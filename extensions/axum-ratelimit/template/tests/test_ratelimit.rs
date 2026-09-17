//! Rate limit end-to-end test: bursts are capped per key, keys are
//! isolated, and anonymous callers share one bucket.
//!
//! A single test drives the whole flow because governor buckets are shared
//! process-wide: every request here runs within milliseconds, far below the
//! one-second replenishment.

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use axum_starter::{app::create_app, config::Config};
use tower::ServiceExt;

async fn get(app: axum::Router, key: Option<&str>) -> StatusCode {
    let mut builder = Request::builder().uri("/ping");
    if let Some(key) = key {
        builder = builder.header("x-api-key", key);
    }
    app.oneshot(builder.body(Body::empty()).unwrap())
        .await
        .unwrap()
        .status()
}

#[tokio::test]
async fn quotas_apply_per_key() {
    let app = create_app(&Config::default());

    // Burst of two passes, the third immediate request is rejected.
    assert_eq!(get(app.clone(), Some("key-a")).await, StatusCode::OK);
    assert_eq!(get(app.clone(), Some("key-a")).await, StatusCode::OK);
    assert_eq!(
        get(app.clone(), Some("key-a")).await,
        StatusCode::TOO_MANY_REQUESTS
    );

    // A different key has its own fresh quota.
    assert_eq!(get(app.clone(), Some("key-b")).await, StatusCode::OK);

    // Callers without a key share the anonymous bucket.
    assert_eq!(get(app.clone(), None).await, StatusCode::OK);
    assert_eq!(get(app.clone(), None).await, StatusCode::OK);
    assert_eq!(get(app, None).await, StatusCode::TOO_MANY_REQUESTS);
}
