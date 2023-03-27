use eyre::bail;
use network::{
    create_channel, gather_link_modes, run_network_manager_loop, send_command, NetworkCommand,
    State,
};
use nicrules::get_nic_ord_types;
use std::env;
use std::sync::Arc;
use std::thread;

#[tokio::main]
async fn main() {
    let (glib_sender, glib_receiver) = create_channel();
    let link_modes = gather_link_modes(None).await.unwrap();
    let arc_link_modes = Arc::new(link_modes);
    let nicrule_file = env::args()
        .nth(1)
        .expect("Please provide the rule file name.");

    thread::spawn(move || {
        run_network_manager_loop(glib_receiver, arc_link_modes);
    });
    let shared_state = Arc::new(State::new(glib_sender));
    let mut devices = send_command(shared_state, NetworkCommand::ListDeivces)
        .await
        .and_then(|x| Ok(x.into_value().unwrap()))
        .unwrap();
    if let Some(devices) = devices.as_array_mut() {
        devices.sort_by(|device_a, device_b| {
            match get_nic_ord_types(&nicrule_file, device_a).and_then(|device_ord_type_a| {
                get_nic_ord_types(&nicrule_file, device_b).and_then(|device_ord_type_b| {
                    if device_ord_type_a != device_ord_type_b {
                        Ok(device_ord_type_a.cmp(&device_ord_type_b))
                    } else {
                        bail!("The ord type is the same.")
                    }
                })
            }) {
                Ok(ord) => ord,
                _ => {
                    let id_path_a = device_a["id_path"].as_str().unwrap_or("");
                    let id_path_b = device_b["id_path"].as_str().unwrap_or("");
                    id_path_a.cmp(id_path_b)
                }
            }
        });
        for (idx, device) in devices.iter().enumerate() {
            if let Some(device_name) = device["name"].as_str() {
                let ord_type = get_nic_ord_types(&nicrule_file, device)
                    .expect("Error when run the rule script.");
                println!("Index: {}, Interface name: {}, Type Order: {}", idx, device_name, ord_type);
            }
        };
    };
}
