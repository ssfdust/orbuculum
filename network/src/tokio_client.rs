//! ### Tokio Interaction Module
//!
//! The `Tokio` module used to provide
use super::{NetworkCommand, NetworkResponse};
use crate::dispatch::dispatch_command_requests;
use eyre::{Result, WrapErr};
use glib::{MainContext, MainLoop};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::oneshot;

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

pub type TokioResponder = oneshot::Sender<Result<NetworkResponse>>;

pub struct NetworkRequest {
    pub responder: TokioResponder,
    pub command: NetworkCommand,
}

impl NetworkRequest {
    pub fn new(responder: TokioResponder, command: NetworkCommand) -> Self {
        NetworkRequest { responder, command }
    }
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
pub fn run_network_manager_loop(
    glib_receiver: glib::Receiver<NetworkRequest>,
    link_modes: Arc<Value>,
) {
    let context = MainContext::new();
    let loop_ = MainLoop::new(Some(&context), false);

    context
        .with_thread_default(|| {
            glib_receiver.attach(None, move |request| {
                let link_modes_cloned = Arc::clone(&link_modes);
                dispatch_command_requests(request, link_modes_cloned)
            });

            loop_.run();
        })
        .unwrap();
}
