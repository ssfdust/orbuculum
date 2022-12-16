mod fixture;

use fixture::start_instance;
use network::{send_command, NetworkCommand, NetworkResponse, State};
use rstest::rstest;
use std::sync::Arc;

#[rstest]
#[tokio::test]
async fn test_ip_configuration(start_instance: &Arc<State>) {
    if let NetworkResponse::IP(Some(ipconfig)) = send_command(
        start_instance,
        NetworkCommand::GetIP4Config("virtbr0".to_string()),
    )
    .await
    .unwrap()
    {
        println!("{:?}", ipconfig);
    }
    if let NetworkResponse::IP(Some(ipconfig)) = send_command(
        start_instance,
        NetworkCommand::GetIP6Config("virtbr0".to_string()),
    )
    .await
    .unwrap()
    {
        println!("{:?}", ipconfig);
    }
}
