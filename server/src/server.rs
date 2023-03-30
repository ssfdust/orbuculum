//! The server module implements the server with gio state as an extension
//! and handles the arguments about the server.
//!
//! We create the `GlibStateLayer` layer, which share the `GlibStateMiddleware`
//! with the server.

use network::State;
use std::sync::Arc;

use super::{network_grpc::NETWROK_FILE_DESCRIPTOR_SET, NetworkServer, NetworkService};
use eyre::Result;
use hyper::Body;
use std::{
    task::{Context, Poll},
    time::Duration,
};
use tokio::macros::support::Future;
use tonic::{body::BoxBody, transport::Server};
use tower::{Layer, Service};

/// Before creating the server instance, we have to initilize the shared state
/// between the gio thread and the tonic thread, and pass the shared state to
/// the server as an extension.
pub fn create_server(
    shared_state: Arc<State>,
) -> impl Future<Output = Result<(), tonic::transport::Error>> {
    let addr = "127.0.0.1:50051".parse().unwrap();
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(NETWROK_FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();
    let network_service = NetworkService::default();
    let svc = NetworkServer::new(network_service);
    let layer = tower::ServiceBuilder::new()
        .timeout(Duration::from_secs(30))
        .layer(GlibStateLayer::new(shared_state))
        .into_inner();

    Server::builder()
        .layer(layer)
        .accept_http1(true)
        .add_service(svc)
        .add_service(reflection_service)
        .serve(addr)
}

#[derive(Clone)]
struct GlibStateLayer {
    shared_state: Arc<State>,
}

impl GlibStateLayer {
    pub fn new(shared_state: Arc<State>) -> Self {
        GlibStateLayer { shared_state }
    }
}

impl<S> Layer<S> for GlibStateLayer {
    type Service = GlibStateMiddleware<S>;

    fn layer(&self, service: S) -> Self::Service {
        GlibStateMiddleware {
            inner: service,
            ref_state: Arc::clone(&self.shared_state),
        }
    }
}

#[derive(Clone)]
struct GlibStateMiddleware<S> {
    inner: S,
    ref_state: Arc<State>,
}

impl<S> Service<hyper::Request<Body>> for GlibStateMiddleware<S>
where
    S: Service<hyper::Request<Body>, Response = hyper::Response<BoxBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = futures::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: hyper::Request<Body>) -> Self::Future {
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);
        req.extensions_mut().insert(self.ref_state.clone());

        Box::pin(async move {
            // Do extra async work here...
            let response = inner.call(req).await?;

            Ok(response)
        })
    }
}
