use network::{create_channel, run_network_manager_loop, State, gather_link_modes};
use server::create_server;
use std::sync::Arc;
use std::thread;

#[tokio::main]
async fn main() {
    let (glib_sender, glib_receiver) = create_channel();
    let link_modes = gather_link_modes(None).await.unwrap();
    let arc_link_modes = Arc::new(link_modes);

    thread::spawn(move || {
        run_network_manager_loop(glib_receiver, arc_link_modes);
    });

    let shared_state = Arc::new(State::new(glib_sender));
    create_server(shared_state).await.unwrap();
}
