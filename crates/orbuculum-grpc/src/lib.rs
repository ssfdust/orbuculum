#[macro_use]
extern crate eyre;
mod initilize;
mod server;
mod services;
pub use initilize::initilize_network_manager;
pub use server::create_server;
use services::nm::NetworkService;

pub mod network_grpc {
    tonic::include_proto!("network");
    pub(crate) const NETWROK_FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("network_descriptor");
}
pub use network_grpc::network_client::NetworkClient;
use network_grpc::network_server::{Network, NetworkServer};
pub use network_grpc::{
    ConnectionBody, ConnectionReply, ConnectionUuidRequest, ConnectionsReply, DevicesReply, HostnameBody
};
