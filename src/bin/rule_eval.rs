use network::{
    create_channel, gather_link_modes, run_network_manager_loop, send_command, NetworkCommand,
    State,
};
use nicrules::{insert_device_con_names, sort_devices};
use std::collections::HashMap;
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

    if let Some(devices) = sort_devices(&nicrule_file, &mut devices) {
        for (idx, device) in devices.iter().enumerate() {
            if let Some(device_name) = device["name"].as_str() {
                let ord_type = device["type_ord"]
                    .as_i64()
                    .expect("type_ord must be a number");
                println!("==================================================");
                println!("Origin: {}", serde_json::to_string_pretty(device).unwrap());
                println!(
                    "Index: {}, Interface name: {}, Type Order: {}",
                    idx, device_name, ord_type
                );
                println!("==================================================\n");
            }
        }
        let mut groups: HashMap<String, Vec<serde_json::Value>> = HashMap::new();
        for device in devices.iter() {
            if let Some(value) = device.get("device_type") {
                let entry = groups
                    .entry(value.as_str().map(|x| x.to_string()).unwrap())
                    .or_default();
                entry.push(device.clone());
            }
        }
        for (key, items) in groups.iter() {
            serde_json::to_value(&items).unwrap();
            println!("Key: {}, Count Of Items: {}", key, items.len());
        }
        println!("==================================================\n");
        let devices = insert_device_con_names(&nicrule_file, &devices).unwrap();
        println!("Final Result:");
        for device in devices {
            if let Some(conn) = device.get("con_name") {
                if let Some(interface) = device.get("name") {
                    let device_type = device
                        .get("device_type")
                        .map(|x| x.as_str().unwrap())
                        .unwrap();
                    println!(
                        "Device Type: {}, Interface Name: {}, Connection: {}",
                        device_type,
                        interface.as_str().map(|x| x.to_string()).unwrap(),
                        conn.as_str().map(|x| x.to_string()).unwrap()
                    );
                }
            }
        }
    };
}
