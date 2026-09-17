//! Demo protected route exercising the [`crate::auth`] extractor.
//!
//! Registered on [`crate::app::CUSTOM_ROUTERS`] so it merges under the API
//! prefix without forking the template router.

use axum::{routing::get, Json, Router};
use serde::Serialize;

use crate::{
    app::{AppState, CUSTOM_ROUTERS},
    auth::AuthUser,
};

/// Payload returned by the demo `/me` endpoint.
#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct MeResponse {
    /// Subject from the verified bearer token.
    pub sub: String,
}

/// Router for the auth demo feature; nested under the API prefix by the app.
pub fn router() -> Router<AppState> {
    Router::new().route("/me", get(me))
}

/// Echo the authenticated subject.
async fn me(user: AuthUser) -> Json<MeResponse> {
    Json(MeResponse { sub: user.sub })
}

#[linkme::distributed_slice(CUSTOM_ROUTERS)]
static AUTH_ROUTER: fn() -> Router<AppState> = router;
