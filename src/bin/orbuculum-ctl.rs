use orbuculum_ctl::mainloop;
use std::sync::Arc;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "orbuculum-web", about = "Usage information for orbuculum-web.")]
struct Argument {
    #[structopt(short, long, default_value = "http://127.0.0.1:15051")]
    grpc_address: String,
}


#[tokio::main]
async fn main() {
    let args = Argument::from_args();
    let grpc_addr = Arc::new(args.grpc_address.as_str());
    loop {
        match mainloop(grpc_addr.clone()).await {
            Ok(_) => break,
            _ => ()
        }
    }
}
