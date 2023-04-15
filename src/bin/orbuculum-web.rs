use axum::routing::{get, post, put};
use orbuculum_web::{
    get_connection_by_uuid, get_hostname, get_networking, health, list_connections, list_devices,
    restart_networking, set_hostname, set_networking, update_connection, update_connections,
    GrpcInfo,
};
use std::sync::Arc;
use structopt::StructOpt;
use tower_http::{
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::{info, Level};

#[derive(Debug, StructOpt)]
#[structopt(name = "orbuculum-web", about = "Usage information for orbuculum-web.")]
struct Argument {
    #[structopt(short, long, default_value = "http://127.0.0.1:15051")]
    grpc_address: String,
    #[structopt(default_value = "127.0.0.1:3000")]
    bind_address: String,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Argument::from_args();
    let state = Arc::new(GrpcInfo::new(&args.grpc_address));
    let addr = args.bind_address.parse().unwrap();

    // Build our application by creating our router.
    let app = axum::Router::new()
        .route("/api/proxy/devices", get(list_devices))
        .route("/api/proxy/hostname", get(get_hostname).post(set_hostname))
        .route(
            "/api/proxy/connections",
            get(list_connections).post(update_connections),
        )
        .route("/api/proxy/connection/:uuid", get(get_connection_by_uuid))
        .route("/api/proxy/connection", put(update_connection))
        .route(
            "/api/proxy/networking",
            get(get_networking).patch(set_networking),
        )
        .route("/api/proxy/restart", post(restart_networking))
        // health with tracing
        .route("/health", get(health))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Micros),
                ),
        )
        // healthz without tracing
        .route("/healthz", get(health))
        .with_state(state);

    info!("Web starts at {}", addr);

    // Run our application as a hyper server on http://localhost:3000.
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
