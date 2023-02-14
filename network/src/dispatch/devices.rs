//! Device Module
//!
//! The module is used to provide the api about network devices for
//! the NetworkManager.
//!
//! Functions:
//! - list_ether_devices
use super::{create_client, NetworkResponse};
use eyre::Result;
use nm::{ConnectionExt, DeviceExt};

#[derive(Clone, Default)]
pub struct NetDevice {
    pub name: String,
    pub mac: String,
    pub device_type: String,
    pub conn: Option<String>,
}

/// List all interfaces with network manager connection names.
///
/// The function shows all information about the interfaces, including interface
/// name, device_type, associated connection names and ip addresses.
///
/// The returned result is not user friendly, high layer application should 
/// convert the result by themselfs.
pub async fn list_ether_devices() -> Result<NetworkResponse> {
    let client = create_client().await?;

    let devices: Vec<NetDevice> = client
        .devices()
        .into_iter()
        .map(|device| {
            let mut net_dev = NetDevice::default();
            if let Some(interface) = device.interface() {
                if let Some(mac) = device.hw_address() {
                    let conn = device
                        .available_connections()
                        .into_iter()
                        .next()
                        .and_then(|x| (x.id()))
                        .map(|x| x.to_string());
                    net_dev = NetDevice {
                        name: interface.to_string(),
                        device_type: device.device_type().to_string(),
                        mac: mac.to_string(),
                        conn,
                    }
                }
            }
            net_dev
        })
        .collect();
    Ok(NetworkResponse::ListDeivces(devices))
}
