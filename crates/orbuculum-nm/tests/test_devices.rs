//! Device tests module
//!
//! Testing hardware devices is really a hard work. You need a virtual mathine.
//! In this project, the vagrant configuration is provided.
mod context;
mod fixture;

use fixture::start_instance;
use futures::{Future, FutureExt};
use orbuculum_nm::{send_command, NetworkCommand, State};
use rstest::rstest;
use serde_json::Value;
use std::panic;
use std::pin::Pin;
use std::sync::Arc;

#[rstest]
#[tokio::test]
/// All network devices are defined in Vagrantfile.
async fn test_list_devices(#[future] start_instance: Arc<State>) {
    context::setup_nm_test_devices_connections().await;
    let start_instance_ref = &start_instance.await;
    let async_wrapper = |start_instance_ref: Arc<State>| {
        Box::pin(async move {
            let mut eth1_exists = false;
            let mut eth2_exists = false;
            let devices =
                send_command(Arc::clone(&start_instance_ref), NetworkCommand::ListDeivces)
                    .await
                    .ok()
                    .map(|x| x.into_value().unwrap())
                    .unwrap();
            match devices {
                Value::Array(items) => {
                    for item in items {
                        if item["name"].as_str() == Some("lo") {
                            assert_eq!(item["virtual"].as_bool(), Some(true));
                        }
                        if item["name"].as_str() == Some("eth1") {
                            eth1_exists = true;
                            let ip4addrs = item["ip4info"]["addresses"]
                                .as_array()
                                .and_then(|x| {
                                    Some(
                                        x.iter()
                                            .filter_map(|x| x.as_str().map(|x| x.to_string()))
                                            .collect::<Vec<String>>(),
                                    )
                                })
                                .unwrap();
                            let conn_name = item["connection"]["id"].as_str().unwrap();
                            assert_eq!(conn_name, "test-con-2");
                            assert!(ip4addrs.contains(&"19.94.9.11/24".to_string()));
                            assert!(item["conn"].as_array().unwrap().len() == 0);
                            assert!(item["dev_path"]
                                .as_str()
                                .map(|x| x.contains("0000:01:04.0"))
                                .unwrap());
                            assert_eq!(item["virtual"].as_bool(), Some(false));
                            assert_eq!(item["mac"].as_str(), Some("52:54:5E:13:7F:43"));
                            assert_eq!(item["device_type"].as_str(), Some("Ethernet"));
                            assert_eq!(item["id_path"].as_str(), Some("pci-0000:01:04.0"));
                            assert_eq!(item["state"].as_str(), Some("Unmanaged"));
                            assert_eq!(item["driver"].as_str(), Some("virtio_net"));
                        }
                        if item["name"].as_str() == Some("eth2") {
                            eth2_exists = true;
                            let ip4addrs = item["ip6info"]["addresses"]
                                .as_array()
                                .and_then(|x| {
                                    Some(
                                        x.iter()
                                            .filter_map(|x| x.as_str().map(|x| x.to_string()))
                                            .collect::<Vec<String>>(),
                                    )
                                })
                                .unwrap();

                            assert!(item["dev_path"]
                                .as_str()
                                .map(|x| x.contains("0000:01:05.0"))
                                .unwrap());
                            assert!(ip4addrs.contains(&"fe80::5054:ff:fe70:732e/64".to_string()));
                            assert_eq!(item["virtual"].as_bool(), Some(false));

                            assert_eq!(item["mac"].as_str(), Some("52:54:5E:13:7F:44"));
                            assert_eq!(item["state"].as_str(), Some("Activated"));
                            assert_eq!(item["device_type"].as_str(), Some("Ethernet"));
                            assert_eq!(item["id_path"].as_str(), Some("pci-0000:01:05.0"));
                            assert_eq!(item["driver"].as_str(), Some("virtio_net"));
                        }
                    }
                }
                _ => assert!(false),
            }
            assert!(eth1_exists);
            assert!(eth2_exists);
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
    context::teardown_nm_test_devices_connections().await;
    assert!(result.is_ok());
}

#[rstest]
#[tokio::test]
/// To achieve the test being passed with a non-root user, you need to
/// add custom policies to polkit, then restart polkit service.
///
/// For example, file at /etc/polkit-1/rules.d/49-nm.rules
/// ```javascript
/// polkit.addRule(function(action, subject) {
///     if (action.id === "org.freedesktop.NetworkManager.network-control" &&
///         subject.user === "vagrant")
///     {
///         return polkit.Result.YES;
///     }
/// });
/// ```
async fn test_manage_devices(#[future] start_instance: Arc<State>) {
    let start_instance_ref = &start_instance.await;
    let async_wrapper = |start_instance_ref: Arc<State>| {
        Box::pin(async move {
            send_command(
                start_instance_ref,
                NetworkCommand::SetManage("eth3".to_string(), false),
            )
            .await
            .unwrap();
            let unmanaged_interface = context::run_shell_cmd("nmcli -t device status | awk -F: '/eth3.*unmanaged/{print $1}'").unwrap();
            assert_eq!(unmanaged_interface, "eth3");
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
    context::run_shell_cmd("nmcli device set eth3 managed on").unwrap();
    assert!(result.is_ok());
}
