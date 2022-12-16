#![feature(try_blocks)]

#[macro_use]
extern crate eyre;

mod dispatch;

use std::sync::Arc;

use dispatch::connections::Connection;
use dispatch::devices::NetDeivce;
use dispatch::dispatch_command_requests;
use eyre::{Result, WrapErr};
use glib::{MainContext, MainLoop};
use tokio::sync::oneshot;

use dispatch::ipconfigs::IPConfig;

type TokioResponder = oneshot::Sender<Result<NetworkResponse>>;

pub struct State {
    pub glib_sender: glib::Sender<NetworkRequest>,
}

#[derive(Debug)]
pub enum NetworkCommand {
    ListDeivces,
    CreateWiredConnection(String, String),
    ListConnections,
    GetIP4Config(String),
    GetIP6Config(String),
    DeleteConnection(String),
    UpdateIP4Config(String, IPConfig),
    UpdateIP6Config(String, IPConfig),
}

pub struct NetworkRequest {
    responder: TokioResponder,
    command: NetworkCommand,
}

pub enum NetworkResponse {
    ListDeivces(Vec<NetDeivce>),
    ListConnection(Vec<Connection>),
    IP(Option<IPConfig>),
    Success,
    Failed,
}

impl NetworkRequest {
    pub fn new(responder: TokioResponder, command: NetworkCommand) -> Self {
        NetworkRequest { responder, command }
    }
}

pub async fn send_command(state: &Arc<State>, command: NetworkCommand) -> Result<NetworkResponse> {
    let (responder, receiver) = oneshot::channel();

    state
        .glib_sender
        .send(NetworkRequest::new(responder, command))
        .unwrap();

    let received = receiver
        .await
        .context("Failed to receive network thread response");

    received
        .and_then(|r| r)
        .or_else(|e| Err(e).context(format!("Execute command failed")))
}

pub fn create_channel() -> (glib::Sender<NetworkRequest>, glib::Receiver<NetworkRequest>) {
    glib::MainContext::channel(glib::PRIORITY_DEFAULT)
}

pub fn run_network_manager_loop(glib_receiver: glib::Receiver<NetworkRequest>) {
    let context = MainContext::new();
    let loop_ = MainLoop::new(Some(&context), false);

    context
        .with_thread_default(|| {
            glib_receiver.attach(None, dispatch_command_requests);

            loop_.run();
        })
        .unwrap();
}
