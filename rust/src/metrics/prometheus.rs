use anyhow::Result;
use async_trait::async_trait;
use http_body_util::{BodyExt, Empty, Full};
use hyper::body::{Bytes, Incoming};
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::server::conn::auto::Builder;
use hyper_util::rt::{TokioExecutor, TokioIo};
use prometheus::{Encoder, Registry, TextEncoder};
use std::net::SocketAddr;
use tokio::net::TcpListener;

use super::Metrics;

pub struct PrometheusMetrics {
    registry: Registry,
    address: SocketAddr,
}

impl PrometheusMetrics {
    pub fn new(address: SocketAddr) -> Self {
        PrometheusMetrics {
            registry: Registry::new(),
            address,
        }
    }
}

#[async_trait]
impl Metrics for PrometheusMetrics {
    async fn start(&self) -> Result<()> {
        let listener = TcpListener::bind(self.address).await?;
        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);
            let registry = self.registry.clone();

            tokio::task::spawn(async move {
                let service = hyper::service::service_fn(move |req: Request<Incoming>| {
                    let registry = registry.clone();
                    async move {
                        let mut response = Response::new(Empty::new().boxed());
                        match (req.method(), req.uri().path()) {
                            (&Method::GET, "/metrics") => {
                                let mut buffer = vec![];
                                let encoder = TextEncoder::new();
                                let metric_families = registry.gather();
                                encoder.encode(&metric_families, &mut buffer).unwrap();
                                *response.body_mut() = Full::new(Bytes::from(buffer)).boxed();
                            }
                            _ => {
                                *response.status_mut() = StatusCode::NOT_FOUND;
                            }
                        }
                        Ok::<_, hyper::Error>(response)
                    }
                });

                if let Err(err) = Builder::new(TokioExecutor::new())
                    .serve_connection(io, service)
                    .await
                {
                    tracing::error!("failed to serve connection: {}", err);
                }
            });
        }
    }
}
