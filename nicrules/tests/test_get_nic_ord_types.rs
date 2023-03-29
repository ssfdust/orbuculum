use nicrules::{insert_nic_ord_types, get_desired_devices};
use rstest::rstest;
use serde_json::Value;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
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
    let tempdir = TempDir::default();
    let mut file_path = PathBuf::from(tempdir.as_ref());
    file_path.push("nic.rules");

    let script = r#"
        fn insert_nic_type_ord(device) {
            let lower_net_link_modes = device["net_link_modes"].map(|x| x.to_lower());
            let type_ord = -1;
            if lower_net_link_modes.contains("fibre") || lower_net_link_modes.contains("fiber") {
                 type_ord = 10;
            }
            else if device["dev_path"].contains("platform") {
                type_ord = 2;
            }
            else if device["device_type"] == "Ethernet" {
                type_ord = 3;
            }
            device["type_ord"] = type_ord;
            return device;
        }
    "#;
    let mut f = File::create(file_path.clone()).unwrap();
    f.write(script.as_bytes()).unwrap();
    let device_with_type_ord = insert_nic_ord_types(file_path.to_str().unwrap(), &device)
        .map(|x| x.into_iter().next().unwrap())
        .unwrap();
    let rule_ord_type = device_with_type_ord["type_ord"]
        .as_i64()
        .map(|x| x as i32)
        .unwrap();
    assert_eq!(ord_type, rule_ord_type);
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
    let tempdir = TempDir::default();
    let mut file_path = PathBuf::from(tempdir.as_ref());
    file_path.push("nic.rules");

    let script = r#"
        fn insert_nic_type_ord(device) {
            let lower_net_link_modes = device["net_link_modes"].map(|x| x.to_lower());
            let type_ord = -1;
            if lower_net_link_modes.contains("fibre") || lower_net_link_modes.contains("fiber") {
                 type_ord = 10;
            }
            else if device["dev_path"].contains("platform") {
                type_ord = 2;
            }
            else if device["device_type"] == "Ethernet" {
                type_ord = 3;
            }
            device["type_ord"] = type_ord;
            return device;
        }
        fn modify_connections(devices, device_type) {
            let new_devices = [];
            if device_type == "Ethernet" {
                for (device, idx) in devices {
                    device["con_name"] = "eth" + idx;
                    new_devices.push(device);
                }
            }
            if device_type == "Wireless" {
                for (device, idx) in devices {
                    device["con_name"] = "wifi" + idx;
                    new_devices.push(device);
                }
            }
            return new_devices;
        }
    "#;
    let mut f = File::create(file_path.clone()).unwrap();
    f.write(script.as_bytes()).unwrap();
    let devices_info = get_desired_devices(file_path.to_str().unwrap(), &device).unwrap();
    let results_arr = results.as_array().unwrap();
    for device_info in devices_info {
        println!("Device Info: {}", serde_json::to_string_pretty(&device_info).unwrap());
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
}
