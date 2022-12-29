fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        // .out_dir("src/network_grpc")  // you can change the generated code's location
        .compile(
            &["proto/network.proto"],
            &["proto/"], // specify the root location to search proto dependencies
        )
        .unwrap();
    Ok(())
}
