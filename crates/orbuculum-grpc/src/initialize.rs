use eyre::{ContextCompat, Result};
use orbuculum_nm::{send_command, NetworkCommand, State};
use orbuculum_rules::get_desired_devices;
use serde_json::{json, Value};
use std::sync::Arc;

fn get_ip_config(config: &Value, key: &str) -> Result<Value> {
    let addresses: Vec<String> = config[key]["addresses"]
        .as_array()
        .map(|values| {
            values
                .iter()
                .map(|value| {
                    value
                        .as_str()
                        .map(|address| address.to_string())
                        .unwrap_or_default()
                })
                .collect()
        })
        .wrap_err("Failed to get addresses")?;
    let dns: Vec<String> = config[key]["dns"]
        .as_array()
        .map(|values| {
            values
                .iter()
                .map(|value| {
                    value
                        .as_str()
                        .map(|address| address.to_string())
                        .unwrap_or_default()
                })
                .collect()
        })
        .wrap_err("Failed to get dns")?;
    let gateway = config[key]["gateway"]
        .as_str()
        .map(|x| x.to_string())
        .wrap_err("Failed get gateway")?;
    let method = config[key]["method"]
        .as_str()
        .map(|x| x.to_string())
        .wrap_err("Failed get method")?;
    Ok(json!({"addresses": addresses, "dns": dns, "gateway": gateway, "method": method}))
}

struct Initlizer {
    nicrule_file: String,
    devices: Vec<Value>,
    state: Arc<State>,
}

impl Initlizer {
    async fn new_future(nicrule_file: String, state: Arc<State>) -> Result<Self> {
        let cloned_state = state.clone();
        let opt_devices = send_command(cloned_state, NetworkCommand::ListDeivces)
            .await
            .and_then(|resp| {
                Ok(resp.into_value().and_then(|value| {
                    value.as_array().and_then(|items| {
                        let items: Vec<Value> = items.iter().map(|x| x.clone()).collect();
                        Some(items)
                    })
                }))
            })?;
        let devices = opt_devices.unwrap_or_default();
        Ok(Self {
            nicrule_file,
            devices,
            state,
        })
    }

    async fn restart_networking(&self) -> Result<()> {
        println!("Restarting network at startup!");
        match send_command(self.state.clone(), NetworkCommand::SetNetworking(false)).await {
            _ => (),
        }
        send_command(self.state.clone(), NetworkCommand::SetNetworking(true)).await?;
        Ok(())
    }

    async fn update_configuration(&self, device_info: &Value, uuid: &str) {
        if device_info["ip4info"].is_object() || device_info["ip6info"].is_object() {
            let resp = send_command(
                self.state.clone(),
                NetworkCommand::GetConnection(uuid.to_string()),
            )
            .await
            .unwrap();
            let mut connection = resp.into_value().expect("Failed to get connection by uuid");
            match get_ip_config(device_info, "ip4info") {
                Ok(ip4info) => connection["ip4info"] = ip4info,
                _ => (),
            }

            match get_ip_config(device_info, "ip6info") {
                Ok(ip6info) => connection["ip6info"] = ip6info,
                _ => (),
            }
            send_command(
                self.state.clone(),
                NetworkCommand::UpdateConnection(connection.to_owned()),
            )
            .await
            .unwrap();
            match send_command(
                self.state.clone(),
                NetworkCommand::Reactive(uuid.to_owned()),
            )
            .await
            {
                _ => eprintln!("Failed to activate {}", uuid),
            }
        }
    }

    async fn update_managed_state(&self, device_info: &Value) {
        let managed_state = device_info["is_managed"].as_bool().unwrap_or(false);
        let device = device_info["name"].as_str().map(|x| x.to_string()).unwrap();
        send_command(
            self.state.clone(),
            NetworkCommand::SetManage(device, managed_state),
        )
        .await
        .unwrap();
    }

    async fn init_connections(&self) -> Result<bool> {
        let devices_val = serde_json::to_value(&self.devices)?;
        let sorted_devices = get_desired_devices(&self.nicrule_file, &devices_val)
            .expect("Fail to get devices info");
        let mut need_start = false;
        for device_info in sorted_devices {
            let device_name = device_info["name"]
                .as_str()
                .expect("Failed to get name when initilising");
            let conn_name = device_info["con_name"]
                .as_str()
                .expect("Failed to get con_name when initilising");
            let current_connection = device_info["connection"]
                .as_object()
                .expect("Failed to get connection when initilising");
            let current_uuid = current_connection
                .get("uuid")
                .expect("no uuid in connection object");
            if let Some(uuid) = current_uuid.as_str() {
                let current_name = current_connection["id"].as_str().unwrap_or("");
                if current_name != conn_name {
                    println!(
                        "uuid: {}, current connection name: {}, new connection name:{}",
                        uuid, current_name, conn_name
                    );
                    need_start = true;
                    send_command(
                        self.state.clone(),
                        NetworkCommand::RenameConnection(uuid.to_owned(), conn_name.to_owned()),
                    )
                    .await?;
                }
            } else {
                need_start = true;
                println!("Creating new connection {} for {}", conn_name, device_name);
                let resp = send_command(
                    self.state.clone(),
                    NetworkCommand::CreateWiredConnection(
                        conn_name.to_owned(),
                        device_name.to_owned(),
                    ),
                )
                .await?;
                let uuid = resp
                    .into_value()
                    .map(|value| {
                        value
                            .as_str()
                            .map(|uuid| uuid.to_owned())
                            .expect("Failed to get uuid of the new connection")
                    })
                    .expect("Failed to initilise new connection");

                self.update_configuration(&device_info, &uuid).await;
            };
            self.update_managed_state(&device_info).await;
        }
        Ok(need_start)
    }
}

pub async fn initialize_network_manager(state: Arc<State>, nicrule_file: String) {
    let initializer = Initlizer::new_future(nicrule_file, state).await.unwrap();
    let need_start = initializer.init_connections().await.unwrap();
    if need_start {
        initializer.restart_networking().await.unwrap();
    }
}
