//! Device Module
//!
//! The module is used to provide the api about network devices for
//! the NetworkManager.
use super::{create_client, NetworkResponse};
use crate::net::NetInfo;
use eyre::Result;
use nm::{ConnectionExt, DeviceExt};
use serde::Serialize;
use udev::Device;
use crate::utils::format_display;

/// The network device structure
#[derive(Clone, Default, Debug, Serialize)]
pub struct NetDevice {
    pub name: String,
    pub mac: String,
    /// The network manager state
    pub state: String,
    /// The network manager device type
    pub device_type: String,
    /// Whether the device is a virtual device
    pub r#virtual: bool,
    pub ip4info: Option<NetInfo>,
    pub ip6info: Option<NetInfo>,
    /// all the connections related to the network device
    pub conn: Vec<String>,
    /// The udev property `DEV_PATH` of the network device
    pub dev_path: Option<String>,
    /// The udev property `ID_PATH` of the network device
    pub id_path: Option<String>,
}

/// Extract device property information
///
/// This function grabs linux udev property information from network devices.
fn grab_udev(interface: &str, property: &str) -> Option<String> {
    let sys_path_str = format!("/sys/class/net/{}", interface);
    let path = std::path::Path::new(&sys_path_str);
    let device = Device::from_syspath(&path).unwrap();
    device
        .property_value(property)
        .map(|x| x.to_string_lossy().to_string())
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
                    let state = format_display(device.state());
                    let device_type = format_display(device.device_type());
                    let conn = device
                        .available_connections()
                        .into_iter()
                        .map(|x| x.id().map(|x| x.to_string()))
                        .filter_map(|x| x)
                        .collect();
                    let ip4info = device
                        .ip4_config()
                        .map(|x| NetInfo::try_from(x).and_then(|x| Ok(x)).ok())
                        .unwrap_or(None);
                    let ip6info = device
                        .ip6_config()
                        .map(|x| NetInfo::try_from(x).and_then(|x| Ok(x)).ok())
                        .unwrap_or(None);
                    let dev_path = grab_udev(&interface.to_string(), "DEVPATH");
                    let id_path = grab_udev(&interface.to_string(), "ID_PATH");
                    net_dev = NetDevice {
                        name: interface.to_string(),
                        ip4info,
                        state,
                        r#virtual: dev_path.as_ref().map(|x| x.contains("virtual")).unwrap_or(false),
                        ip6info,
                        dev_path,
                        id_path,
                        device_type,
                        mac: mac.to_string(),
                        conn,
                    }
                }
            }
            net_dev
        })
        .collect();

    let value = serde_json::to_value(devices)?;
    Ok(NetworkResponse::Return(value))
}
