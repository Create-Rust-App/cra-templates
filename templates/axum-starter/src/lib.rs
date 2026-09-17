//! Axum HTTP API starter.
//!
//! Feature-based layout: each domain lives under `src/routes/<feature>/`
//! with its own router, handlers, and types. Shared application state and
//! router assembly live in [`app`]; runtime configuration in [`config`].

pub mod app;
pub mod config;
pub mod routes;
pub mod telemetry;
