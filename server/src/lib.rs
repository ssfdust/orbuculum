use eyre::Result;
use hyper::Body;
use network::{send_command, NetworkCommand, NetworkResponse, State};
use network_grpc::network_server::{Network, NetworkServer};
use network_grpc::{ConnectionReply, ConnectionReplyBody};
use std::sync::Arc;
use std::{
    task::{Context, Poll},
    time::Duration,
};
use tokio::macros::support::Future;
use tonic::{body::BoxBody, transport::Server, Request, Response, Status};
use tower::{Layer, Service};

pub mod network_grpc {
    tonic::include_proto!("network"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct NetworkService {}

#[tonic::async_trait]
impl Network for NetworkService {
    async fn list_connection(
        &self,
        request: Request<()>,
    ) -> Result<Response<ConnectionReply>, Status> {
        let shared_state = request.extensions().get::<Arc<State>>().unwrap();
        let mut body: Vec<ConnectionReplyBody> = vec![];
        if let Ok(NetworkResponse::ListConnection(conns)) =
            send_command(shared_state, NetworkCommand::ListConnections).await
        {
            for conn in conns {
                body.push(ConnectionReplyBody {
                    name: conn.name.clone(),
                    uuid: conn.uuid.clone(),
                    interface: conn.interface.unwrap_or("".to_string()).clone()
                });
            }
        }

        let reply = ConnectionReply {
            code: 0,
            msg: "Successfully".into(),
            data: body,
        };
        Ok(Response::new(reply))
    }
}

pub fn create_server(
    shared_state: Arc<State>,
) -> impl Future<Output = Result<(), tonic::transport::Error>> {
    let addr = "127.0.0.1:50051".parse().unwrap();
    let network_service = NetworkService::default();
    let svc = NetworkServer::new(network_service);
    let layer = tower::ServiceBuilder::new()
        .timeout(Duration::from_secs(30))
        .layer(GlibStateLayer::new(shared_state))
        .into_inner();

    Server::builder().layer(layer).add_service(svc).serve(addr)
}

#[derive(Clone)]
struct GlibStateMiddleware<S> {
    inner: S,
    ref_state: Arc<State>,
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
            ref_state: Arc::clone(self.shared_state),
        }
    }
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
