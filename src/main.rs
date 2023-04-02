extern crate orbuculum_grpc;
extern crate orbuculum_nm;
use orbuculum_grpc::create_server;
use orbuculum_nm::{create_channel, gather_link_modes, run_network_manager_loop, State};
use std::sync::Arc;
use std::thread;

#[tokio::main]
async fn main() {
    env_logger::init();

    let (glib_sender, glib_receiver) = create_channel();
    let link_modes = gather_link_modes(None).await.unwrap();
    let arc_link_modes = Arc::new(link_modes);

    thread::spawn(move || {
        run_network_manager_loop(glib_receiver, arc_link_modes);
    });

    let shared_state = Arc::new(State::new(glib_sender));
    create_server(shared_state).await.unwrap();
}
