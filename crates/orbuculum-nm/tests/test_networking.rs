mod context;
mod fixture;

use fixture::start_instance;
use futures::{Future, FutureExt};
use orbuculum_nm::{send_command, NetworkCommand, State};
use rstest::rstest;
use std::panic;
use std::pin::Pin;
use std::sync::Arc;

#[rstest]
#[tokio::test]
async fn test_set_networking(#[future] start_instance: Arc<State>) {
    let start_instance_ref = &start_instance.await;
    context::run_shell_cmd("nmcli networking on").unwrap();
    let async_wrapper = |start_instance_ref: Arc<State>| {
        Box::pin(async move {
            send_command(start_instance_ref, NetworkCommand::SetNetworking(false))
                .await
                .unwrap();
        let state = context::run_shell_cmd("nmcli networking").unwrap();
        assert_eq!(state, "disabled");
        }) as Pin<Box<dyn Future<Output = ()>>>
    };

    // Actually run the async test
    let result = async move {
        panic::AssertUnwindSafe(async_wrapper(Arc::clone(start_instance_ref)))
            .catch_unwind()
            .await
    }
    .await;
    context::run_shell_cmd("nmcli networking on").unwrap();

    // Test teardown
    assert!(result.is_ok());
}

#[rstest]
#[tokio::test]
async fn test_get_networking(#[future] start_instance: Arc<State>) {
    let start_instance_ref = &start_instance.await;
    let async_wrapper = |start_instance_ref: Arc<State>| {
        Box::pin(async move {
            let resp = send_command(start_instance_ref, NetworkCommand::GetNetworking)
                .await
                .unwrap();
            let value = resp.into_value().unwrap();
            let state = value["state"].as_bool().unwrap();
        let cur_state = context::run_shell_cmd("nmcli networking").unwrap();
        if cur_state == "disabled" {
            assert!(!state);
        } else {
            assert!(state);
        }
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
