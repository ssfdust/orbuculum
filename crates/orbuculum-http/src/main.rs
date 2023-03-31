use orbuculum_grpc::{NetworkClient, ConnectionUuidRequest};
use axum::{routing::get, extract::Path};
use serde_json::Value;

#[tokio::main]
async fn main() {
    // Build our application by creating our router.
    let app = axum::Router::new()
        .route("/api/proxy/devices", get(list_devices))
        .route("/api/proxy/connections", get(list_connections))
        .route("/api/proxy/connection/:uuid", get(get_connection_by_uuid));

    // Run our application as a hyper server on http://localhost:3000.
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
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
