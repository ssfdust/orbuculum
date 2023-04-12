//! Hostname related actions via NetworkManager
//!
//! ### Functions
//! - get_hostname: get hostname of the server
//! - set_hostname: set hostname to the server
use super::{create_client, NetworkResponse};
use eyre::Result;

pub async fn get_hostname() -> Result<NetworkResponse> {
    let client = create_client().await?;
    let hostname = client.hostname().map(|x| x.to_string()).unwrap_or_default();
    let ret = serde_json::to_value(hostname)?;
    Ok(NetworkResponse::Return(ret))
}

pub async fn set_hostname(hostname: String) -> Result<NetworkResponse> {
    let client = create_client().await?;
    client.save_hostname_future(Some(&hostname)).await?;
    Ok(NetworkResponse::Success)
}
