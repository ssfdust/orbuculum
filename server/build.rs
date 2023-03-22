use std::{env, path::PathBuf};
fn main() {
    /*
    tonic_build::configure()
        .build_server(true)
        // .out_dir("src/network_grpc")  // you can change the generated code's location
        .compile(
            &["proto/network.proto"],
            &["proto/"], // specify the root location to search proto dependencies
        )
        .unwrap();
    Ok(())
    */
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    build_json_codec_service(out_dir);
}

fn build_json_codec_service(out_dir: PathBuf) {
    let greeter_service = tonic_build::manual::Service::builder()
        .name("Network")
        .package("json.network")
        .method(
            tonic_build::manual::Method::builder()
                .name("list_devices")
                .route_name("ListDeivces")
                .input_type("crate::Empty")
                .output_type("crate::Test")
                .codec_path("crate::decoder::JsonCodec")
                .build(),
        )
        .build();

    tonic_build::manual::Builder::new()
        .out_dir(out_dir)
        .compile(&[greeter_service]);
}

