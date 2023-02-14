//! IP Config Module
//!
//! The module is used to provide the api about ip configuration in NM.
//! It provides both IpV4 and IpV6 Configuration.
use super::{create_client, NetworkResponse};
use eyre::Result;
use ipnet::IpNet;
use libc::{AF_INET, AF_INET6};
use nm::{ConnectionExt, IPAddress, SettingIPConfig, SettingIPConfigExt, SettingIP4Config, SettingIP6Config};
use std::boxed::Box;
use crate::utils::IPConfig;

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
pub async fn get_ip_config(conn_name: String, family: i32) -> Result<NetworkResponse> {
    let client = create_client().await?;
    let ip_config_rst = if let Some(connection) = client.connection_by_id(&conn_name) {
        // Parser configuration
        if family == 4 {
            if let Some(setting_ip4_config) = connection.setting_ip4_config().map(|x| <SettingIP4Config as Into<SettingIPConfig>>::into(x)) {
                IPConfig::try_from(setting_ip4_config)
            } else {
                bail!("Failed to get ipv4 config")
            }
        } else {
            if let Some(setting_ip6_config) = connection.setting_ip6_config().map(|x| <SettingIP6Config as Into<SettingIPConfig>>::into(x)) {
                IPConfig::try_from(setting_ip6_config)
            } else {
                bail!("Failed to get ipv6 config")
            }
        }
    } else {
        bail!("Failed to get connection `{}`.", conn_name)
    };
    ip_config_rst.map(|x| NetworkResponse::IP(Some(x)))
    // Ok(NetworkResponse::IP(Some(ip_config_rst.unwrap_or(IPConfig::default()))))
}

/// Update the settings of IP configuration
pub async fn update_ip_config(
    conn_name: String,
    family: i32,
    config: IPConfig,
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
