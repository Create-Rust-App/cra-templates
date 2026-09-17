//! SQLite todo end-to-end test: full CRUD flow against an in-memory
//! database through the served routes.
//!
//! A single test drives the whole flow because the pool is a shared static:
//! the in-memory database is fresh for every test process.

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use axum_starter::{app::create_app, config::Config};
use serde_json::Value;
use tower::ServiceExt;

fn json_request(method: &str, uri: &str, body: &str) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap()
}

async fn json_body(response: axum::response::Response) -> Value {
    let body = axum::body::to_bytes(response.into_body(), 1024 * 1024)
        .await
        .expect("read body");
    serde_json::from_slice(&body).expect("parse body")
}

#[tokio::test]
async fn todo_crud_flow() {
    std::env::set_var("DATABASE_URL", "sqlite::memory:");
    let app = create_app(&Config::default());

    // Create.
    let response = app
        .clone()
        .oneshot(json_request(
            "POST",
            "/api/v1/todos",
            r#"{"title":"Write docs"}"#,
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let created = json_body(response).await;
    assert_eq!(created["title"], "Write docs");
    assert_eq!(created["done"], false);
    let id = created["id"].as_i64().expect("numeric id");

    // Blank titles are rejected.
    let response = app
        .clone()
        .oneshot(json_request("POST", "/api/v1/todos", r#"{"title":"  "}"#))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    // List.
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/todos")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let listed = json_body(response).await;
    assert_eq!(listed.as_array().expect("array").len(), 1);

    // Fetch one; unknown ids are 404.
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/v1/todos/{id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(json_body(response).await["title"], "Write docs");

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/todos/9999")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    // Delete; deleting again is 404.
    let response = app
        .clone()
        .oneshot(json_request("DELETE", &format!("/api/v1/todos/{id}"), ""))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let response = app
        .oneshot(json_request("DELETE", &format!("/api/v1/todos/{id}"), ""))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
