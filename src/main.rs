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
    devices_paths.sort_by(|a, b| a.1.cmp(&b.1));

    devices_paths.iter().map(|x| x.0.clone()).collect()
}

#[tokio::main]
async fn main() {
    let (glib_sender, glib_receiver) = create_channel();

    thread::spawn(move || {
        run_network_manager_loop(glib_receiver);
    });

    let shared_state = Arc::new(State::new(glib_sender));
}
