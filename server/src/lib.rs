use tonic::{transport::Server, Request, Response, Status};
use network_grpc::{ConnectionReply, ConnectionReplyBody};
use network_grpc::network_server::{Network, NetworkServer};
use eyre::Result;
use tokio::macros::support::Future;

pub mod network_grpc {
    tonic::include_proto!("network"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct NetworkService {}

#[tonic::async_trait]
impl Network for NetworkService {
    async fn list_connection(&self, request: Request<()>) -> Result<Response<ConnectionReply>, Status> {
        println!("Got a request: {:?}", request);
        let reply = ConnectionReply {
            code: 0,
            msg: "".into(),
            data: vec![]
        };
        Ok(Response::new(reply))
    }
}


pub fn create_server() -> impl Future<Output = Result<(), tonic::transport::Error>> {
    let addr = "127.0.0.1:50051".parse().unwrap();
    let network_service = NetworkService::default();

    Server::builder().add_service(NetworkServer::new(network_service)).serve(addr)
}
