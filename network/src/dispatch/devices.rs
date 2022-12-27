//! Device Module
//!
//! The module is used to provide the api about network devices for
//! the NetworkManager.
//!
//! Functions:
//! - list_ether_devices
use super::{create_client, NetworkResponse};
use eyre::Result;
use nm::{ConnectionExt, DeviceExt, DeviceType};

#[derive(Clone)]
pub struct NetDevice {
    pub name: String,
    pub mac: String,
    pub conn: Option<String>,
}

/// List all wired interfaces
pub async fn list_ether_devices() -> Result<NetworkResponse> {
    let client = create_client().await?;

    let devices: Vec<NetDevice> = client
        .devices()
        .into_iter()
        .filter_map(|device| match device.device_type() {
            DeviceType::Ethernet => {
                let mut net_dev = None;
                if let Some(interface) = device.interface() {
                    if let Some(mac) = device.hw_address() {
                        let conn = device
                            .available_connections()
                            .into_iter()
                            .next()
                            .and_then(|x| x.id())
                            .map(|x| x.to_string());
                        net_dev = Some(NetDevice {
                            name: interface.to_string(),
                            mac: mac.to_string(),
                            conn,
                        })
                    }
                }
                net_dev
            }
            _ => None,
        })
        .collect();
    Ok(NetworkResponse::ListDeivces(devices))
}
