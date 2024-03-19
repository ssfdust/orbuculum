//! ### Network Manager module
//! The module to interact with orbuculum-grpc server
//!
//! ### Functions:
//! -
use std::sync::Arc;

use eyre::{ContextCompat, Result};
use orbuculum_grpc::{ConnectionBody, ConnectionUuidRequest, NetworkClient};
use serde_json::Value;

pub async fn get_devices(grpc_addr: Arc<&str>) -> Result<Vec<Value>> {
    let mut client = NetworkClient::connect(grpc_addr.to_string()).await?;
    let request = tonic::Request::new(().into());
    let response = client.list_devices(request).await?;
    let devices = response
        .into_inner()
        .data
        .into_iter()
        .filter_map(|x| {
            serde_json::to_value(x).ok()
        })
        .collect();
    Ok(devices)
}

pub async fn get_connection(grpc_addr: Arc<&str>, uuid: String) -> Result<Value> {
    let mut client = NetworkClient::connect(grpc_addr.to_string()).await?;
    let request = tonic::Request::new(ConnectionUuidRequest { uuid });
    let response = client.get_connection_by_uuid(request).await?;
    let connection = response
        .into_inner()
        .data
        .wrap_err("Failed to get connection")?;
    let connection = serde_json::to_value(&connection)?;
    Ok(connection)
}

pub async fn update_connection(grpc_addr: Arc<&str>, connection: &Value) -> Result<()> {
    let mut client = NetworkClient::connect(grpc_addr.to_string()).await?;
    let connection_req: ConnectionBody = serde_json::from_value(connection.to_owned()).unwrap();
    let request = tonic::Request::new(connection_req);
    client.update_connection(request).await?;
    Ok(())
}

pub fn connection_json2info(connection: &Value) -> Result<String> {
    let ipv4_addresses: Vec<String> =
        serde_json::from_value(connection["ip4info"]["addresses"].clone())?;
    let ipv4_dns: Vec<String> = serde_json::from_value(connection["ip4info"]["dns"].clone())?;
    let ipv6_addresses: Vec<String> =
        serde_json::from_value(connection["ip6info"]["addresses"].clone())?;
    let ipv6_dns: Vec<String> = serde_json::from_value(connection["ip6info"]["dns"].clone())?;
    let info = format!(
        "Connection: {}\n\
        UUID: {} \n\
        \n\
        IPv4:\n\
        Method: {}\n\
        Addresses: {}\n\
        Gateway: {}\n\
        DNS: {}\n\
        \n\
        IPv6:\n\
        Method: {}\n\
        Addresses: {}\n\
        Gateway: {}\n\
        DNS: {}\n",
        connection["name"].as_str().unwrap_or(""),
        connection["uuid"].as_str().unwrap_or(""),
        connection["ip4info"]["method"].as_str().unwrap_or(""),
        ipv4_addresses.join(","),
        connection["ip4info"]["gateway"].as_str().unwrap_or(""),
        ipv4_dns.join(","),
        connection["ip6info"]["method"].as_str().unwrap_or(""),
        ipv6_addresses.join(","),
        connection["ip6info"]["gateway"].as_str().unwrap_or(""),
        ipv6_dns.join(","),
    );
    Ok(info)
}

pub fn edit_connection<T>(method: &str, ipversion: &str, connection: &Value, func: T) -> Value
where
    T: FnOnce(&str) -> (Vec<String>, String, Vec<String>),
{
    let mut new_connection = connection.clone();
    if ipversion == "IPv4" {
        if method == "DHCP" {
            new_connection["ip4info"]["method"] = "auto".into();
            new_connection["ip4info"]["addresses"] = serde_json::from_str("[]").unwrap();
            new_connection["ip4info"]["gateway"] = Value::Null;
            new_connection["ip4info"]["dns"] = serde_json::from_str("[]").unwrap();
        } else {
            let (addresses, gateway, dns) = func(ipversion);
            new_connection["ip4info"]["method"] = "manual".into();
            new_connection["ip4info"]["addresses"] = serde_json::to_value(addresses).unwrap();
            new_connection["ip4info"]["gateway"] = serde_json::to_value(gateway).unwrap();
            new_connection["ip4info"]["dns"] = serde_json::to_value(dns).unwrap();
        }
    } else {
        if method == "DHCP" {
            new_connection["ip6info"]["method"] = "auto".into();
            new_connection["ip6info"]["addresses"] = serde_json::from_str("[]").unwrap();
            new_connection["ip6info"]["gateway"] = Value::Null;
            new_connection["ip6info"]["dns"] = serde_json::from_str("[]").unwrap();
        } else {
            let (addresses, gateway, dns) = func(ipversion);
            new_connection["ip4info"]["method"] = "manual".into();
            new_connection["ip4info"]["addresses"] = serde_json::to_value(addresses).unwrap();
            new_connection["ip4info"]["gateway"] = serde_json::to_value(gateway).unwrap();
            new_connection["ip4info"]["dns"] = serde_json::to_value(dns).unwrap();
        }
    }
    new_connection
}

pub async fn restart_connection(grpc_addr: Arc<&str>, uuid: String) -> Result<()> {
    let mut client = NetworkClient::connect(grpc_addr.to_string()).await?;
    let request = tonic::Request::new(ConnectionUuidRequest { uuid });
    client.reactive_connection(request).await?;
    Ok(())
}
