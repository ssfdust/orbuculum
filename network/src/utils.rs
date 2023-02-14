use eyre::Result;
use ipnet::IpNet;
use std::boxed::Box;
use std::net::IpAddr;
use nm::{IPAddress, IPRoute, SettingIPConfig, SettingIPConfigExt};

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

impl TryFrom<SettingIPConfig> for IPConfig {
    type Error = eyre::ErrReport;
    fn try_from(setting_ip_config: SettingIPConfig) -> Result<IPConfig> {
        // Get configuration method
        let ipconfig: Option<IPConfig> = try {
            let mut config = IPConfig::default();
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
