use crate::network_grpc::{ConnectionsReply, HostnameReply, HostnameBody};

use super::super::{ConnectionBody, ConnectionReply, ConnectionUuidRequest, DevicesReply, Network};
use eyre::{Result, ContextCompat};
use orbuculum_nm::{send_command, NetworkCommand, State};
use serde_json::json;

use std::sync::Arc;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct NetworkService {}

#[tonic::async_trait]
impl Network for NetworkService {
    async fn list_devices(&self, request: Request<()>) -> Result<Response<DevicesReply>, Status> {
        let shared_state = request.extensions().get::<Arc<State>>().unwrap();
        let shared_state = Arc::clone(shared_state);
        let resp = send_command(shared_state, NetworkCommand::ListDeivces)
            .await
            .and_then(|x| {
                if let Some(devices) = x.into_value() {
                    let data = serde_json::from_value(devices).unwrap();
                    Ok(Response::new(DevicesReply {
                        code: 0,
                        msg: "Sucessful".into(),
                        data,
                    }))
                } else {
                    bail!("")
                }
            })
            .unwrap();
        Ok(resp)
    }

    async fn get_connection_by_uuid(
        &self,
        request: Request<ConnectionUuidRequest>,
    ) -> Result<Response<ConnectionReply>, Status> {
        let shared_state = request.extensions().get::<Arc<State>>().unwrap();
        let shared_state = Arc::clone(shared_state);
        let uuid = request.into_inner().uuid;
        let resp = send_command(shared_state, NetworkCommand::GetConnection(uuid.clone()))
            .await
            .and_then(|x| {
                if let Some(connection) = x.into_value() {
                    let data = serde_json::from_value(connection).unwrap();
                    Ok(Response::new(ConnectionReply {
                        code: 0,
                        msg: "Sucessful".into(),
                        data,
                    }))
                } else {
                    bail!("Failed to get connection with uuid {}", uuid)
                }
            })
            .unwrap();
        Ok(resp)
    }

    async fn reactive_connection(
        &self,
        request: Request<ConnectionUuidRequest>,
    ) -> Result<Response<ConnectionReply>, Status> {
        let shared_state = request.extensions().get::<Arc<State>>().unwrap();
        let shared_state = Arc::clone(shared_state);
        let uuid = request.into_inner().uuid;
        send_command(shared_state.clone(), NetworkCommand::Reactive(uuid.clone())).await.unwrap();
        let resp = send_command(shared_state, NetworkCommand::GetConnection(uuid.clone()))
            .await
            .and_then(|x| {
                if let Some(connection) = x.into_value() {
                    let data = serde_json::from_value(connection).unwrap();
                    Ok(Response::new(ConnectionReply {
                        code: 0,
                        msg: "Sucessful".into(),
                        data,
                    }))
                } else {
                    bail!("Failed to get connection with uuid {}", uuid)
                }
            })
            .unwrap();
        Ok(resp)
    }

    async fn list_connections(
        &self,
        request: Request<()>,
    ) -> Result<Response<ConnectionsReply>, Status> {
        let shared_state = request.extensions().get::<Arc<State>>().unwrap();
        let shared_state = Arc::clone(shared_state);
        let resp = send_command(shared_state, NetworkCommand::ListConnections)
            .await
            .and_then(|x| {
                if let Some(connections) = x.into_value() {
                    let data = serde_json::from_value(connections).unwrap();
                    Ok(Response::new(ConnectionsReply {
                        code: 0,
                        msg: "Sucessful".into(),
                        data,
                    }))
                } else {
                    bail!("")
                }
            })
            .unwrap();
        Ok(resp)
    }

    async fn update_connection(
        &self,
        request: Request<ConnectionBody>,
    ) -> Result<Response<ConnectionReply>, Status> {
        let shared_state = request.extensions().get::<Arc<State>>().unwrap();
        let shared_state = Arc::clone(shared_state);
        match serde_json::to_value(request.into_inner()) {
            Ok(connection) => {
                let resp = send_command(shared_state, NetworkCommand::UpdateConnection(connection))
                    .await
                    .and_then(|x| {
                        if let Some(connection) = x.into_value() {
                            let data = serde_json::from_value(connection).unwrap();
                            Ok(Response::new(ConnectionReply {
                                code: 0,
                                msg: "Sucessful".into(),
                                data,
                            }))
                        } else {
                            bail!("Failed to update connection")
                        }
                    })
                    .unwrap();
                Ok(resp)
            }
            _ => Err(Status::invalid_argument("Failed to parse request data")),
        }
    }

    async fn get_hostname(
        &self,
        request: Request<()>,
    ) -> Result<Response<HostnameReply>, Status> {
        let shared_state = request.extensions().get::<Arc<State>>().unwrap();
        let shared_state = Arc::clone(shared_state);
        let resp = send_command(shared_state, NetworkCommand::GetHostname)
            .await
            .and_then(|x| {
                if let Some(hostname) = x.into_value() {
                    let hostname = hostname.as_str().map(|x| x.to_string()).wrap_err("Failed to parse hostname from request")?;
                    let data = serde_json::from_value(json!({
                        "hostname": hostname
                    }))?;
                    Ok(Response::new(HostnameReply {
                        code: 0,
                        msg: "Sucessful".into(),
                        data,
                    }))
                } else {
                    bail!("Failed to get hostname")
                }
            })
            .unwrap();
        Ok(resp)
    }

    async fn set_hostname(
        &self,
        request: Request<HostnameBody>,
    ) -> Result<Response<HostnameReply>, Status> {
        let shared_state = request.extensions().get::<Arc<State>>().unwrap();
        let shared_state = Arc::clone(shared_state);
        match serde_json::to_value(request.into_inner()) {
            Ok(hostname) => {
                let hostname = hostname["hostname"].as_str().map(|x| x.to_string()).unwrap();
                let hostname_clone = hostname.clone();
                let resp = send_command(shared_state, NetworkCommand::SetHostname(hostname))
                    .await
                    .and_then(|_| {
                            let data = serde_json::from_value(json!({
                                "hostname": hostname_clone
                            }))?;
                            Ok(Response::new(HostnameReply {
                                code: 0,
                                msg: "Sucessful".into(),
                                data,
                            }))
                    })
                    .unwrap();
                Ok(resp)
            }
            _ => Err(Status::invalid_argument("Failed to parse request data")),
        }
    }
}
