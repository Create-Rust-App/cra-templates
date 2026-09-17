//! Request ID middleware for the Axum service.
//!
//! Every response carries a unique `x-request-id` header, so logs and error
//! reports can be correlated across services. Registered through the
//! template's `CUSTOM_LAYERS` extension point (no file forks).

use axum::Router;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};

use crate::app::{AppState, CUSTOM_LAYERS};

/// Stamp a UUID request id and propagate it back on the response.
///
/// Ordering matters: `SetRequestIdLayer` is outermost so it inserts the
/// header before `PropagateRequestIdLayer` reads it (`Propagate` captures
/// the id from the request headers when the request passes through).
pub fn apply_request_id(router: Router<AppState>) -> Router<AppState> {
    router
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
}

#[linkme::distributed_slice(CUSTOM_LAYERS)]
static REQUEST_ID_LAYER: fn(Router<AppState>) -> Router<AppState> = apply_request_id;
