// Generated protobuf/gRPC code.
mod proto {
    tonic::include_proto!("iohk.chain.node");
}

pub mod client;
pub mod server;

mod convert;
mod streaming;

pub use client::Client;
pub use server::Server;
