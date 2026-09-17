//! End-to-end tests: serve on an ephemeral port, drive a real client.

use tonic::transport::Server;
use tonic_starter::{
    config::Config,
    proto::hello::{greeter_client::GreeterClient, greeter_server::GreeterServer, HelloRequest},
    service::GreeterService,
};

/// Serve the Greeter on an OS-assigned port, returning its address.
async fn serve_ephemeral() -> std::net::SocketAddr {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind ephemeral");
    let addr = listener.local_addr().expect("local addr");
    let incoming = tokio_stream::wrappers::TcpListenerStream::new(listener);
    tokio::spawn(async move {
        Server::builder()
            .add_service(GreeterServer::new(GreeterService))
            .serve_with_incoming(incoming)
            .await
            .expect("serve test instance");
    });
    addr
}

#[tokio::test]
async fn greets_over_the_wire() {
    let addr = serve_ephemeral().await;
    let mut client = GreeterClient::connect(format!("http://{addr}"))
        .await
        .expect("connect");
    let response = client
        .say_hello(HelloRequest {
            name: "Ferris".to_string(),
        })
        .await
        .expect("rpc");
    assert_eq!(response.into_inner().message, "Hello, Ferris!");
}

#[test]
fn default_config_matches_env_example() {
    let config = Config::default();
    assert_eq!(config.addr(), "0.0.0.0:50051");
}
