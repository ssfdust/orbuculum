//! Device tests module
//!
//! Testing hardware devices is really a hard work. You need a virtual mathine.
//! In this project, the vagrant configuration is provided.
mod fixture;

use fixture::start_instance;
use network::{send_command, NetworkCommand, State};
use rstest::rstest;
use serde_json::Value;
use std::sync::Arc;

#[rstest]
#[tokio::test]
/// All network devices are defined in Vagrantfile.
async fn test_list_devices(#[future] start_instance: Arc<State>) {
    let mut enp1s4_exists = false;
    let mut enp1s5_exists = false;

    let devices = send_command(Arc::clone(&start_instance.await), NetworkCommand::ListDeivces)
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
                if item["name"].as_str() == Some("enp1s4") {
                    enp1s4_exists = true;
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
                    assert!(ip4addrs.contains(&"19.94.9.11/24".to_string()));
                    assert!(item["conn"].as_array().unwrap().len() > 0);
                    assert!(item["dev_path"]
                        .as_str()
                        .map(|x| x.contains("0000:01:04.0"))
                        .unwrap());
                    assert_eq!(item["virtual"].as_bool(), Some(false));
                    assert_eq!(item["mac"].as_str(), Some("52:54:5E:13:7F:43"));
                    assert_eq!(item["device_type"].as_str(), Some("Ethernet"));
                    assert_eq!(item["id_path"].as_str(), Some("pci-0000:01:04.0"));
                    assert_eq!(item["state"].as_str(), Some("Activated"));
                }
                if item["name"].as_str() == Some("enp1s5") {
                    enp1s5_exists = true;
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
                }
            }
        }
        _ => assert!(false),
    }
    assert!(enp1s4_exists);
    assert!(enp1s5_exists);
}

#[rstest]
#[tokio::test]
async fn test_print_devices(#[future] start_instance: Arc<State>) {
    let devices = send_command(Arc::clone(&start_instance.await), NetworkCommand::ListDeivces)
        .await
        .ok()
        .map(|x| x.into_value().unwrap())
        .unwrap();
    println!("{}", serde_json::to_string_pretty(&devices).unwrap());
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
    let interface = "enp1s4";
    let state = Arc::clone(&start_instance.await);
    send_command(
        Arc::clone(&state),
        NetworkCommand::SetManage(interface.to_string(), false),
    )
    .await
    .unwrap();
    let devices = send_command(Arc::clone(&state), NetworkCommand::ListDeivces)
        .await
        .ok()
        .map(|x| x.into_value().unwrap())
        .unwrap();
    match devices {
        Value::Array(items) => {
            for item in items {
                if item["name"].as_str() == Some("enp1s4") {
                    assert_eq!(item["is_managed"].as_bool(), Some(false));
                }
            }
        }
        _ => assert!(false),
    }
    send_command(
        Arc::clone(&state),
        NetworkCommand::SetManage(interface.to_string(), true),
    )
    .await
    .unwrap();
}
