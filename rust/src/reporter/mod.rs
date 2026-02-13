use anyhow::Result;
use async_trait::async_trait;

pub mod grpc;

#[async_trait]
pub trait Reporter {
    async fn report(&self, profile: Vec<u8>) -> Result<()>;
}
