#[macro_use]
extern crate eyre;
mod decoder;
mod server;
mod services;
pub use server::create_server;
use services::network::NetworkService;

pub mod network_grpc {
    tonic::include_proto!("network");
    pub(crate) const NETWROK_FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("network_descriptor");
}
use network_grpc::network_server::{Network, NetworkServer};
use network_grpc::DevicesReply;
