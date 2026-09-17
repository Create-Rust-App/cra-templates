//! Tokio background worker starter.
//!
//! Feature-based layout: each job owns a module under `src/jobs/` exposing
//! a serializable payload plus an `execute` function. The [`queue`] moves
//! payloads from producers (scheduler, other services) to a worker pool;
//! runtime configuration lives in [`config`].

pub mod config;
pub mod jobs;
pub mod queue;
