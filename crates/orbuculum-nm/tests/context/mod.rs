use std::process::Command;
use eyre::Result;

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
        run_shell_cmd(command).unwrap();
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
        run_shell_cmd(command).unwrap();
    }
}

pub async fn teardown_nm_test_devices_connections() {
    let commands = vec![
        "nmcli device set eth1 managed on",
        "nmcli connection delete test-con-1 test-con-2 test-con-3",
        "nmcli -t con s | awk -F: '{print $2}' | xargs -n1 timeout 2 nmcli con up || true"
    ];
    for command in commands {
        run_shell_cmd(command).unwrap();
    }
}

pub fn run_shell_cmd(command: &str) -> Result<String> {
    let output = Command::new("bash")
        .arg("-c")
        .arg(command)
        .output()
        .map(|x| String::from_utf8_lossy(&x.stdout).trim().to_string())?;
    Ok(output)
}

pub fn tearup_nm_old_unique_connection() -> String {
    run_shell_cmd("nmcli connection add type ethernet ifname eth8 con-name my_old_unique_connection ipv4.method disabled ipv6.method disabled").unwrap();
    run_shell_cmd("nmcli -t connection show | awk -F: '/my_old_unique_connection/ {print $2}'").unwrap()
}

pub fn tearup_nm_testable_connection() -> String {
    run_shell_cmd("nmcli connection add type ethernet ifname eth8 con-name my_testable_connection ipv4.method manual ipv4.addresses 11.22.33.44/24 ipv4.gateway 11.22.33.1 ipv4.dns 8.8.8.8 ipv6.method manual ipv6.addresses fe80::64e2:92ff:fe2e:ff23/64").unwrap();
    run_shell_cmd("nmcli -t connection show | awk -F: '/my_old_unique_connection/ {print $2}'").unwrap()
}
