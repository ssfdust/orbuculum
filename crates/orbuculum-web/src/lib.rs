use std::sync::Arc;

use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use orbuculum_grpc::{
    ConnectionBody, ConnectionUuidRequest, HostnameBody, NetworkClient, NetworkingStateBody,
};
use serde_json::Value;
pub struct GrpcInfo {
    address: String,
}

impl GrpcInfo {
    pub fn new(address: &str) -> Self {
        let address = address.to_owned();
        Self { address }
    }

    pub fn address(&self) -> String {
        self.address.clone()
    }
}

pub async fn health() -> StatusCode {
    StatusCode::OK
}

pub async fn list_devices(State(grpc_info): State<Arc<GrpcInfo>>) -> axum::extract::Json<Value> {
    let grpc_addr = grpc_info.address();
    let mut client = NetworkClient::connect(grpc_addr).await.unwrap();

    let request = tonic::Request::new(().into());

    let response = client.list_devices(request).await.unwrap();
    let json_val = serde_json::to_value(response.into_inner()).unwrap();
    json_val.into()
}

pub async fn list_connections(
    State(grpc_info): State<Arc<GrpcInfo>>,
) -> axum::extract::Json<Value> {
    let grpc_addr = grpc_info.address();
    let mut client = NetworkClient::connect(grpc_addr).await.unwrap();

    let request = tonic::Request::new(().into());

    let response = client.list_connections(request).await.unwrap();
    let json_val = serde_json::to_value(response.into_inner()).unwrap();
    json_val.into()
}

pub async fn update_connections(
    State(grpc_info): State<Arc<GrpcInfo>>,
    Json(connections): Json<Vec<ConnectionBody>>,
) -> axum::extract::Json<Value> {
    let grpc_addr = grpc_info.address();
    let mut client = NetworkClient::connect(grpc_addr).await.unwrap();

    // Update connections one by one
    for connection in connections {
        let request = tonic::Request::new(connection);
        client.update_connection(request).await.unwrap();
    }

    // Restart overall network
    let request = tonic::Request::new(().into());
    client.restart_networking(request).await.unwrap();

    // get current network connections
    let request = tonic::Request::new(().into());
    let response = client.list_connections(request).await.unwrap();
    let json_val = serde_json::to_value(response.into_inner()).unwrap();
    json_val.into()
}

pub async fn get_hostname(State(grpc_info): State<Arc<GrpcInfo>>) -> axum::extract::Json<Value> {
    let grpc_addr = grpc_info.address();
    let mut client = NetworkClient::connect(grpc_addr).await.unwrap();

    let request = tonic::Request::new(().into());

    let response = client.get_hostname(request).await.unwrap();
    let json_val = serde_json::to_value(response.into_inner()).unwrap();
    json_val.into()
}

pub async fn set_hostname(
    State(grpc_info): State<Arc<GrpcInfo>>,
    Json(hostname_json): Json<HostnameBody>,
) -> axum::extract::Json<Value> {
    let grpc_addr = grpc_info.address();
    let mut client = NetworkClient::connect(grpc_addr).await.unwrap();

    let request = tonic::Request::new(hostname_json);

    let response = client.set_hostname(request).await.unwrap();
    let json_val = serde_json::to_value(response.into_inner()).unwrap();
    json_val.into()
}

pub async fn get_connection_by_uuid(
    Path(uuid): Path<String>,
    State(grpc_info): State<Arc<GrpcInfo>>,
) -> axum::extract::Json<Value> {
    let grpc_addr = grpc_info.address();
    let mut client = NetworkClient::connect(grpc_addr).await.unwrap();

    let request = tonic::Request::new(ConnectionUuidRequest { uuid });

    let response = client.get_connection_by_uuid(request).await.unwrap();
    let json_val = serde_json::to_value(response.into_inner()).unwrap();
    json_val.into()
}

pub async fn update_connection(
    State(grpc_info): State<Arc<GrpcInfo>>,
    Json(connection): Json<ConnectionBody>,
) -> axum::extract::Json<Value> {
    let grpc_addr = grpc_info.address();
    let mut client = NetworkClient::connect(grpc_addr).await.unwrap();

    let request = tonic::Request::new(connection);

    let response = client.update_connection(request).await.unwrap();
    let json_val = serde_json::to_value(response.into_inner()).unwrap();
    json_val.into()
}

pub async fn get_networking(State(grpc_info): State<Arc<GrpcInfo>>) -> axum::extract::Json<Value> {
    let grpc_addr = grpc_info.address();
    let mut client = NetworkClient::connect(grpc_addr).await.unwrap();

    let request = tonic::Request::new(().into());

    let response = client.get_networking(request).await.unwrap();
    let json_val = serde_json::to_value(response.into_inner()).unwrap();
    json_val.into()
}

pub async fn set_networking(
    State(grpc_info): State<Arc<GrpcInfo>>,
    Json(networking_state): Json<NetworkingStateBody>,
) -> axum::extract::Json<Value> {
    let grpc_addr = grpc_info.address();
    let mut client = NetworkClient::connect(grpc_addr).await.unwrap();

    let request = tonic::Request::new(networking_state);

    let response = client.set_networking(request).await.unwrap();
    let json_val = serde_json::to_value(response.into_inner()).unwrap();
    json_val.into()
}

pub async fn restart_networking(
    State(grpc_info): State<Arc<GrpcInfo>>,
) -> axum::extract::Json<Value> {
    let grpc_addr = grpc_info.address();
    let mut client = NetworkClient::connect(grpc_addr).await.unwrap();

    let request = tonic::Request::new(().into());

    let response = client.restart_networking(request).await.unwrap();
    let json_val = serde_json::to_value(response.into_inner()).unwrap();
    json_val.into()
}
