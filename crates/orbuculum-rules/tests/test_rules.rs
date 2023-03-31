mod context;

use nicrules::{insert_nic_ord_types, get_desired_devices};
use rstest::rstest;
use serde_json::Value;
use std::panic;
use temp_testdir::TempDir;

#[rstest]
#[case(
    r#"[{"net_link_modes": ["FIBER"], "dev_path": "/pci/to/slot", "device_type": "Ethernet"}]"#,
    10
)]
#[case(
    r#"[{"net_link_modes": ["FIBRE"], "dev_path": "/pci/to/slot", "device_type": "Ethernet"}]"#,
    10
)]
#[case(
    r#"[{"net_link_modes": ["FIBRE"], "dev_path": "/platform/to/slot", "device_type": "Ethernet"}]"#,
    10
)]
#[case(
    r#"[{"net_link_modes": [], "dev_path": "/platform/to/slot", "device_type": "Ethernet"}]"#,
    2
)]
#[case(
    r#"[{"net_link_modes": [], "dev_path": "/pci/to/slot", "device_type": "Ethernet"}]"#,
    3
)]
#[case(r#"[{"net_link_modes": [], "dev_path": "/pci/to/slot", "device_type": "Loopback"}]"#, -1)]
fn test_insert_nic_ord_types(#[case] device: Value, #[case] ord_type: i32) {
    // Tear up actions
    let tempdir = TempDir::default();
    let file_path = context::setup_rule_files_for_nic_type_ord(&tempdir);

    // actually run the test
    let result = panic::catch_unwind(|| {
        let device_with_type_ord = insert_nic_ord_types(file_path.to_str().unwrap(), &device)
            .map(|x| x.into_iter().next().unwrap())
            .unwrap();
        let rule_ord_type = device_with_type_ord["type_ord"]
            .as_i64()
            .map(|x| x as i32)
            .unwrap();
        assert_eq!(ord_type, rule_ord_type);
    });
    assert!(result.is_ok());
}

#[rstest]
#[case(
    r#"
    [
        {
            "net_link_modes": ["FIBER"],
            "dev_path": "/pci/to/slot",
            "device_type": "Ethernet",
            "name": "enp1s0",
            "id_path": "2"
        },
        {
            "net_link_modes": [],
            "dev_path": "/pci/to/slot",
            "id_path": "1",
            "device_type": "Ethernet",
            "name": "enp2s0"
        },
        {
            "net_link_modes": [],
            "dev_path": "/pci/to/slot",
            "device_type": "Wireless",
            "id_path": "3",
            "name": "wlp3s0"
        }
    ]"#,
    r#"
    [
        {
            "name": "enp2s0",
            "con_name": "eth0"
        },
        {
            "name": "enp1s0",
            "con_name": "eth1"
        },
        {
            "name": "wlp3s0",
            "con_name": "wifi0"
        }
    ]
    "#
)]
fn test_get_desired_devices(#[case] device: Value, #[case] results: Value) {
    // Tear up actions
    let tempdir = TempDir::default();
    let file_path = context::setup_rule_files_for_get_desired_devices(&tempdir);

    let result = panic::catch_unwind(|| {
        let devices_info = get_desired_devices(file_path.to_str().unwrap(), &device).unwrap();
        let results_arr = results.as_array().unwrap();
        for device_info in devices_info {
            let device_name = device_info["name"].as_str().unwrap();
            let conn_name = device_info["con_name"].as_str().unwrap();
            let mut found = false;
            for result in results_arr {
                let result_device_name = result["name"].as_str().unwrap();
                let result_conn_name = result["con_name"].as_str().unwrap();
                if device_name == result_device_name {
                    found = true;
                    assert_eq!(device_name, result_device_name);
                    assert_eq!(conn_name, result_conn_name);
                    break;
                }
            }
            assert!(found);
        }
    });
    assert!(result.is_ok());
}
