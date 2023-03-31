use std::process::Command;

/// The connection must be shown even if the connection is down
pub async fn setup_nm_test_devices_connections() {
    let commands = vec![
        "nmcli connection add type ethernet ifname eth1 con-name test-con-1 ipv4.method disabled ipv6.method disabled",
        "nmcli connection add type ethernet ifname eth1 con-name test-con-2 ipv4.method manual ipv4.addresses 19.94.9.11/24 ipv6.method disabled",
        "nmcli connection add type ethernet ifname eth2 con-name test-con-3 ipv4.method disabled ipv6.method manual ipv6.addresses fe80::5054:ff:fe70:732e/64",
        "nmcli connection up test-con-1",
        "nmcli connection down test-con-1",
        "nmcli connection up test-con-2",
        "nmcli connection down test-con-2",
        "while :; do nmcli -t con show --active | grep -q eth1 && nmcli connection down \"$(nmcli -t dev | awk -F: '/eth1/{print $4}')\" || break; done",
    ];
    for command in commands {
        Command::new("bash")
            .arg("-c")
            .arg(command)
            .output()
            .unwrap();
    }
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    let commands = vec![
        "nmcli connection up test-con-2",
        "nmcli connection down test-con-2",
        "nmcli connection up test-con-3",
        "nmcli device set eth1 managed off",
        "sudo ip addr add 19.94.9.11/24 dev eth1",
    ];
    for command in commands {
        Command::new("bash")
            .arg("-c")
            .arg(command)
            .output()
            .unwrap();
    }
}

pub async fn teardown_nm_test_devices_connections() {
    let commands = vec![
        "nmcli device set eth1 managed on",
        "nmcli connection delete test-con-1 test-con-2 test-con-3",
        "nmcli -t con s | awk -F: '{print $2}' | xargs -n1 timeout 2 nmcli con up || true"
    ];
    for command in commands {
        Command::new("bash")
            .arg("-c")
            .arg(command)
            .output()
            .unwrap();
    }
}
