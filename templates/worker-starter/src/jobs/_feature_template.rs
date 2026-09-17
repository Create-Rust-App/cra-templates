//! Template for a new job module.
//!
//! Copy this file to `<feature>.rs`, declare it in `jobs/mod.rs`, and add a
//! variant to [`crate::jobs::Job`] plus a branch in
//! [`crate::jobs::dispatch`]. Delete this module if unused.

use serde::{Deserialize, Serialize};

use super::JobResult;

/// Payload for the feature job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeaturePayload {
    /// Placeholder input.
    pub input: String,
}

/// Execute the feature job.
pub async fn execute(payload: FeaturePayload) -> anyhow::Result<JobResult> {
    Ok(JobResult {
        summary: format!("processed {}", payload.input),
    })
}
