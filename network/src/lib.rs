#[macro_use]
extern crate eyre;

mod dispatch;

use std::sync::Arc;
use std::{fmt::Display, net::IpAddr};

use eyre::{Result, WrapErr};
use glib::{MainLoop, MainContext};
use tokio::sync::oneshot;
use dispatch::dispatch_command_requests;
use dispatch::devices::NetDeivce;
use dispatch::connections::Connection;

type TokioResponder = oneshot::Sender<Result<NetworkResponse>>;

pub struct State {
    pub glib_sender: glib::Sender<NetworkRequest>,
}

#[derive(Debug)]
pub enum NetworkCommand {
    ListDeivces,
    CreateWiredConnection(String, String),
    ListConnections,
    DeleteConnection(String)
}

pub struct NetworkRequest {
    responder: TokioResponder,
    command: NetworkCommand,
}

pub enum NetworkResponse {
    ListDeivces(Vec<NetDeivce>),
    ListConnection(Vec<Connection>),
    Success,
    Failed
}

impl NetworkRequest {
    pub fn new(responder: TokioResponder, command: NetworkCommand) -> Self {
        NetworkRequest { responder, command }
    }
}

/// The Ip configuration struct
#[derive(Debug)]
pub struct IPConfig {
    address: IpAddr,
    gateway: Option<IpAddr>,
    dns: Option<IpAddr>,
    prefix: u32,
}

impl Display for IPConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "address: {}/{}\ngateway: {}\ndns: {}",
            self.address,
            self.prefix,
            self.gateway.map_or(String::new(), |x| x.to_string()),
            self.dns.map_or(String::new(), |x| x.to_string())
        )
    }
}

/// A simple Network Config consists of connection name,
/// IpV4 configuration and IpV6 configuration.
#[derive(Debug)]
pub struct NetworkConfig {
    /// The connection name
    name: String,
    /// The IpV4 Config of the connection
    ipv4cfg: Option<IPConfig>,
    /// The IpV6 Config of the connection
    ipv6cfg: Option<IPConfig>,
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

    context.with_thread_default(|| {
        glib_receiver.attach(None, dispatch_command_requests);

        loop_.run();
    }).unwrap();
}
