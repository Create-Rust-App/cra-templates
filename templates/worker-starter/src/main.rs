//! Binary entry point: worker pool plus demo scheduler and shutdown.

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use worker_starter::{
    config::Config,
    jobs::{greet::GreetPayload, Job},
    queue,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "worker_starter=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env();
    let (producer, consumers) = queue::pool(config.workers);
    let (shutdown_tx, _) = tokio::sync::watch::channel(false);
    for consumer in consumers {
        let watch = shutdown_tx.subscribe();
        tokio::spawn(async move {
            tracing::info!(worker = consumer.id(), "worker started");
            consumer.run(watch).await;
        });
    }

    // Demo scheduler: greet every tick until shutdown.
    let tick = config.tick_secs;
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(tick));
        loop {
            interval.tick().await;
            let job = Job::Greet(GreetPayload {
                name: "scheduler".to_string(),
            });
            if producer.enqueue(job).await.is_err() {
                break;
            }
        }
    });

    tokio::signal::ctrl_c()
        .await
        .expect("install shutdown handler");
    tracing::info!("shutting down");
    shutdown_tx.send(true).expect("notify workers");
}
