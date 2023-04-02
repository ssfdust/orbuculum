//! Tearup and teardown functions for testing rules
//!
use temp_testdir::TempDir;
use std::path::PathBuf;
use std::fs::File;
use std::io::Write;

pub fn setup_rule_files_for_nic_type_ord(tempdir: &TempDir) -> PathBuf {
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
    file_path
}

pub fn setup_rule_files_for_get_desired_devices(tempdir: &TempDir) -> PathBuf {
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
    file_path
}
