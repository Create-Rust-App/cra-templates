//! Baseline security headers for the Axum service.
//!
//! Every response carries a conservative hardening set (content-type
//! sniffing off, clickjacking denied, narrow referrer policy, modern
//! cross-origin isolation). Registered through the template's
//! `CUSTOM_LAYERS` extension point (no file forks).

use axum::Router;
use http::header::{HeaderName, HeaderValue};
use tower_http::set_header::SetResponseHeaderLayer;

use crate::app::{AppState, CUSTOM_LAYERS};

/// Header/value pairs applied to every response.
pub const SECURITY_HEADERS: [(&str, &str); 4] = [
    ("x-content-type-options", "nosniff"),
    ("x-frame-options", "DENY"),
    ("referrer-policy", "strict-origin-when-cross-origin"),
    ("cross-origin-opener-policy", "same-origin"),
];

/// Stamp [`SECURITY_HEADERS`] on every response.
pub fn apply_security_headers(router: Router<AppState>) -> Router<AppState> {
    SECURITY_HEADERS
        .iter()
        .fold(router, |router, (name, value)| {
            router.layer(SetResponseHeaderLayer::overriding(
                HeaderName::from_static(name),
                HeaderValue::from_static(value),
            ))
        })
}

#[linkme::distributed_slice(CUSTOM_LAYERS)]
static SECURITY_HEADERS_LAYER: fn(Router<AppState>) -> Router<AppState> = apply_security_headers;
