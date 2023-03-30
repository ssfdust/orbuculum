//! Connection Module
//!
//! The module is used to provide the api about connection for the NetworkManager.
//!
//! Functions:
//! - create_wired_connection
//! - list_wired_connection
//! - delete_connection
use super::{create_client, NetworkResponse};
use crate::net::NetInfo;
use eyre::Result;
use glib::future_with_timeout;
use ipnet::IpNet;
use libc::{AF_INET, AF_INET6};
use nm::{
    ConnectionExt, DeviceExt, IPAddress, SettingConnection, SettingIP4Config, SettingIP6Config,
    SettingIPConfig, SettingIPConfigExt, SettingWired, SimpleConnection,
    SETTING_WIRED_SETTING_NAME,
};
use serde::Serialize;
use std::boxed::Box;

/// The simplified connection struct
#[derive(Serialize)]
pub struct Connection {
    pub name: String,
    pub uuid: String,
    pub interface: Option<String>,
    pub mac: Option<String>,
    pub ip4info: NetInfo,
    pub ip6info: NetInfo,
}

impl Connection {
    fn new(
        name: String,
        uuid: String,
        interface: Option<String>,
        mac: Option<String>,
        ip4info: NetInfo,
        ip6info: NetInfo,
    ) -> Self {
        Self {
            name,
            uuid,
            mac,
            interface,
            ip4info,
            ip6info,
        }
    }

    fn from_nm_connection(
        nm_connection: &nm::RemoteConnection,
        client: &nm::Client,
    ) -> Option<Self> {
        let mut connection = None;
        if let Some(name) = nm_connection.id() {
            if let Some(uuid) = nm_connection.uuid() {
                if let Some(interface) = nm_connection.interface_name() {
                    if let Ok(ip4config) = get_ip_config(nm_connection, 4) {
                        if let Ok(ip6config) = get_ip_config(nm_connection, 6) {
                            let mac = client
                                .device_by_iface(&interface)
                                .and_then(|x| x.hw_address().map(|x| x.to_string()));
                            connection = Some(Connection::new(
                                name.to_string(),
                                uuid.to_string(),
                                Some(interface.to_string()),
                                mac,
                                ip4config,
                                ip6config,
                            ))
                        }
                    }
                }
            }
        }
        connection
    }
}

/// Rename a network connection with the UUID
pub async fn rename_connection(conn_uuid: String, new_name: String) -> Result<NetworkResponse> {
    let client = create_client().await?;
    match client.connection_by_uuid(&conn_uuid) {
        Some(connection) => {
            let setting = connection
                .setting_connection()
                .expect("Failed to get connection setting");
            setting.set_id(Some(&new_name));
            connection.commit_changes_future(true).await?;
        }
        _ => bail!("Uuid {} not found", conn_uuid),
    }
    Ok(NetworkResponse::Success)
}

/// Create a new connection via `Connection Name` and `Device Name`
///
/// * `device`: The network interface name or the network mac address.
/// * `conn_name`: The desired connection name.
pub async fn create_wired_connection(conn_name: String, device: String) -> Result<NetworkResponse> {
    let client = create_client().await?;

    let connection = SimpleConnection::new();
    let s_connection = SettingConnection::new();
    let mut uuid = String::new();

    s_connection.set_type(Some(&SETTING_WIRED_SETTING_NAME));
    s_connection.set_id(Some(&conn_name));
    s_connection.set_autoconnect(true);
    if device.contains(":") {
        let wired_settings = SettingWired::new();
        wired_settings.set_mac_address(Some(&device));
        connection.add_setting(wired_settings);
    } else {
        s_connection.set_interface_name(Some(&device));
    }
    connection.add_setting(s_connection);

    match future_with_timeout(std::time::Duration::from_millis(600), async {
        client.add_connection_future(&connection, true).await
    })
    .await
    {
        Ok(Ok(connection)) => {
            uuid = connection.uuid().map(|x| x.to_string()).unwrap_or(uuid);
        }
        _ => eprintln!("add connection {} timeout", conn_name),
    }

    Ok(NetworkResponse::Return(
        serde_json::to_value(&uuid).unwrap(),
    ))
}

/// List all connections in NetworkManager.
pub async fn list_connections() -> Result<NetworkResponse> {
    let client = create_client().await?;
    let nm_connecionts: Vec<Connection> = client
        .connections()
        .iter()
        .filter_map(|x| Connection::from_nm_connection(x, &client))
        .collect();
    let nm_connecionts = serde_json::to_value(nm_connecionts)?;
    Ok(NetworkResponse::Return(nm_connecionts))
}

/// Get a connection by connection uuid
pub async fn get_connection(uuid: String) -> Result<NetworkResponse> {
    let client = create_client().await?;
    if let Some(Some(connection)) = client
        .connection_by_uuid(&uuid)
        .map(|x| Connection::from_nm_connection(&x, &client))
    {
        let connection = serde_json::to_value(&connection)?;
        Ok(NetworkResponse::Return(connection))
    } else {
        bail!("Failed to get connection with uuid {}", uuid)
    }
}

/// Delete connections by name, this function will delete all the connections
/// which match the name.
pub async fn delete_connection(conn_name: String) -> Result<NetworkResponse> {
    let client = create_client().await?;
    let nm_connecionts: Vec<nm::RemoteConnection> = client
        .connections()
        .into_iter()
        .filter_map(|x| {
            let mut connection = None;
            if let Some(name) = x.id() {
                if name == conn_name {
                    connection = Some(x)
                }
            }
            connection
        })
        .collect();
    for conn in nm_connecionts {
        conn.delete_future().await?
    }
    Ok(NetworkResponse::Success)
}

fn ipnet2ipaddr(ipnet: IpNet) -> Result<IPAddress> {
    let ipaddress: IPAddress;
    match ipnet {
        IpNet::V4(v4) => {
            ipaddress = IPAddress::new(AF_INET, &v4.addr().to_string(), v4.prefix_len() as u32)?;
        }
        IpNet::V6(v6) => {
            ipaddress = IPAddress::new(AF_INET6, &v6.addr().to_string(), v6.prefix_len() as u32)?;
        }
    }
    Ok(ipaddress)
}

/// Get the configuration via connection name and ip family
fn get_ip_config(connection: &nm::RemoteConnection, family: i32) -> Result<NetInfo> {
    if family == 4 {
        if let Some(setting_ip4_config) = connection
            .setting_ip4_config()
            .map(|x| <SettingIP4Config as Into<SettingIPConfig>>::into(x))
        {
            NetInfo::try_from(setting_ip4_config)
        } else {
            bail!("Failed to get ipv4 config")
        }
    } else {
        if let Some(setting_ip6_config) = connection
            .setting_ip6_config()
            .map(|x| <SettingIP6Config as Into<SettingIPConfig>>::into(x))
        {
            NetInfo::try_from(setting_ip6_config)
        } else {
            bail!("Failed to get ipv6 config")
        }
    }
}

/// Update the settings of IP configuration
pub async fn update_ip_config(
    conn_name: String,
    family: i32,
    config: NetInfo,
) -> Result<NetworkResponse> {
    let client = create_client().await?;
    let _conn: Option<nm::RemoteConnection> = try {
        let connection: nm::RemoteConnection = client.connection_by_id(&conn_name)?;
        let ipconfig: SettingIPConfig;

        // Parser configuration
        if family == 4 {
            ipconfig = connection.setting_ip4_config().map(|x| x.into())?;
        } else {
            ipconfig = connection.setting_ip6_config().map(|x| x.into())?;
        }

        ipconfig.set_method(Some(&config.method));
        ipconfig.set_gateway(
            config
                .gateway
                .map(|x| &*Box::leak(x.to_string().into_boxed_str())),
        );

        ipconfig.clear_addresses();
        for address in config.addresses {
            ipconfig.add_address(&ipnet2ipaddr(address).ok()?);
        }

        ipconfig.clear_dns();

        for dns in config.dns {
            ipconfig.add_dns(&dns.to_string());
        }

        ipconfig.clear_routes();
        for route in config.routes {
            ipconfig.add_route(&route.try_into().ok()?);
        }

        connection.commit_changes_future(true).await.unwrap();
        connection
    };
    Ok(NetworkResponse::Success)
}
