extern crate orbuculum_grpc;
extern crate orbuculum_nm;
use orbuculum_grpc::{create_server, initialize_network_manager};
use orbuculum_nm::{create_channel, gather_link_modes, run_network_manager_loop, State};
use std::sync::Arc;
use std::thread;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "orbuculum", about = "Usage information for orbuculum.")]
struct Argument {
    #[structopt(short, long)]
    no_initialize: bool,
    #[structopt(default_value = "127.0.0.1:15051")]
    bind_address: String,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Argument::from_args();

    let (glib_sender, glib_receiver) = create_channel();
    let link_modes = gather_link_modes(None).await.unwrap();
    let arc_link_modes = Arc::new(link_modes);

    thread::spawn(move || {
        run_network_manager_loop(glib_receiver, arc_link_modes);
    });

    let shared_state = Arc::new(State::new(glib_sender));

    if !args.no_initialize {
        initialize_network_manager(shared_state.clone(), "/etc/orbuculum/nic.rules".to_owned())
            .await;
    }
    create_server(shared_state, args.bind_address)
        .await
        .unwrap();
}
