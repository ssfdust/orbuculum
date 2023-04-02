use axum::routing::get;
use orbuculum_web::{get_connection_by_uuid, list_connections, list_devices, health};
use tracing::{Level, info};
use tower_http::{
    LatencyUnit,
    trace::{TraceLayer, DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse},
};

#[tokio::main]
async fn main() {
    env_logger::init();

    let addr = "0.0.0.0:3000";

    // Build our application by creating our router.
    let app = axum::Router::new()
        .route("/api/proxy/devices", get(list_devices))
        .route("/api/proxy/connections", get(list_connections))
        .route("/api/proxy/connection/:uuid", get(get_connection_by_uuid))
        // health with tracing
        .route("/health", get(health))
        .layer(TraceLayer::new_for_http()
                .make_span_with(
                    DefaultMakeSpan::new().include_headers(true)
                )
                .on_request(
                    DefaultOnRequest::new().level(Level::INFO)
                )
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Micros)
                ))
        // healhz without tracing
        .route("/healthz", get(health));

    info!("Web starts at {}", addr);

    // Run our application as a hyper server on http://localhost:3000.
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
