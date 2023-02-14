#![feature(try_blocks)]
//! The network module is to conmuicate with the Network Manager service.
//! It provides `list`, `modify`, `delete`, `create` and `update` function 
//! to a network manager connection.
//!
//! And , you can list all the devices' information as well.
//! 
//! All commands are defined in the `NetworkCommand` enumeration.
//!
//! # How to use the library
//! ## Within glib library
//! 
//! ```rust
//! 
//! ```

#[macro_use]
extern crate eyre;

mod dispatch;
mod utils;

use std::sync::Arc;

pub use dispatch::connections::Connection;
pub use dispatch::devices::NetDevice;
use dispatch::dispatch_command_requests;
use eyre::{Result, WrapErr};
use glib::{MainContext, MainLoop};
use tokio::sync::oneshot;

pub use utils::{IPConfig, Route};

type TokioResponder = oneshot::Sender<Result<NetworkResponse>>;

/// The shared state for tokio application to conmuicate with glib maincontext.
pub struct State {
    glib_sender: glib::Sender<NetworkRequest>,
}

impl State {
    pub fn new(sender: glib::Sender<NetworkRequest>) -> Self {
        State {
            glib_sender: sender,
        }
    }
}

/// The network command list
/// provides all the command supported by the server.
#[derive(Debug)]
pub enum NetworkCommand {
    // list
    ListDeivces,
    CreateWiredConnection(String, String),
    ListConnections,
    GetIP4Config(String),
    GetIP6Config(String),
    // modify
    DeleteConnection(String),
    UpdateIP4Config(String, IPConfig),
    UpdateIP6Config(String, IPConfig),
}

pub struct NetworkRequest {
    responder: TokioResponder,
    command: NetworkCommand,
}

impl NetworkRequest {
    pub fn new(responder: TokioResponder, command: NetworkCommand) -> Self {
        NetworkRequest { responder, command }
    }
}

/// The network response list
/// provides all the responses supported by the server.
pub enum NetworkResponse {
    ListDeivces(Vec<NetDevice>),
    ListConnection(Vec<Connection>),
    IP(Option<IPConfig>),
    Success,
    Failed,
}


pub async fn send_command(state: Arc<State>, command: NetworkCommand) -> Result<NetworkResponse> {
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

/// The glib channel
pub fn create_channel() -> (glib::Sender<NetworkRequest>, glib::Receiver<NetworkRequest>) {
    glib::MainContext::channel(glib::PRIORITY_DEFAULT)
}

/// the main loop in glibc.
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
