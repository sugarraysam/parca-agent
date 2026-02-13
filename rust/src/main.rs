use anyhow::Result;
use parca_agent::{
    config, logger,
    metadata::{providers::system::SystemProvider, MetadataProvider},
    metrics::{prometheus::PrometheusMetrics, Metrics},
    reporter::grpc::GrpcReporter,
};
use std::net::SocketAddr;
use tonic::transport::Channel;

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = config::load()?;
    logger::init(&cfg.log_level);
    tracing::info!("Hello, from Parca Agent in Rust!");
    tracing::debug!("Config: {:#?}", cfg);

    if let Some(address_str) = cfg.http_address.clone() {
        let address: SocketAddr = address_str.parse()?;
        let metrics = PrometheusMetrics::new(address);
        tokio::spawn(async move {
            if let Err(e) = metrics.start().await {
                tracing::error!("Failed to start metrics server: {}", e);
            }
        });
    }

    let metadata_provider = SystemProvider;
    let metadata = metadata_provider.get_metadata().await?;
    tracing::info!("Metadata: {:#?}", metadata);

    let channel = Channel::from_shared(cfg.remote_store.address.unwrap_or_default())?
        .connect()
        .await?;
    let reporter = GrpcReporter::new(channel);

    // Keep the main thread alive
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
