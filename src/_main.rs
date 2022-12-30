use network::{
    create_channel, run_network_manager_loop, send_command, Connection, IPConfig, NetDevice,
    NetworkCommand, NetworkResponse, State,
};
use std::sync::Arc;
use std::thread;
use udev::Device;

fn sort_devices_by_udev_path(devices: Vec<NetDevice>) -> Vec<NetDevice> {
    let mut devices_paths: Vec<(NetDevice, String)> = devices
        .iter()
        .map(|net_device| {
            let device_name = net_device.name.clone();
            let sys_path_str = format!("/sys/class/net/{}", device_name);
            let path = std::path::Path::new(&sys_path_str);
            let device = Device::from_syspath(&path).unwrap();
            (
                net_device.to_owned(),
                device
                    .property_value("ID_PATH")
                    .map(|x| x.to_string_lossy().to_string())
                    .unwrap_or(String::new()),
            )
        })
        .collect();
    let mut platform_devices_paths: Vec<(NetDevice, String)> = devices_paths
        .iter()
        .filter_map(|dev_path| {
            if dev_path.1.contains("platform") {
                Some(dev_path.clone())
            } else {
                None
            }
        }).collect();
    let mut none_platform_devices_paths: Vec<(NetDevice, String)> = devices_paths
        .iter()
        .filter_map(|dev_path| {
            if dev_path.1.contains("platform") {
                None
            } else {
                Some(dev_path.clone())
            }
        }).collect();
    platform_devices_paths.sort_by(|a, b| a.1.cmp(&b.1));
    none_platform_devices_paths.sort_by(|a, b| a.1.cmp(&b.1));
    // devices_paths.sort_by(|a, b| a.1.cmp(&b.1));
    platform_devices_paths.extend(none_platform_devices_paths);

    platform_devices_paths.iter().map(|x| x.0.clone()).collect()
}

#[tokio::main]
async fn main() {
    let (glib_sender, glib_receiver) = create_channel();

    thread::spawn(move || {
        run_network_manager_loop(glib_receiver);
    });

    let shared_state = Arc::new(State::new(glib_sender));
    let mut devices: Vec<NetDevice> = vec![];
    let mut connections: Vec<Connection> = vec![];
    let mut connection: Option<&Connection>;
    let mut index: i32 = 0;

    match send_command(&shared_state, NetworkCommand::ListDeivces).await {
        Ok(NetworkResponse::ListDeivces(netdevs)) => {
            devices = sort_devices_by_udev_path(netdevs);
        }
        _ => (),
    }

    match send_command(&shared_state, NetworkCommand::ListConnections).await {
        Ok(NetworkResponse::ListConnection(conns)) => {
            connections = conns;
        }
        _ => (),
    }

    for device in &devices {
        connection = None;
        let mut ip4config: Option<IPConfig> = None;
        let name = format!("eth{}", index);
        for conn in &connections {
            if let Some(dev_conn_name) = &device.conn {
                if *dev_conn_name == conn.name {
                    connection = Some(conn);
                    break;
                }
            }
        }
        if let Some(conn) = connection {
            match send_command(
                &shared_state,
                NetworkCommand::GetIP4Config(conn.name.to_owned()),
            )
            .await
            {
                Ok(NetworkResponse::IP(Some(ipconfig))) => {
                    ip4config = Some(ipconfig);
                }
                _ => (),
            }
        }
        let cloned_name = name.clone();
        send_command(
            &shared_state,
            NetworkCommand::CreateWiredConnection(cloned_name, device.mac.clone()),
        )
        .await.unwrap();
        if let Some(ipconfig) = ip4config {
            send_command(
                &shared_state,
                NetworkCommand::UpdateIP4Config(name.clone(), ipconfig),
            )
            .await
            .unwrap();
        }

        index += 1;
    }
}
