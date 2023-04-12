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
async fn test_hostname(#[future] start_instance: Arc<State>) {
    let start_instance_ref = &start_instance.await;
    let async_wrapper = |start_instance_ref: Arc<State>| {
        Box::pin(async move {
            let cloned_state = start_instance_ref.clone();
            let resp = send_command(start_instance_ref, NetworkCommand::GetHostname)
                .await
                .unwrap();
            let ret = resp.into_value().unwrap();
            let hostname = ret.as_str().unwrap();
            assert_eq!(hostname, "rocky9");

            send_command(
                cloned_state,
                NetworkCommand::SetHostname("myhostname".to_string()),
            )
            .await
            .unwrap();
            let new_hostname = context::run_shell_cmd("hostname").unwrap();
            assert_eq!(new_hostname, "myhostname");
        }) as Pin<Box<dyn Future<Output = ()>>>
    };

    // Actually run the async test
    let result = async move {
        panic::AssertUnwindSafe(async_wrapper(Arc::clone(start_instance_ref)))
            .catch_unwind()
            .await
    }
    .await;
    context::run_shell_cmd("sudo hostnamectl set-hostname rocky9").unwrap();

    // Test teardown
    assert!(result.is_ok());
}
