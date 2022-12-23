mod fixture;

use fixture::start_instance;
use ipnet::IpNet;
use network::{send_command, IPConfig, NetworkCommand, NetworkResponse, Route, State};
use rstest::rstest;
use std::net::IpAddr;
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

#[rstest]
#[tokio::test]
async fn test_modify_ip_configuration(start_instance: &Arc<State>) {
    if let NetworkResponse::ListDeivces(devices) =
        send_command(start_instance, NetworkCommand::ListDeivces)
            .await
            .unwrap()
    {
        for device in devices {
            send_command(
                start_instance,
                NetworkCommand::CreateWiredConnection("test_conn".to_string(), device.mac),
            )
            .await
            .unwrap();
            break;
        }
    }
    let gw: IpAddr = "192.168.100.1".parse().unwrap();
    let addresses: Vec<IpNet> = vec!["192.168.100.39/24".parse().unwrap()];
    let ip_config = IPConfig {
        method: String::from("manual"),
        addresses: vec![],
        gateway: Some(gw),
        dns: vec![gw],
        routes: vec![],
    };
    send_command(
        start_instance,
        NetworkCommand::UpdateIP4Config(String::from("test_conn"), ip_config),
    )
    .await
    .unwrap();
}
