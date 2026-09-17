//! Job definitions: serializable payloads plus executors.
//!
//! Each job owns a submodule with its payload struct and `execute` function.
//! Copy `greet.rs` (or `_feature_template.rs`) to add a job, declare it in
//! the [`Job`] enum below, and handle it in [`dispatch`].

pub mod _feature_template;
pub mod greet;

use serde::{Deserialize, Serialize};

/// Every job the worker pool can execute (tagged for the queue).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", content = "payload")]
pub enum Job {
    /// Greet a name (demo job).
    Greet(greet::GreetPayload),
}

/// Execute one job to completion.
pub async fn dispatch(job: Job) -> anyhow::Result<JobResult> {
    match job {
        Job::Greet(payload) => greet::execute(payload).await,
    }
}

/// Outcome reported after a job completes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JobResult {
    /// Human-readable outcome, e.g. `greeted Ferris`.
    pub summary: String,
}
