//! In-memory job queue feeding the worker pool.
//!
//! Each worker owns a channel; the producer round-robins across them.
//! Results travel back over a oneshot carried by the job, so producers can
//! await outcomes. Swap this module for a durable broker (see the task-queue
//! extension) without touching job definitions.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use tokio::sync::{mpsc, oneshot, watch};

use crate::jobs::{dispatch, Job, JobResult};

/// Per-worker channel capacity.
pub const CHANNEL_CAPACITY: usize = 64;

/// A job in flight with its reply channel.
#[derive(Debug)]
struct TrackedJob {
    job: Job,
    reply: Option<oneshot::Sender<JobResult>>,
}

/// Producer handle: enqueue jobs across the worker pool.
#[derive(Debug, Clone)]
pub struct Producer {
    senders: Arc<Vec<mpsc::Sender<TrackedJob>>>,
    next: Arc<AtomicUsize>,
}

/// Consumer handle: one per worker task.
#[derive(Debug)]
pub struct Consumer {
    id: usize,
    receiver: mpsc::Receiver<TrackedJob>,
}

/// Create a producer plus one consumer per worker.
pub fn pool(workers: usize) -> (Producer, Vec<Consumer>) {
    assert!(workers > 0, "pool needs at least one worker");
    let mut senders = Vec::with_capacity(workers);
    let mut consumers = Vec::with_capacity(workers);
    for id in 0..workers {
        let (sender, receiver) = mpsc::channel(CHANNEL_CAPACITY);
        senders.push(sender);
        consumers.push(Consumer { id, receiver });
    }
    (
        Producer {
            senders: Arc::new(senders),
            next: Arc::new(AtomicUsize::new(0)),
        },
        consumers,
    )
}

impl Producer {
    /// Pick the next worker round-robin.
    fn route(&self) -> &mpsc::Sender<TrackedJob> {
        let index = self.next.fetch_add(1, Ordering::Relaxed) % self.senders.len();
        &self.senders[index]
    }

    /// Enqueue one job, returning a receiver for its result.
    pub async fn submit(&self, job: Job) -> anyhow::Result<oneshot::Receiver<JobResult>> {
        let (tx, rx) = oneshot::channel();
        self.route()
            .send(TrackedJob {
                job,
                reply: Some(tx),
            })
            .await
            .map_err(|_| anyhow::anyhow!("worker disconnected"))?;
        Ok(rx)
    }

    /// Enqueue one job without waiting for its result.
    pub async fn enqueue(&self, job: Job) -> anyhow::Result<()> {
        self.route()
            .send(TrackedJob { job, reply: None })
            .await
            .map_err(|_| anyhow::anyhow!("worker disconnected"))?;
        Ok(())
    }
}

impl Consumer {
    /// Worker id within the pool.
    pub fn id(&self) -> usize {
        self.id
    }

    /// Run the worker loop until producers disconnect or shutdown fires.
    pub async fn run(mut self, mut shutdown: watch::Receiver<bool>) {
        loop {
            tokio::select! {
                _ = shutdown.changed() => break,
                incoming = self.receiver.recv() => match incoming {
                    Some(tracked) => match dispatch(tracked.job).await {
                        Ok(outcome) => {
                            if let Some(reply) = tracked.reply {
                                let _ = reply.send(outcome);
                            }
                        }
                        Err(error) => tracing::error!(%error, "job failed"),
                    },
                    None => break,
                },
            }
        }
    }
}
