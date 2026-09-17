//! Served OpenAPI document plus Redoc UI for the Axum service.
//!
//! Registered on [`crate::app::CUSTOM_ROUTERS`] so the document endpoints
//! merge under the API prefix without forking the template router.

use axum::{routing::get, Json, Router};
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};

use crate::app::{AppState, CUSTOM_ROUTERS};

/// Serve the generated OpenAPI document as JSON.
#[utoipa::path(
    get,
    path = "/api/v1/openapi.json",
    responses(
        (status = 200, description = "OpenAPI 3.1 document for this service")
    )
)]
async fn serve_spec() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

/// OpenAPI document describing the served endpoints.
#[derive(OpenApi)]
#[openapi(paths(serve_spec))]
struct ApiDoc;

/// Router exposing `/openapi.json` and the Redoc UI under `/docs`.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/openapi.json", get(serve_spec))
        .merge(Redoc::with_url("/docs", ApiDoc::openapi()))
}

#[linkme::distributed_slice(CUSTOM_ROUTERS)]
static OPENAPI_ROUTER: fn() -> Router<AppState> = router;
