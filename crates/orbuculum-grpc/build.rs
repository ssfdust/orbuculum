use std::{env, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        // .out_dir("target/")  // you can change the generated code's location
        .type_attribute(
            "DevicesReply",
            "#[derive(serde::Deserialize,serde::Serialize)]",
        )
        .type_attribute(
            "DevicesReplyBody",
            "#[derive(serde::Deserialize,serde::Serialize)]",
        )
        .type_attribute("Routes", "#[derive(serde::Deserialize,serde::Serialize)]")
        .type_attribute("Netinfo", "#[derive(serde::Deserialize,serde::Serialize)]")
        .type_attribute(
            "NetworkingStateBody",
            "#[derive(serde::Deserialize,serde::Serialize)]",
        )
        .type_attribute(
            "NetworkingStateReply",
            "#[derive(serde::Deserialize,serde::Serialize)]",
        )
        .type_attribute(
            "HostnameReply",
            "#[derive(serde::Deserialize,serde::Serialize)]",
        )
        .type_attribute(
            "HostnameBody",
            "#[derive(serde::Deserialize,serde::Serialize)]",
        )
        .type_attribute(
            "ConnectionBody",
            "#[derive(serde::Deserialize,serde::Serialize)]",
        )
        .type_attribute(
            "ConnectionReply",
            "#[derive(serde::Deserialize,serde::Serialize)]",
        )
        .type_attribute(
            "ConnectionsReply",
            "#[derive(serde::Deserialize,serde::Serialize)]",
        )
        .type_attribute(
            "ConnectionItem",
            "#[derive(serde::Deserialize,serde::Serialize)]",
        )
        .file_descriptor_set_path(out_dir.join("network_descriptor.bin"))
        .compile(
            &["proto/network.proto"],
            &["proto/"], // specify the root location to search proto dependencies
        )
        .unwrap();
}
