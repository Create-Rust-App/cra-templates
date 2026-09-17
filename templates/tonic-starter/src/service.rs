//! Greeter service implementation.

use tonic::{Request, Response, Status};

use crate::proto::hello::{greeter_server::Greeter, HelloReply, HelloRequest};

/// Greeter service: unary hello RPC.
#[derive(Debug, Default)]
pub struct GreeterService;

#[tonic::async_trait]
impl Greeter for GreeterService {
    /// Reply `Hello, {name}!` to a greeting request.
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let name = request.into_inner().name;
        tracing::info!(name = %name, "greeting request");
        Ok(Response::new(HelloReply {
            message: format!("Hello, {name}!"),
        }))
    }
}

/// Render the greeting message (shared with tests).
pub fn greeting(name: &str) -> String {
    format!("Hello, {name}!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn say_hello_greets_by_name() {
        let service = GreeterService;
        let response = service
            .say_hello(Request::new(HelloRequest {
                name: "Ferris".to_string(),
            }))
            .await
            .expect("greeting");
        assert_eq!(response.into_inner().message, "Hello, Ferris!");
    }
}
