//! Protobuf bindings generated from `proto/hello.proto` (checked in).
//!
//! Regenerate with the exact `protoc` + `tonic-build` versions from
//! `proto/REGENERATE.md`, then copy the outputs back here.

#![allow(clippy::all, clippy::pedantic)]
#![allow(missing_docs)]

/// File descriptor set for server reflection (generated alongside).
pub const FILE_DESCRIPTOR_SET: &[u8] = include_bytes!("proto/hello_descriptor.bin");

/// Greeter service bindings.
pub mod hello {
    include!("proto/hello.rs");
}
