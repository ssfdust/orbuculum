mod fixture;

use fixture::start_instance;
use network::{send_command, NetworkCommand, NetworkResponse, State};
use rstest::rstest;
use std::sync::Arc;

#[rstest]
#[tokio::test]
async fn test_list_devices(start_instance: &Arc<State>) {
    if let NetworkResponse::ListDeivces(devices) =
        send_command(Arc::clone(start_instance), NetworkCommand::ListDeivces)
            .await
            .unwrap()
    {
        for device in devices {
            println!("{:?}", device);
        }
    }
}

/// Try to create a connection
#[rstest]
#[tokio::test]
async fn test_connections(start_instance: &Arc<State>) {
    let mut cnt = 0;
    if let NetworkResponse::ListDeivces(devices) =
        send_command(Arc::clone(start_instance), NetworkCommand::ListDeivces)
            .await
            .unwrap()
    {
        for device in devices {
            send_command(
                Arc::clone(start_instance),
                NetworkCommand::CreateWiredConnection("test_conn".to_string(), device.mac),
            )
            .await
            .unwrap();
            break;
        }
    }
    if let NetworkResponse::ListConnection(conns) =
        send_command(Arc::clone(start_instance), NetworkCommand::ListConnections)
            .await
            .unwrap()
    {
        for conn in conns {
            if conn.name.contains("test_conn") {
                cnt += 1;
                send_command(Arc::clone(start_instance), NetworkCommand::DeleteConnection(conn.name))
                    .await
                    .unwrap();
            }
        }
    }
    assert_eq!(cnt, 1);
}
