//! JWT authentication end-to-end test: token round-trip plus the demo
//! protected route with valid, missing, and invalid credentials.

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use axum_starter::{
    app::create_app,
    auth::{create_token, verify_token, AuthConfig},
    config::Config,
};
use serde_json::Value;
use tower::ServiceExt;

fn test_config() -> AuthConfig {
    AuthConfig {
        secret: "test-secret".to_string(),
        ttl_secs: 3600,
    }
}

#[tokio::test]
async fn tokens_round_trip() {
    let config = test_config();
    let token = create_token(&config, "user-1").expect("mint token");
    let claims = verify_token(&config, &token).expect("verify token");
    assert_eq!(claims.sub, "user-1");
}

#[tokio::test]
async fn protected_route_rejects_missing_token() {
    let app = create_app(&Config::default());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/me")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn protected_route_rejects_bad_token() {
    let app = create_app(&Config::default());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/me")
                .header("authorization", "Bearer not-a-token")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn protected_route_echoes_subject() {
    let token = create_token(&test_config(), "user-1").expect("mint token");
    // The extractor reads `JWT_SECRET` from the environment.
    std::env::set_var("JWT_SECRET", "test-secret");
    let app = create_app(&Config::default());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/me")
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), 1024)
        .await
        .expect("read body");
    let json: Value = serde_json::from_slice(&body).expect("parse body");
    assert_eq!(json["sub"], "user-1");
}
