use orbuculum_nm::{create_channel, gather_link_modes, run_network_manager_loop, State};
use rstest::fixture;
use std::sync::Arc;
use std::thread;

#[fixture]
pub async fn start_instance() -> Arc<State> {
    let (glib_sender, glib_receiver) = create_channel();
    let nic_linkmodes = Arc::new(
        gather_link_modes(None)
            .await
            .expect("Failed to gather nic_linkmodes, please check your permissions."),
    );
    thread::spawn(move || {
        run_network_manager_loop(glib_receiver, nic_linkmodes);
    });
    Arc::new(State::new(glib_sender))
}
