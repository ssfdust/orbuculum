use network::{create_channel, run_network_manager_loop, State};
use rstest::fixture;
use std::sync::Arc;
use std::thread;

#[fixture]
#[once]
pub fn start_instance() -> Arc<State> {
    let (glib_sender, glib_receiver) = create_channel();
    thread::spawn(move || {
        run_network_manager_loop(glib_receiver);
    });
    Arc::new(State { glib_sender })
}
