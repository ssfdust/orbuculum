mod server;
mod decoder;
use eyre::Result;
use std::sync::Arc;
use tonic::{Request, Response, Status};
use network::{NetworkResponse, State, NetDevice, send_command, NetworkCommand};
pub use server::create_server;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod network_grpc {
    include!(concat!(env!("OUT_DIR"), "/json.network.Network.rs"));
}
use network_grpc::network_server::{Network, NetworkServer};

#[derive(Serialize, Deserialize)]
pub struct Empty {}

#[derive(Serialize, Deserialize, Default)]
pub struct Test {
    name: String,
    data: Option<Value>
}

#[derive(Debug, Default)]
pub struct NetworkService {}

#[tonic::async_trait]
impl Network for NetworkService {
    async fn list_devices(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<Test>, Status> {
        let shared_state = request.extensions().get::<Arc<State>>().unwrap();
        let resp = send_command(Arc::clone(shared_state), NetworkCommand::ListDeivces).await.and_then(|x| {
            Ok(Response::new(Test{name: "aaa".to_string(), data: x.into_value()}))
        }).unwrap_or(Response::new(Test::default()));
        Ok(resp)
    }
}
