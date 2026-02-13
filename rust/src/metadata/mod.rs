use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;

pub mod providers;

#[derive(Debug, Clone)]
pub struct Metadata {
    pub labels: HashMap<String, String>,
}

#[async_trait]
pub trait MetadataProvider {
    async fn get_metadata(&self) -> Result<Metadata>;
}
