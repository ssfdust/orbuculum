//! Dispatch Module
//!
//! The dispatch module provides a service with command routers. It wraps the
//! Glib context `spawn_local` function into the `spawn` function. Every command
//! is executed via the `spawn` function.
//!
//! The list of dispatch routers is in `dispatch_command_requests` function.
//!
//! All the NetworkManager core functions are in the sub modules.
//!
//! Sub Modules:
//! - `devices`: provides functions related to network hardware device.
//!     - List all network devices and their mac addresses.
//! - `connections`: provides functions related to nm connections.
//!     - List all connections.
//!     - Create a new wired connection.
pub mod devices;
pub mod connections;
use super::{NetworkCommand, NetworkRequest, NetworkResponse, TokioResponder};
use devices::list_ether_devices;
use connections::{list_connections, create_wired_connection, delete_connection};
use eyre::{Result, WrapErr};
use glib::MainContext;
use nm::Client;
use std::future::Future;

/// Define the dispatch routers
pub fn dispatch_command_requests(command_request: NetworkRequest) -> glib::Continue {
    let NetworkRequest { responder, command } = command_request;
    match command {
        NetworkCommand::ListDeivces => spawn(list_ether_devices(), responder),
        NetworkCommand::CreateWiredConnection(conn, device) => {
            spawn(create_wired_connection(conn, device), responder)
        },
        NetworkCommand::ListConnections => spawn(list_connections(), responder),
        NetworkCommand::DeleteConnection(conn) => spawn(delete_connection(conn), responder)
    };
    glib::Continue(true)
}

/// Wrap the glib
fn spawn(
    command_future: impl Future<Output = Result<NetworkResponse>> + 'static,
    responder: TokioResponder,
) {
    let context = MainContext::ref_thread_default();
    context.spawn_local(execute_and_respond(command_future, responder));
}

async fn execute_and_respond(
    command_future: impl Future<Output = Result<NetworkResponse>> + 'static,
    responder: TokioResponder,
) {
    let result = command_future.await;
    let _ = responder.send(result);
}

/// Create the NetworkManager client in async way.
/// If there is no NetworkManager daemon running, it will throw an error.
async fn create_client() -> Result<Client> {
    let client = Client::new_future()
        .await
        .context("Failed to create NetworkManager client")?;

    if !client.is_nm_running() {
        return Err(anyhow!("NetworkManager daemon is not running"));
    }

    Ok(client)
}
