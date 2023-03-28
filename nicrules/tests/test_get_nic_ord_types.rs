use nicrules::insert_nic_ord_types;
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
