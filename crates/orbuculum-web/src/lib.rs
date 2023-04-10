use orbuculum_grpc::{NetworkClient, ConnectionUuidRequest, ConnectionBody};
use axum::extract::{Path, Json};
use axum::http::StatusCode;
use serde_json::Value;

pub async fn health() -> StatusCode {
    StatusCode::OK
}

pub async fn list_devices() -> axum::extract::Json<Value> {
    let mut client = NetworkClient::connect("http://127.0.0.1:50051").await.unwrap();

    let request = tonic::Request::new(().into());

    let response = client.list_devices(request).await.unwrap();
    let json_val = serde_json::to_value(response.into_inner()).unwrap();
    json_val.into()

}

pub async fn list_connections() -> axum::extract::Json<Value> {
    let mut client = NetworkClient::connect("http://127.0.0.1:50051").await.unwrap();

    let request = tonic::Request::new(().into());

    let response = client.list_connections(request).await.unwrap();
    let json_val = serde_json::to_value(response.into_inner()).unwrap();
    json_val.into()

}

pub async fn get_connection_by_uuid(Path(uuid): Path<String>) -> axum::extract::Json<Value> {
    let mut client = NetworkClient::connect("http://127.0.0.1:50051").await.unwrap();

    let request = tonic::Request::new(ConnectionUuidRequest{ uuid});

    let response = client.get_connection_by_uuid(request).await.unwrap();
    let json_val = serde_json::to_value(response.into_inner()).unwrap();
    json_val.into()

}

pub async fn update_connection(Json(connection): Json<ConnectionBody>) -> axum::extract::Json<Value> {
    let mut client = NetworkClient::connect("http://127.0.0.1:50051").await.unwrap();

    let request = tonic::Request::new(connection);

    let response = client.update_connection(request).await.unwrap();
    let json_val = serde_json::to_value(response.into_inner()).unwrap();
    json_val.into()

}
