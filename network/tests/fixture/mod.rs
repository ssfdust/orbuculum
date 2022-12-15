use rstest::fixture;
use std::thread;
use std::sync::Arc;
use network::{run_network_manager_loop, create_channel, State};

#[fixture]
#[once]
pub fn start_instance() -> Arc<State> {
    let (glib_sender, glib_receiver) = create_channel();
    thread::spawn(move || {
        run_network_manager_loop(glib_receiver);
    });
    Arc::new(State { glib_sender })
}
