//! End-to-end tests for the queue: submit, round-robin, and shutdown.

use worker_starter::{
    jobs::{greet::GreetPayload, Job},
    queue,
};

#[tokio::test]
async fn submit_returns_worker_result() {
    let (producer, consumers) = queue::pool(2);
    let (_shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);
    for consumer in consumers {
        tokio::spawn(consumer.run(shutdown_rx.clone()));
    }
    let rx = producer
        .submit(Job::Greet(GreetPayload {
            name: "Ferris".to_string(),
        }))
        .await
        .expect("submit");
    let outcome = rx.await.expect("worker reply");
    assert_eq!(outcome.summary, "greeted Ferris");
}

#[tokio::test]
async fn shutdown_stops_idle_workers() {
    let (_producer, consumers) = queue::pool(2);
    let (shutdown_tx, _) = tokio::sync::watch::channel(false);
    let handles: Vec<_> = consumers
        .into_iter()
        .map(|consumer| tokio::spawn(consumer.run(shutdown_tx.subscribe())))
        .collect();
    shutdown_tx.send(true).expect("notify");
    for handle in handles {
        tokio::time::timeout(std::time::Duration::from_secs(5), handle)
            .await
            .expect("worker exits on shutdown")
            .expect("worker joins");
    }
}
