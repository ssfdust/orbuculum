mod fixture;

use fixture::start_instance;
use network::{send_command, NetworkCommand, State};
use rstest::rstest;
use std::sync::Arc;
use serde_json::Value;

#[rstest]
#[tokio::test]
async fn test_list_devices(start_instance: &Arc<State>) {
    let mut enp1s4_exists = false;
    let mut enp1s5_exists = false;

    let devices = send_command(Arc::clone(start_instance), NetworkCommand::ListDeivces)
        .await
        .ok()
        .map(|x| x.into_value().unwrap()).unwrap();
    match devices {
        Value::Array(items) => {
            for item in items {
                if item["name"].as_str() == Some("lo") {
                    assert_eq!(item["virtual"].as_bool(), Some(true));
                }
                if item["name"].as_str() == Some("enp1s4") {
                    enp1s4_exists = true;
                    let ip4addrs = item["ip4info"]["addresses"].as_array().and_then(|x| {
                        Some(x.iter().filter_map(|x| x.as_str().map(|x| x.to_string())).collect::<Vec<String>>())
                    }).unwrap();
                    assert!(ip4addrs.contains(&"19.94.9.11/24".to_string()));
                    assert!(item["conn"].as_array().unwrap().len() > 0);
                    assert!(item["dev_path"].as_str().map(|x| x.contains("0000:01:04.0")).unwrap());
                    assert_eq!(item["mac"].as_str(), Some("52:54:5E:13:7F:43"));
                    assert_eq!(item["device_type"].as_str(), Some("Ethernet"));
                    assert_eq!(item["id_path"].as_str(), Some("pci-0000:01:04.0"));
                    assert_eq!(item["state"].as_str(), Some("Activated"));
                }
                if item["name"].as_str() == Some("enp1s5") {
                    enp1s5_exists = true;
                    let ip4addrs = item["ip6info"]["addresses"].as_array().and_then(|x| {
                        Some(x.iter().filter_map(|x| x.as_str().map(|x| x.to_string())).collect::<Vec<String>>())
                    }).unwrap();

                    assert!(item["dev_path"].as_str().map(|x| x.contains("0000:01:05.0")).unwrap());
                    assert!(ip4addrs.contains(&"fe80::5054:ff:fe70:732e/64".to_string()));

                    assert_eq!(item["mac"].as_str(), Some("52:54:5E:13:7F:44"));
                    assert_eq!(item["state"].as_str(), Some("Activated"));
                    assert_eq!(item["device_type"].as_str(), Some("Ethernet"));
                    assert_eq!(item["id_path"].as_str(), Some("pci-0000:01:05.0"));
                }
            }
        },
        _ => assert!(false)
    }
    assert!(enp1s4_exists);
    assert!(enp1s5_exists);
}
