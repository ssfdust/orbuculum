use super::super::{DevicesReply, Network};
use eyre::Result;
use network::{send_command, NetworkCommand, State};
use serde_json::Value;
use std::sync::Arc;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct NetworkService {}

fn get_device_type_ord(device: Value) -> i32 {
    let net_link_modes: Vec<&str> = device["net_link_modes"]
        .as_array()
        .and_then(|x| Some(x.iter().filter_map(|x| x.as_str()).collect()))
        .unwrap();
    if net_link_modes.contains(&"FIBRE") {
        return 4;
    }
    if let Some(dev_path) = device["dev_path"].as_str() {
        if dev_path.contains("platform") {
            return 2;
        }
    }
    if let Some(device_type) = device["device_type"].as_str() {
        if device_type.contains("Ethernet") {
            return 3;
        }
    }
    -1
}

fn sort_devices(devices: Value) -> Value {
    devices.as_array_mut().and_then(|x| {
        x.into()
    }).unwrap()
}

#[tonic::async_trait]
impl Network for NetworkService {
    async fn list_devices(&self, request: Request<()>) -> Result<Response<DevicesReply>, Status> {
        let shared_state = request.extensions().get::<Arc<State>>().unwrap();
        let shared_state = Arc::clone(shared_state);
        let resp = send_command(shared_state, NetworkCommand::ListDeivces)
            .await
            .and_then(|x| {
                if let Some(mut devices) = x.into_value() {
                    if let Some(mut devices_arr) =  {
                        for device in devices_arr {}
                    }
                    let data = serde_json::from_value(devices).unwrap();
                    Ok(Response::new(DevicesReply {
                        code: 0,
                        msg: "".into(),
                        data,
                    }))
                } else {
                    bail!("")
                }
            })
            .unwrap();
        Ok(resp)
    }
}
