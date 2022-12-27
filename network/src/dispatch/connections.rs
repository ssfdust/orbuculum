//! Connection Module
//!
//! The module is used to provide the api about connection for the NetworkManager.
//!
//! Functions:
//! - create_wired_connection
//! - list_wired_connection
//! - delete_connection
use super::{create_client, NetworkResponse};
use eyre::Result;
use glib::future_with_timeout;
use nm::{
    ConnectionExt, SettingConnection, SettingWired, SimpleConnection,
    SETTING_WIRED_SETTING_NAME,
};

/// The simplified connection struct
pub struct Connection {
    pub name: String,
    pub uuid: String,
    pub interface: Option<String>,
}

impl Connection {
    fn new(name: String, uuid: String, interface: Option<String>) -> Self {
        Self {
            name,
            uuid,
            interface,
        }
    }
}

/// Create a new connection via `Connection Name` and `Device Name`
///
/// * `device`: The network interface name or the network mac address.
/// * `conn_name`: The desired connection name.
pub async fn create_wired_connection(conn_name: String, device: String) -> Result<NetworkResponse> {
    let client = create_client().await?;

    let connection = SimpleConnection::new();
    let s_connection = SettingConnection::new();

    s_connection.set_type(Some(&SETTING_WIRED_SETTING_NAME));
    s_connection.set_id(Some(&conn_name));
    s_connection.set_autoconnect(true);
    if device.contains(":") {
        let wired_settings = SettingWired::new();
        wired_settings.set_mac_address(Some(&device));
        connection.add_setting(&wired_settings);
    } else {
        s_connection.set_interface_name(Some(&device));
    }
    connection.add_setting(&s_connection);

    if let Err(_) = future_with_timeout(std::time::Duration::from_millis(600), async {
        client
            .add_connection_future(&connection, true)
            .await
            .unwrap();
    })
    .await
    {
        println!("add connection {} timeout", conn_name);
    }

    Ok(NetworkResponse::Success)
}

/// List all connections in NetworkManager.
pub async fn list_connections() -> Result<NetworkResponse> {
    let client = create_client().await?;
    let nm_connecionts = client
        .connections()
        .iter()
        .filter_map(|x| {
            let mut connection = None;
            if let Some(name) = x.id() {
                if let Some(uuid) = x.uuid() {
                    if let Some(interface) = x.interface_name() {
                        connection = Some(Connection::new(
                            name.to_string(),
                            uuid.to_string(),
                            Some(interface.to_string()),
                        ))
                    } else {
                        connection = Some(Connection::new(name.to_string(), uuid.to_string(), None))
                    }
                }
            }
            connection
        })
        .collect();
    Ok(NetworkResponse::ListConnection(nm_connecionts))
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
