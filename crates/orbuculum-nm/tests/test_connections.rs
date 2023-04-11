mod context;
mod fixture;

use fixture::start_instance;
use futures::{Future, FutureExt};
use orbuculum_nm::{send_command, NetworkCommand, State};
use rstest::rstest;
use std::panic;
use std::pin::Pin;
use std::process::Command;
use std::sync::Arc;

#[rstest]
#[tokio::test]
async fn test_list_connections(#[future] start_instance: Arc<State>) {
    let start_instance_ref = &start_instance.await;
    let async_wrapper = |start_instance_ref: Arc<State>| {
        Box::pin(async move {
            let connections = send_command(start_instance_ref, NetworkCommand::ListConnections)
                .await
                .ok()
                .map(|x| x.into_value().unwrap())
                .unwrap();
            let mut names: Vec<&str> = connections
                .as_array()
                .unwrap()
                .iter()
                .filter_map(|conn| conn.get("name").and_then(|x| x.as_str()))
                .collect();

            let nmcli_con_names =
                context::run_shell_cmd("nmcli -t connection show | awk -F: '{print $1}'").unwrap();
            let mut nmcli_con_names: Vec<&str> = nmcli_con_names.lines().collect();
            nmcli_con_names.sort();
            names.sort();
            assert_eq!(names, nmcli_con_names);
        }) as Pin<Box<dyn Future<Output = ()>>>
    };

    // Actually run the async test
    let result = async move {
        panic::AssertUnwindSafe(async_wrapper(Arc::clone(start_instance_ref)))
            .catch_unwind()
            .await
    }
    .await;

    // Test teardown
    assert!(result.is_ok());
}

#[rstest]
#[tokio::test]
async fn test_create_connection(#[future] start_instance: Arc<State>) {
    let start_instance_ref = &start_instance.await;
    let async_wrapper = |start_instance_ref: Arc<State>| {
        Box::pin(async move {
            let connection = send_command(
                start_instance_ref,
                NetworkCommand::CreateWiredConnection(
                    "my_special_connection".to_string(),
                    "eth4".to_string(),
                ),
            )
            .await
            .ok()
            .map(|x| x.into_value().unwrap())
            .unwrap();
            let uuid = connection.as_str().unwrap();
            let connection_string = format!("my_special_connection:{}", uuid);
            let output =
                context::run_shell_cmd("nmcli -t connection show | grep my_special_connection")
                    .unwrap();
            assert!(uuid.len() > 0 && output.contains(&connection_string));
        }) as Pin<Box<dyn Future<Output = ()>>>
    };

    // Actually run the async test
    let result = async move {
        panic::AssertUnwindSafe(async_wrapper(Arc::clone(start_instance_ref)))
            .catch_unwind()
            .await
    }
    .await;

    // Test teardown
    context::run_shell_cmd("nmcli connection delete my_special_connection").unwrap();
    assert!(result.is_ok());
}

#[rstest]
#[tokio::test]
async fn test_rename_connection(#[future] start_instance: Arc<State>) {
    let uuid = context::tearup_nm_old_unique_connection();

    let start_instance_ref = &start_instance.await;

    let async_wrapper = |start_instance_ref: Arc<State>| {
        Box::pin(async move {
            send_command(
                start_instance_ref,
                NetworkCommand::RenameConnection(
                    uuid.to_string(),
                    "my_unique_new_connection".to_string(),
                ),
            )
            .await
            .unwrap();
            let connection_string = format!("my_unique_new_connection:{}", uuid);
            let output =
                context::run_shell_cmd("nmcli -t connection show | grep my_unique_new_connection")
                    .unwrap();
            assert!(uuid.len() > 0 && output.contains(&connection_string));
        }) as Pin<Box<dyn Future<Output = ()>>>
    };

    // Actually run the async test
    let result = async move {
        panic::AssertUnwindSafe(async_wrapper(Arc::clone(start_instance_ref)))
            .catch_unwind()
            .await
    }
    .await;

    // Test teardown
    context::run_shell_cmd("nmcli connection delete my_unique_new_connection").unwrap();
    assert!(result.is_ok());
}

#[rstest]
#[tokio::test]
async fn test_get_connection(#[future] start_instance: Arc<State>) {
    let connection_uuid = context::tearup_nm_testable_connection();
    let start_instance_ref = &start_instance.await;
    let async_wrapper = |start_instance_ref: Arc<State>| {
        Box::pin(async move {
            let connection = send_command(
                start_instance_ref,
                NetworkCommand::GetConnection(connection_uuid.to_string()),
            )
            .await
            .ok()
            .map(|x| x.into_value().unwrap())
            .unwrap();
            let got_uuid = connection["uuid"].as_str().unwrap();
            let got_name = connection["name"].as_str().unwrap();
            assert_eq!(got_uuid, connection_uuid);
            assert_eq!(got_name, "my_old_unique_connection");
        }) as Pin<Box<dyn Future<Output = ()>>>
    };

    // Actually run the async test
    let result = async move {
        panic::AssertUnwindSafe(async_wrapper(Arc::clone(start_instance_ref)))
            .catch_unwind()
            .await
    }
    .await;

    // Test teardown
    context::run_shell_cmd("nmcli connection delete my_testable_connection").unwrap();
    assert!(result.is_ok());
}
