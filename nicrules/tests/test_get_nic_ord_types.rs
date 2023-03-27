use nicrules::get_nic_ord_types;
use rstest::rstest;
use serde_json::Value;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use temp_testdir::TempDir;

#[rstest]
#[case(
    r#"{"net_link_modes": ["FIBER"], "dev_path": "/pci/to/slot", "device_type": "Ethernet"}"#,
    10
)]
#[case(
    r#"{"net_link_modes": ["FIBRE"], "dev_path": "/pci/to/slot", "device_type": "Ethernet"}"#,
    10
)]
#[case(
    r#"{"net_link_modes": ["FIBRE"], "dev_path": "/platform/to/slot", "device_type": "Ethernet"}"#,
    10
)]
#[case(
    r#"{"net_link_modes": [], "dev_path": "/platform/to/slot", "device_type": "Ethernet"}"#,
    2
)]
#[case(
    r#"{"net_link_modes": [], "dev_path": "/pci/to/slot", "device_type": "Ethernet"}"#,
    3
)]
#[case(r#"{"net_link_modes": [], "dev_path": "/pci/to/slot", "device_type": "Loopback"}"#, -1)]
fn test_get_nic_ord_types(#[case] device: Value, #[case] ord_type: i32) {
    let tempdir = TempDir::default();
    let mut file_path = PathBuf::from(tempdir.as_ref());
    file_path.push("nic.rules");

    let script = r#"
        fn get_nic_type_ord(device) {
            let lower_net_link_modes = device["net_link_modes"].map(|x| x.to_lower());
            if lower_net_link_modes.contains("fibre") || lower_net_link_modes.contains("fiber") {
                return 10;
            }
            if device["dev_path"].contains("platform") {
                return 2;
            }
            if device["device_type"] == "Ethernet" {
                return 3;
            }
            return -1;
        }
    "#;
    let mut f = File::create(file_path.clone()).unwrap();
    f.write(script.as_bytes()).unwrap();
    let rule_ord_type = get_nic_ord_types(file_path.to_str().unwrap(), &device).unwrap();
    assert_eq!(ord_type, rule_ord_type);
}
