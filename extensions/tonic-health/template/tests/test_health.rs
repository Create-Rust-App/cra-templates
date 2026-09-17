//! Health probe end-to-end test: the standard `grpc.health.v1` service
//! reports the Greeter `SERVING` on an ephemeral server.
//!
//! Marking serving is spawned during assembly, so the test polls until the
//! status lands (bounded, well under a second in practice).

use std::time::{Duration, Instant};

use tonic::transport::Server;
use tonic_health::pb::{
    health_check_response::ServingStatus, health_client::HealthClient, HealthCheckRequest,
};
use tonic_starter::{health::add_health, proto::hello::greeter_server::GreeterServer};

async fn serve_ephemeral() -> std::net::SocketAddr {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind ephemeral");
    let addr = listener.local_addr().expect("local addr");
    let incoming = tokio_stream::wrappers::TcpListenerStream::new(listener);
    tokio::spawn(async move {
        let router = Server::builder()
            .add_service(GreeterServer::new(tonic_starter::service::GreeterService));
        add_health(router)
            .serve_with_incoming(incoming)
            .await
            .expect("serve test instance");
    });
    addr
}

#[tokio::test]
async fn greeter_reports_serving() {
    let addr = serve_ephemeral().await;
    // The checked-in health bindings omit the `connect` helper, so dial
    // explicitly like the generated code does.
    let channel = tonic::transport::Channel::from_shared(format!("http://{addr}"))
        .expect("valid uri")
        .connect()
        .await
        .expect("connect");
    let mut client = HealthClient::new(channel);
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let status = client
            .check(HealthCheckRequest {
                service: "hello.Greeter".to_string(),
            })
            .await
            .expect("health rpc")
            .into_inner()
            .status();
        if status == ServingStatus::Serving {
            return;
        }
        assert!(
            Instant::now() < deadline,
            "Greeter never reported SERVING (last: {status:?})"
        );
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}
