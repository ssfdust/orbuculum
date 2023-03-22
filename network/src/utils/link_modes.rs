//! The module provides functions that gather network cards link modes.
use ethtool::{
    new_connection,
    EthtoolAttr::LinkMode,
    EthtoolHeader::DevName,
    EthtoolLinkModeAttr::{Header, Ours},
};
use eyre::Result;
use futures::stream::TryStreamExt;
use serde_json::Value;
use tokio::spawn;

/// Gather all network cards' link modes and convert them into serde_json::Value
/// Stolen codes from rust-netlink link_modes example
pub async fn gather_link_modes(iface_name: Option<&str>) -> Result<Value> {
    let mut nic_linkmodes: Value = serde_json::from_str("{}")?;
    let (connection, mut handle, _) = new_connection()?;
    spawn(connection);

    let mut link_mode_handle = handle.link_mode().get(iface_name).execute().await;

    let mut msgs = Vec::new();
    while let Some(msg) = link_mode_handle.try_next().await? {
        msgs.push(msg);
    }
    for msg in msgs {
        let mut features: Vec<String> = vec![];
        let mut iface_name: String = String::new();
        for nlas in msg.payload.nlas {
            match nlas {
                LinkMode(Ours(items)) => {
                    features = items.iter().map(|x| x.to_owned()).collect::<Vec<String>>();
                }
                LinkMode(Header(items)) => {
                    for item in items {
                        match item {
                            DevName(name) => iface_name = name,
                            _ => (),
                        }
                    }
                }
                _ => (),
            }
        }
        if iface_name.len() > 0 {
            nic_linkmodes[iface_name] = serde_json::to_value(features)?;
        }
    }
    Ok(nic_linkmodes)
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[tokio::test]
    async fn test_gather_nic_linkmodes() {
        let link_modes = gather_link_modes(None).await;
        assert!(link_modes.is_ok());
        let link_modes = link_modes.unwrap();
        assert!(link_modes.as_object().is_some());
        let link_modes = link_modes.as_object().unwrap();
        for (key, val) in link_modes {
            assert!(key.len() > 0);
            assert!(val.as_array().is_some());
            let val = val.as_array().unwrap();
            for item in val {
                assert!(item.as_str().is_some())
            }
        }
    }
}
