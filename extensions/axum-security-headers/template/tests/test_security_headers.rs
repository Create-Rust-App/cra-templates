//! Security headers end-to-end test: every response carries the full
//! hardening set with the documented values.

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use axum_starter::{app::create_app, config::Config, security_headers::SECURITY_HEADERS};
use tower::ServiceExt;

#[tokio::test]
async fn responses_carry_security_headers() {
    let app = create_app(&Config::default());
    for uri in ["/ping", "/", "/api/v1/healthz"] {
        let response = app
            .clone()
            .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK, "status for {uri}");
        for (name, value) in SECURITY_HEADERS {
            assert_eq!(
                response
                    .headers()
                    .get(name)
                    .unwrap_or_else(|| panic!("header {name} on {uri}"))
                    .to_str()
                    .expect("header value"),
                value,
                "value of {name} on {uri}"
            );
        }
    }
}
