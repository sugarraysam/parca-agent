use anyhow::Result;

pub mod prometheus;

#[async_trait::async_trait]
pub trait Metrics {
    async fn start(&self) -> Result<()>;
}
