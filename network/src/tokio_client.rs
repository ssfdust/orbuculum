//! ### Tokio Interaction Module
//!
//! The `Tokio` module used to provide
use crate::{NetworkCommand, NetworkRequest, NetworkResponse, State};
use eyre::{Result, WrapErr};
use std::sync::Arc;
use tokio::sync::oneshot;

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
