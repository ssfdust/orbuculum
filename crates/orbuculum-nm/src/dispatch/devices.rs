//! Device Module
//!
//! The module is used to provide the api about network devices for
//! the NetworkManager.
use super::{create_client, NetworkResponse};
use crate::net::NetInfo;
use crate::utils::nm_display;
use eyre::Result;
use nm::{ActiveConnectionExt, ConnectionExt, Device};
use serde::Serialize;
use std::sync::Arc;

#[derive(Clone, Default, Debug, Serialize)]
pub struct ConnectionItem {
    pub id: Option<String>,
    pub uuid: Option<String>,
}

/// The network device structure
#[derive(Clone, Default, Debug, Serialize)]
pub struct NetDevice {
    /// The network interface name, e.g. ens3, eth0
    pub name: String,
    /// The first connection related to the network card would be the conn_name
    pub connection: ConnectionItem,
    pub mac: String,
    /// The network manager state
    pub state: String,
    /// The network manager device type
    pub device_type: String,
    /// Whether the device is a virtual device
    pub r#virtual: bool,
    /// Whether the device is managed by NetworkManager
    pub is_managed: bool,
    /// The network nic driver name
    pub driver: Option<String>,
    pub ip4info: Option<NetInfo>,
    pub ip6info: Option<NetInfo>,
    /// all the connections related to the network device
    pub conn: Vec<String>,
    /// The udev property `DEV_PATH` of the network device
    pub dev_path: Option<String>,
    /// The udev property `ID_PATH` of the network device
    pub id_path: Option<String>,
    pub net_link_modes: Vec<String>,
}

fn get_latest_connection(
    connections: &mut Vec<nm::RemoteConnection>,
) -> Option<&nm::RemoteConnection> {
    connections.sort_by(|a, b| {
        let ts_a = a
            .setting_connection()
            .map(|x| x.timestamp())
            .unwrap_or(0u64);
        let ts_b = b
            .setting_connection()
            .map(|x| x.timestamp())
            .unwrap_or(0u64);
        ts_b.cmp(&ts_a)
    });
    connections.get(0)
}

pub fn get_managed_status(device: &Device) -> bool {
    use nm::DeviceExt;
    device.is_managed()
}

/// List all interfaces with network manager connection names.
///
/// The function shows all information about the interfaces, including interface
/// name, device_type, associated connection names and ip addresses.
///
/// The returned result is not user friendly, high layer application should
/// convert the result by themselfs.
pub async fn list_ether_devices(link_modes: Arc<serde_json::Value>) -> Result<NetworkResponse> {
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
                    let iface_clone = interface.clone();
                    let device_type = nm_display(device.device_type());
                    let conn = device
                        .available_connections()
                        .into_iter()
                        .map(|x| x.uuid().map(|x| x.to_string()))
                        .filter_map(|x| x)
                        .collect();
                    let connection = device
                        .active_connection()
                        .map(|x| {
                            let id = x.id().map(|x| x.to_string());
                            let uuid = x.uuid().map(|x| x.to_string());
                            ConnectionItem { id, uuid }
                        })
                        .unwrap_or(
                            get_latest_connection(
                                &mut client
                                    .connections()
                                    .into_iter()
                                    .filter_map(|x| {
                                        if x.interface_name().as_ref() == Some(&iface_clone) {
                                            Some(x)
                                        } else {
                                            None
                                        }
                                    })
                                    .collect::<Vec<nm::RemoteConnection>>(),
                            )
                            .and_then(|y| {
                                let id = y.id().map(|x| x.to_string());
                                let uuid = y.uuid().map(|x| x.to_string());
                                Some(ConnectionItem { id, uuid })
                            })
                            .unwrap_or_default(),
                        );
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
                    let driver = device.driver().map(|x| x.to_string());
                    let net_link_modes: Vec<String> = link_modes[interface.to_string()]
                        .as_array()
                        .and_then(|x| {
                            Some(
                                x.iter()
                                    .filter_map(|x| x.as_str().map(|x| x.to_string()))
                                    .collect(),
                            )
                        })
                        .unwrap_or(vec![]);
                    net_dev = NetDevice {
                        name: interface.to_string(),
                        connection,
                        ip4info,
                        state,
                        r#virtual: device.is_software(),
                        is_managed,
                        ip6info,
                        driver,
                        dev_path,
                        id_path,
                        device_type,
                        mac: mac.to_string(),
                        conn,
                        net_link_modes,
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
    let device_interface = format!("{}.Device", nm::DBUS_INTERFACE);
    if let Some(device) = client.device_by_iface(&device_name) {
        if let Some(device_object_path) = device.path().map(|x| x.to_string()) {
            let managed_status = glib::Variant::from(is_managed);
            let current_status = get_managed_status(&device);
            if current_status != is_managed {
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
        }
    } else {
        bail!("The given network device is not found.")
    }
    Ok(NetworkResponse::Success)
}
