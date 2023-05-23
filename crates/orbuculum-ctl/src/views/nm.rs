//! The Network view
use crate::services::nm::{
    connection_json2info, device_json2info, edit_connection, get_connection, get_devices,
    restart_networking, update_connection,
};
use crate::utils::{QuestionOnce, QuestionText};
use eyre::{ContextCompat, Result};
use requestty::{prompt_one, question, Question};
use std::sync::Arc;

pub async fn draw_nm_ui(grpc_addr: Arc<&str>) -> Result<()> {
    let devices = get_devices(grpc_addr.clone()).await?;
    let devices_info: Vec<String> = devices
        .iter()
        .filter_map(|device| device_json2info(device).ok())
        .collect();
    let choices = devices_info.into_iter().map(|x| x.into()).collect();
    let once_question = QuestionText::new(
        "Network",
        "Please select netowrk device:",
        &choices,
        &devices,
    );
    let device = once_question.execute()?;

    let device_uuid = device["connection"]["uuid"]
        .as_str()
        .wrap_err("The connection doesn't exist.")?;
    let connection = get_connection(grpc_addr.clone(), device_uuid.to_string()).await?;
    let connection_string = connection_json2info(&connection)?;
    println!("{}", connection_string);
    let question = Question::confirm("edit")
        .message("Do you want to edit the connection?")
        .build();
    let answer = prompt_one(question)?;

    let whether_edit = answer.as_bool().unwrap_or_default();
    if whether_edit {
        let selections = vec!["IPv4".to_string(), "IPv6".to_string()];
        let once_question = QuestionText::new(
            "ipversion",
            "Please select netowrk IP version:",
            &selections,
            &selections,
        );
        let ipversion = once_question.execute()?;
        let selections = vec!["DHCP".to_string(), "Manual".to_string()];
        let once_question = QuestionText::new(
            "ipmethod",
            "Please select netowrk IP Method:",
            &selections,
            &selections,
        );
        let ipmethod = once_question.execute()?;
        let new_connection = edit_connection(ipmethod, ipversion, &connection, ask_for_connection);
        match update_connection(grpc_addr.clone(), &new_connection).await {
            Ok(()) => {
                println!("Connection updated");
                ask_for_restart(grpc_addr.clone()).await?;
            }
            _ => println!("Connection updated failed."),
        }
    }

    Ok(())
}

async fn ask_for_restart(grpc_addr: Arc<&str>) -> Result<()> {
    let question = Question::confirm("restart")
        .message("Do you want to restart network now?")
        .build();
    let answer = prompt_one(question)?;
    let if_restart = answer.as_bool().unwrap_or(false);
    if if_restart {
        restart_networking(grpc_addr).await?;
    }
    Ok(())
}

fn ask_for_connection(version: &str) -> (Vec<String>, String, Vec<String>) {
    let ip_msg = format!("Please enter {} addresses/prefix :", version);
    let gw_msg = format!("Please enter {} gateway:", version);
    let dns_msg = format!("Please enter {} dns:", version);
    let questions = vec![
        Question::input("addresses").message(ip_msg).build(),
        Question::input("gateway").message(gw_msg).build(),
        Question::input("dns").message(dns_msg).build(),
    ];
    let answers = requestty::prompt(questions).unwrap();
    let mut addresses: Vec<String> = vec![];
    let mut gateway = None;
    let mut dns: Vec<String> = vec![];
    for (k, v) in answers {
        match k {
            k if k == "addresses" => {
                addresses = v
                    .as_string()
                    .map(|x| x.split(",").into_iter().map(|x| x.to_string()).collect())
                    .unwrap_or_default()
            }
            k if k == "gateway" => gateway = v.as_string().map(|x| x.to_string()),
            k if k == "dns" => {
                dns = v
                    .as_string()
                    .map(|x| x.split(",").into_iter().map(|x| x.to_string()).collect())
                    .unwrap_or_default()
            }
            _ => (),
        }
    }
    (addresses, gateway.unwrap_or_default(), dns)
}
