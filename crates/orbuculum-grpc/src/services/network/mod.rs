use crate::network_grpc::ConnectionsReply;

use super::super::{ConnectionReply, ConnectionUuidRequest, DevicesReply, Network};
use eyre::Result;
use orbuculum_nm::{send_command, NetworkCommand, State};

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
        let resp = send_command(
            shared_state,
            NetworkCommand::GetConnection(uuid.clone()),
        )
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
}
