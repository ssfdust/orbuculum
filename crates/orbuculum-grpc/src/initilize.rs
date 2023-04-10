use eyre::Result;
use orbuculum_nm::{send_command, NetworkCommand, State};
use orbuculum_rules::get_desired_devices;
use serde_json::Value;
use std::sync::Arc;

struct Initlizer {
    nicrule_file: String,
    devices: Vec<Value>,
    state: Arc<State>
}

impl Initlizer {
    async fn new_future(nicrule_file: String, state: Arc<State>) -> Result<Self> {
        let cloned_state = state.clone();
        let opt_devices = send_command(cloned_state, NetworkCommand::ListDeivces)
            .await
            .and_then(|resp| Ok(resp.into_value().and_then(|value| value.as_array().and_then(|items| {
                let items: Vec<Value> = items.iter().map(|x| x.clone()).collect();
                Some(items)
            }))))?;
        let devices = opt_devices.unwrap_or_default();
        Ok(Self {
            nicrule_file,
            devices,
            state
        })
    }

    async fn init_connections(&self) -> Result<()>{
        let devices_val = serde_json::to_value(&self.devices)?;
        let sorted_devices = get_desired_devices(&self.nicrule_file, &devices_val).expect("Fail to get devices info");
        for device_info in sorted_devices {
            let device_name = device_info["name"].as_str().expect("Failed to get name when initilising");
            let conn_name = device_info["con_name"].as_str().expect("Failed to get con_name when initilising");
            let current_connection = device_info["connection"].as_object().expect("Failed to get connection when initilising");
            let current_uuid = current_connection.get("uuid").expect("no uuid in connection object");
            if let Some(uuid) = current_uuid.as_str() {
                send_command(self.state.clone(), NetworkCommand::RenameConnection(uuid.to_owned(), conn_name.to_owned())).await?;
            } else {
                send_command(self.state.clone(), NetworkCommand::CreateWiredConnection(conn_name.to_owned(), device_name.to_owned())).await?;
            }
        }
        Ok(())
    }
}

pub async fn initilize_network_manager(state: Arc<State>, nicrule_file: String) {
    let initilizer = Initlizer::new_future(nicrule_file, state).await.unwrap();
    initilizer.init_connections().await.unwrap();
}
