mod fixture;

use fixture::start_instance;
use orbuculum_nm::{send_command, NetworkCommand, State};
use rstest::rstest;
use std::process::Command;
use std::sync::Arc;

#[rstest]
#[tokio::test]
async fn test_list_connections(#[future] start_instance: Arc<State>) {
    let connections = send_command(
        Arc::clone(&start_instance.await),
        NetworkCommand::ListConnections,
    )
    .await
    .ok()
    .map(|x| x.into_value().unwrap())
    .unwrap();
    let mut names: Vec<String> = connections
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|conn| {
            conn.get("name")
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned)
        })
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

#[rstest]
#[tokio::test]
async fn test_create_connection(#[future] start_instance: Arc<State>) {
    let connection = send_command(
        Arc::clone(&start_instance.await),
        NetworkCommand::CreateWiredConnection(
            "my_special_connection".to_string(),
            "eth4".to_string(),
        ),
    )
    .await
    .ok()
    .map(|x| x.into_value().unwrap())
    .unwrap();
    let uuid = connection.as_str().unwrap();
    assert!(uuid.len() > 0);
    Command::new("nmcli")
        .arg("connection")
        .arg("delete")
        .arg("my_special_connection")
        .output()
        .unwrap();
}

#[rstest]
#[tokio::test]
async fn test_rename_connection(#[future] start_instance: Arc<State>) {
    let shared_state = Arc::clone(&start_instance.await);
    let shared_state_clone = Arc::clone(&shared_state);
    let shared_state_clone_1 = Arc::clone(&shared_state);
    let connection = send_command(
        shared_state_clone,
        NetworkCommand::CreateWiredConnection("my_old_connection".to_string(), "eth4".to_string()),
    )
    .await
    .ok()
    .map(|x| x.into_value().unwrap())
    .unwrap();
    let uuid = connection.as_str().unwrap();
    send_command(
        shared_state_clone_1,
        NetworkCommand::RenameConnection(uuid.to_string(), "my_new_connection".to_string()),
    )
    .await
    .unwrap();
    let nmcli_cons = Command::new("nmcli")
        .arg("-t")
        .arg("con")
        .arg("show")
        .output()
        .unwrap();
    let nmcli_stdout = String::from_utf8_lossy(&nmcli_cons.stdout);
    let some_str = format!("my_new_connection:{}", uuid);
    assert!(nmcli_stdout.contains(&some_str));
    Command::new("nmcli")
        .arg("connection")
        .arg("delete")
        .arg("my_new_connection")
        .output()
        .unwrap();
}

#[rstest]
#[tokio::test]
async fn test_get_connection(#[future] start_instance: Arc<State>) {
    let shared_state = Arc::clone(&start_instance.await);
    let shared_state_clone = Arc::clone(&shared_state);
    let shared_state_clone_1 = Arc::clone(&shared_state);
    let connection_uuid = send_command(
        shared_state,
        NetworkCommand::CreateWiredConnection(
            "my_unique_connection".to_string(),
            "eth4".to_string(),
        ),
    )
    .await
    .ok()
    .map(|x| x.into_value().unwrap())
    .unwrap();
    let connection_uuid = connection_uuid.as_str().unwrap();
    let connection = send_command(
        shared_state_clone,
        NetworkCommand::GetConnection(connection_uuid.to_string()),
    )
    .await
    .ok()
    .map(|x| x.into_value().unwrap())
    .unwrap();
    let got_uuid = connection["uuid"].as_str().unwrap();
    assert_eq!(got_uuid, connection_uuid);
    let clean_args = format!("nmcli connection delete {}", connection_uuid);
    Command::new("bash")
        .arg("-c")
        .arg(&clean_args)
        .output()
        .unwrap();
}
