//! Greet job: render a greeting for a name.

use serde::{Deserialize, Serialize};

use super::JobResult;

/// Payload for the greet job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GreetPayload {
    /// Name to greet.
    pub name: String,
}

/// Execute the greet job.
pub async fn execute(payload: GreetPayload) -> anyhow::Result<JobResult> {
    tracing::info!(name = %payload.name, "greeting");
    Ok(JobResult {
        summary: format!("greeted {}", payload.name),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn greet_summarises_name() {
        let result = execute(GreetPayload {
            name: "Ferris".to_string(),
        })
        .await
        .expect("execute");
        assert_eq!(result.summary, "greeted Ferris");
    }
}
