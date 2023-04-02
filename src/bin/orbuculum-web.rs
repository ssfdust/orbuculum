use axum::routing::get;
use orbuculum_web::{get_connection_by_uuid, list_connections, list_devices};

#[tokio::main]
async fn main() {
    // Build our application by creating our router.
    let app = axum::Router::new()
        .route("/api/proxy/devices", get(list_devices))
        .route("/api/proxy/connections", get(list_connections))
        .route("/api/proxy/connection/:uuid", get(get_connection_by_uuid));

    // Run our application as a hyper server on http://localhost:3000.
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
