use orbuculum_ctl::mainloop;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let grpc_addr = Arc::new("http://127.0.0.1:15051");
    mainloop(grpc_addr).await.unwrap();
}
