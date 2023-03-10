//! Device Module
//!
//! The module is used to provide the api about network devices for
//! the NetworkManager.
use super::{create_client, NetworkResponse};
use crate::net::NetInfo;
use crate::utils::nm_display;
use eyre::Result;
use nm::ConnectionExt;
use serde::Serialize;

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
    /// Whether the device is managed by NetworkManager
    pub is_managed: bool,
    pub ip4info: Option<NetInfo>,
    pub ip6info: Option<NetInfo>,
    /// all the connections related to the network device
    pub conn: Vec<String>,
    /// The udev property `DEV_PATH` of the network device
    pub dev_path: Option<String>,
    /// The udev property `ID_PATH` of the network device
    pub id_path: Option<String>,
}

/// List all interfaces with network manager connection names.
///
/// The function shows all information about the interfaces, including interface
/// name, device_type, associated connection names and ip addresses.
///
/// The returned result is not user friendly, high layer application should
/// convert the result by themselfs.
pub async fn list_ether_devices() -> Result<NetworkResponse> {
    use nm::DeviceExt;
    let client = create_client().await?;

    let devices: Vec<NetDevice> = client
        .devices()
        .into_iter()
        .map(|device| {
            let mut net_dev = NetDevice::default();
            if let Some(interface) = device.interface() {
                if let Some(mac) = device.hw_address() {
                    let state = nm_display(device.state());
                    let is_managed = device.is_managed();
                    let device_type = nm_display(device.device_type());
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
                    let dev_path = device.udi().map(|x| x.to_string());
                    let id_path = device.path().map(|x| x.to_string());
                    net_dev = NetDevice {
                        name: interface.to_string(),
                        ip4info,
                        state,
                        r#virtual: device.is_software(),
                        is_managed,
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

/// Change the manage status for a network device.
///
/// For some applications that conflict with the network manager. we need to
/// set the target device to be unmanaged status. e.g dpdk, or some network
/// traffic trace softwares.
pub async fn set_manage(device_name: String, is_managed: bool) -> Result<NetworkResponse> {
    use nm::traits::ObjectExt;
    let client = create_client().await?;
    let device_interface = format!("{}.Device", *nm::DBUS_INTERFACE);
    if let Some(device) = client.device_by_iface(&device_name) {
        if let Some(device_object_path) = device.path().map(|x| x.to_string()) {
            let managed_status = glib::Variant::from(is_managed);

            client
                .dbus_set_property_future(
                    &device_object_path,
                    &device_interface,
                    "Managed",
                    &managed_status,
                    2000,
                )
                .await?;
        }
    } else {
        bail!("The given network device is not found.")
    }
    Ok(NetworkResponse::Success)
}
