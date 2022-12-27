//! IP Config Module
//!
//! The module is used to provide the api about ip configuration in NM.
//! It provides both IpV4 and IpV6 Configuration.
use super::{create_client, NetworkResponse};
use eyre::Result;
use ipnet::IpNet;
use libc::{AF_INET, AF_INET6};
use nm::{ConnectionExt, IPAddress, IPRoute, SettingIPConfig, SettingIPConfigExt};
use std::boxed::Box;
use std::net::IpAddr;

/// The Ip configuration struct
#[derive(Debug, Default)]
pub struct IPConfig {
    pub method: String,
    pub addresses: Vec<IpNet>,
    pub gateway: Option<IpAddr>,
    pub dns: Vec<IpAddr>,
    pub routes: Vec<Route>,
}

/// The Route struct
#[derive(Debug, Default)]
pub struct Route {
    pub family: i32,
    pub dest: IpNet,
    pub next_hop: Option<IpAddr>,
    pub metric: i64,
}

impl TryFrom<IPRoute> for Route {
    type Error = eyre::ErrReport;
    fn try_from(val: IPRoute) -> Result<Route> {
        let mut route = Route::default();
        if let Some(dest) = val.dest() {
            route.family = val.family();
            route.dest = format!("{}/{}", dest.to_string(), val.prefix()).parse()?;
            route.next_hop = val.next_hop().map(|x| x.parse().unwrap());
            route.metric = val.metric();
        } else {
            bail!("No dest found in route connection");
        }
        Ok(route)
    }
}

impl TryInto<IPRoute> for Route {
    type Error = eyre::ErrReport;
    fn try_into(self) -> Result<IPRoute> {
        let iproute = IPRoute::new(
            self.family,
            &self.dest.addr().to_string(),
            self.dest.prefix_len() as u32,
            self.next_hop
                .map(|x| &*Box::leak(x.to_string().into_boxed_str())),
            self.metric,
        )?;
        Ok(iproute)
    }
}

fn ipaddr2ipnet(ipaddr: IPAddress) -> Result<IpNet> {
    match ipaddr.address() {
        Some(addr) => {
            let addr_with_prefix = format!("{}/{}", addr, ipaddr.prefix());
            let ipnet = addr_with_prefix.parse()?;
            Ok(ipnet)
        }
        _ => bail!("Failed to convert ipaddr to ipnet"),
    }
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
pub async fn get_ip_config(conn_name: String, family: i32) -> Result<NetworkResponse> {
    let client = create_client().await?;
    let ipconfig: Option<IPConfig> = try {
        let connection: nm::RemoteConnection = client.connection_by_id(&conn_name)?;
        let mut config = IPConfig::default();
        let ipconfig: SettingIPConfig;

        // Parser configuration
        if family == 4 {
            ipconfig = connection.setting_ip4_config().map(|x| x.into())?;
        } else {
            ipconfig = connection.setting_ip6_config().map(|x| x.into())?;
        }

        // Get configuration method
        let method = ipconfig.method()?;
        config.method = method.into();

        // Get all ip addresses in the connection
        for i in 0..ipconfig.num_addresses() as i32 {
            if let Some(Ok(ipnet)) = ipconfig.address(i).map(|x| ipaddr2ipnet(x)) {
                config.addresses.push(ipnet);
            }
        }

        // Get all dnses in the configuration
        for i in 0..ipconfig.num_dns() as i32 {
            if let Some(Ok(dns)) = ipconfig.dns(i).map(|x| x.to_string().parse()) {
                config.dns.push(dns);
            }
        }

        // Get the routes of the configuration
        for i in 0..ipconfig.num_routes() as i32 {
            if let Some(Ok(route)) = ipconfig.route(i).map(|x| x.try_into()) {
                config.routes.push(route);
            }
        }

        // Get the gateway of the configuration
        if let Some(Ok(gateway)) = ipconfig.gateway().map(|x| x.to_string().parse()) {
            config.gateway = Some(gateway);
        }

        config
    };
    Ok(NetworkResponse::IP(ipconfig))
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
