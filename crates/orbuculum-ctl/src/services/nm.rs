//! ### Network Manager module
//! The module to interact with orbuculum-grpc server
//!
//! ### Functions:
//! -
use std::sync::Arc;

use orbuculum_grpc::{NetworkClient, ConnectionUuidRequest, ConnectionBody, HostnameBody, NetworkingStateBody};
use eyre::Result;

async fn get_devices(grpc_addr: Arc<&str>) -> Result<serde_json::Value> {
    let mut client = NetworkClient::connect(grpc_addr.to_string()).await?;
    let request = tonic::Request::new(().into());
    let response = client.list_devices(request).await?;
    let json_val = serde_json::to_value(response.into_inner())?;
    Ok(json_val)
}
