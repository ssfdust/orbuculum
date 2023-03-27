mod fixture;

use fixture::start_instance;
use network::{send_command, NetworkCommand, NetworkResponse, State};
use rstest::rstest;
use std::process::Command;
use std::sync::Arc;

#[rstest]
#[tokio::test]
async fn test_list_connections(#[future] start_instance: Arc<State>) {
    let mut cnt = 0;
    let connections = send_command(
        Arc::clone(&start_instance.await),
        NetworkCommand::ListConnections,
    )
    .await
    .ok()
    .map(|x| x.into_value().unwrap())
    .unwrap();
    let result = Command::new("nmcli")
        .arg("-t")
        .arg("con")
        .arg("show")
        .output()
        .unwrap();
    let mut names: Vec<String> = connections
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|conn| conn.get("name").and_then(serde_json::Value::as_str).map(str::to_owned))
        .collect();

    let nmcli_cons = Command::new("nmcli")
        .arg("-t")
        .arg("con")
        .arg("show")
        .output()
        .unwrap();
    let nmcli_stdout = String::from_utf8_lossy(&nmcli_cons.stdout); //将输出转换为utf8字符串
    let mut nmcli_con_names: Vec<String> = Vec::new();

    for line in nmcli_stdout.lines() {
        let first_colon_idx = match line.find(':') {
            Some(idx) => idx,
            None => continue,
        };
        nmcli_con_names.push(line[0..first_colon_idx].to_owned());
    }
    nmcli_con_names.sort();
    names.sort();
    assert_eq!(names, nmcli_con_names);
}
