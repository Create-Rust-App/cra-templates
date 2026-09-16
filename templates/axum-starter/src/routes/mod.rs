//! Domain route modules.
//!
//! Each feature owns a submodule exposing a `router()` function. Copy
//! `health.rs` (or the `_feature_template` below) to add a new feature,
//! then merge it in [`crate::app::create_app`].

pub mod _feature_template;
pub mod health;
