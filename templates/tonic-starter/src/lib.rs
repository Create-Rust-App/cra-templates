//! tonic gRPC microservice starter.
//!
//! Feature-based layout: the Protobuf contract lives in `proto/`, generated
//! bindings (checked in) under `src/proto/`, service implementations under
//! `src/service.rs`, and runtime configuration in [`config`].

pub mod config;
pub mod proto;
pub mod service;
