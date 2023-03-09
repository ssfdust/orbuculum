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
mod net;
mod tokio_client;
mod utils;

pub use dispatch::connections::Connection;
pub use dispatch::devices::NetDevice;
use serde_json::Value;

pub use net::{NetInfo, Route};
pub use tokio_client::{
    create_channel, run_network_manager_loop, send_command, NetworkRequest, State, TokioResponder,
};

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
    SetManage(String, bool),
    DeleteConnection(String),
    UpdateIP4Config(String, NetInfo),
    UpdateIP6Config(String, NetInfo),
}

/// The network response list
/// provides all the responses supported by the server.
pub enum NetworkResponse {
    Return(Value),
    ListConnection(Vec<Connection>),
    IP(Option<NetInfo>),
    Success,
    Failed,
}

impl NetworkResponse {
    pub fn into_value(self) -> Option<Value> {
        match self {
            NetworkResponse::Return(val) => Some(val),
            _ => None,
        }
    }
}
