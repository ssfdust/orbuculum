//! ### Networking Module
//!
//! Control whether overall networking is enabled or disabled.
//!
//! #### Functions
//! - set_networking: Set the overall networking state.
use super::create_client;
use crate::NetworkResponse;
use eyre::Result;
use nm::{DBUS_INTERFACE, DBUS_PATH};
use serde_json::json;

/// When disabled, all interfaces that NM manages are deactivated.
/// When enabled, all managed interfaces are available to be activated.
pub async fn set_networking(state: bool) -> Result<NetworkResponse> {
    let client = create_client().await?;
    let networking_status = glib::Variant::tuple_from_iter(vec![glib::Variant::from(state)]);
    let current_status = client.is_networking_enabled();
    if current_status != state {
        client
            .dbus_call_future(
                DBUS_PATH.as_str(),
                DBUS_INTERFACE.as_str(),
                "Enable",
                Some(&networking_status),
                None,
                2000,
            )
            .await?;
    }
    Ok(NetworkResponse::Success)
}

/// Indicates if overall networking is currently enabled or not.
pub async fn get_networking() -> Result<NetworkResponse> {
    let client = create_client().await?;
    let state = client.is_networking_enabled();
    Ok(NetworkResponse::Return(json!({ "state": state })))
}
