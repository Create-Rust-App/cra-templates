//! Publishable library starter.
//!
//! Feature-based layout: each capability owns a module with unit tests.
//! Copy `_feature_template.rs` to add a feature, declare it here, and cover
//! it in `tests/test_lib.rs`. The `src/main.rs` demo binary exercises the
//! public API end to end.

pub mod _feature_template;
pub mod backoff;
pub mod slug;
