//! ## Net structures
//!
//! The `net` moudule contains structures used to represent ip and route objects
//! and they are the bridges between common rust objects and Glib objects.
use crate::utils::{addrs_to_string, ipver_human, to_string};
use eyre::Result;
use ipnet::IpNet;
use nm::{IPAddress, IPConfig as NMIPConfig, IPRoute, SettingIPConfig, SettingIPConfigExt};
use serde::Serialize;
use std::boxed::Box;
use std::net::IpAddr;

/// A representation of the net information
///
/// The `NetInfo` type is a combination of addresses, gateway, dns and routes.
/// The method is to idenitify how the net is initilized.
#[derive(Debug, Default, Clone, Serialize)]
pub struct NetInfo {
    pub method: String,
    #[serde(serialize_with = "addrs_to_string")]
    pub addresses: Vec<IpNet>,
    pub gateway: Option<IpAddr>,
    pub dns: Vec<IpAddr>,
    pub routes: Vec<Route>,
}

/// A representation of the route information
///
/// The `Route` type consists of four properties. The `family` property is meant
/// for the v4 or v6. The `dest` property is meant for  route destination. The
/// `next_hop` property is meant for the ip address of next hop. The `metric`
/// is meant for linux route metric.
#[derive(Debug, Default, Clone, Serialize)]
pub struct Route {
    #[serde(serialize_with = "ipver_human")]
    pub family: i32,
    #[serde(serialize_with = "to_string")]
    pub dest: IpNet,
    pub next_hop: Option<IpAddr>,
    pub metric: i64,
}

impl TryFrom<SettingIPConfig> for NetInfo {
    type Error = eyre::ErrReport;
    fn try_from(setting_ip_config: SettingIPConfig) -> Result<NetInfo> {
        // Get configuration method
        let ipconfig: Option<NetInfo> = try {
            let mut config = NetInfo::default();
            let method = setting_ip_config.method()?;
            config.method = method.into();

            // Get all ip addresses in the connection
            for i in 0..setting_ip_config.num_addresses() as i32 {
                if let Some(Ok(ipnet)) = setting_ip_config.address(i).map(|x| ipaddr2ipnet(x)) {
                    config.addresses.push(ipnet);
                }
            }

            // Get all dnses in the configuration
            for i in 0..setting_ip_config.num_dns() as i32 {
                if let Some(Ok(dns)) = setting_ip_config.dns(i).map(|x| x.to_string().parse()) {
                    config.dns.push(dns);
                }
            }

            // Get the routes of the configuration
            for i in 0..setting_ip_config.num_routes() as i32 {
                if let Some(Ok(route)) = setting_ip_config.route(i).map(|x| x.try_into()) {
                    config.routes.push(route);
                }
            }

            // Get the gateway of the configuration
            if let Some(Ok(gateway)) = setting_ip_config.gateway().map(|x| x.to_string().parse()) {
                config.gateway = Some(gateway);
            }
            config
        };
        if let Some(ip_config) = ipconfig {
            Ok(ip_config)
        } else {
            bail!("Failed to")
        }
    }
}

impl TryFrom<NMIPConfig> for NetInfo {
    type Error = eyre::ErrReport;
    fn try_from(nm_ip_config: NMIPConfig) -> Result<Self> {
        let ipconfig: Option<NetInfo> = try {
            let mut netinfo = NetInfo::default();

            netinfo.method = "unknown".into();

            // Get all ip addresses in the connection
            netinfo.addresses = nm_ip_config
                .addresses()
                .iter()
                .filter_map(|x| {
                    let addr = x.clone();
                    if let Ok(ipnet) = ipaddr2ipnet(addr) {
                        Some(ipnet)
                    } else {
                        None
                    }
                })
                .collect();

            netinfo.dns = nm_ip_config
                .nameservers()
                .iter()
                .filter_map(|x| {
                    if let Ok(dns) = x.to_string().parse() {
                        Some(dns)
                    } else {
                        None
                    }
                })
                .collect();

            // Get the routes of the configuration
            netinfo.routes = nm_ip_config
                .routes()
                .iter()
                .filter_map(|x| {
                    let addr = x.clone();
                    if let Ok(route) = Route::try_from(addr) {
                        Some(route)
                    } else {
                        None
                    }
                })
                .collect();

            // Get the gateway of the configuration
            if let Some(Ok(gateway)) = nm_ip_config.gateway().map(|x| x.to_string().parse()) {
                netinfo.gateway = Some(gateway);
            }
            netinfo
        };
        if let Some(netinfo) = ipconfig {
            Ok(netinfo)
        } else {
            bail!("Failed to")
        }
    }
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
