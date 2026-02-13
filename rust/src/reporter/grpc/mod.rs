use anyhow::Result;
use async_trait::async_trait;
use tonic::transport::Channel;

use crate::pb::parca::profilestore::v1alpha1::profile_store_service_client::ProfileStoreServiceClient;
use crate::reporter::Reporter;

pub struct GrpcReporter {
    client: ProfileStoreServiceClient<Channel>,
}

impl GrpcReporter {
    pub fn new(channel: Channel) -> Self {
        let client = ProfileStoreServiceClient::new(channel);
        Self { client }
    }
}

#[async_trait]
impl Reporter for GrpcReporter {
    async fn report(&self, profile: Vec<u8>) -> Result<()> {
        // TODO: Implement the report method
        Ok(())
    }
}
