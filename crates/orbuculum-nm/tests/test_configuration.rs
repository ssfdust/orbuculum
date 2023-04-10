mod context;
mod fixture;

use fixture::start_instance;
use futures::{Future, FutureExt};
use orbuculum_nm::{send_command, NetworkCommand, State};
use rstest::rstest;
use serde_json::json;
use std::panic;
use std::pin::Pin;
use std::sync::Arc;

#[rstest]
#[tokio::test]
async fn test_modify_ip_configuration(#[future] start_instance: Arc<State>) {
    let start_instance_ref = &start_instance.await;
    let uuid = context::tearup_nm_modifiable_connection();
    let async_wrapper = |start_instance_ref: Arc<State>| {
        Box::pin(async move {
            let json_data = json!({
                "name": "my_modifiable_connection",
                "uuid": uuid,
                "ip4info": {
                    "addresses": ["192.168.100.1/24"],
                    "gateway": null,
                    "method": "manual",
                    "dns": []
                },
                "ip6info": {
                    "addresses": [],
                    "gateway": null,
                    "method": "disabled",
                    "dns": []
                }
            });
            send_command(
                start_instance_ref,
                NetworkCommand::UpdateConnection(json_data),
            )
            .await
            .unwrap();
            let ipv4_addrs = context::run_shell_cmd("nmcli -t connection show my_modifiable_connection | awk -F: '/ipv4.addresses/ {print $2}'").unwrap();
            let ipv6_addrs = context::run_shell_cmd("nmcli -t connection show my_modifiable_connection | awk -F: '/ipv6.method/ {print $2}'").unwrap();
            assert_eq!(ipv4_addrs, "192.168.100.1/24");
            assert_eq!(ipv6_addrs, "disabled");
        }) as Pin<Box<dyn Future<Output = ()>>>
    };

    // Actually run the async test
    let result = async move {
        panic::AssertUnwindSafe(async_wrapper(Arc::clone(start_instance_ref)))
            .catch_unwind()
            .await
    }
    .await;
    context::teardown_nm_modifiable_connection();

    // Test teardown
    assert!(result.is_ok());
}
