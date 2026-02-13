use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;
use sysinfo::System;

use crate::metadata::{Metadata, MetadataProvider};

pub struct SystemProvider;

#[async_trait]
impl MetadataProvider for SystemProvider {
    async fn get_metadata(&self) -> Result<Metadata> {
        // Always allocate a full System struct on startup, then just refresh
        // what is needed (e.g.: processes running).
        let mut sys = System::new_all();
        sys.refresh_all();

        let mut labels = HashMap::new();
        labels.insert("os".to_string(), System::name().unwrap_or_default());
        labels.insert(
            "kernel_release".to_string(),
            System::kernel_version().unwrap_or_default(),
        );
        labels.insert(
            "hostname".to_string(),
            System::host_name().unwrap_or_default(),
        );
        labels.insert("cpus".to_string(), sys.cpus().len().to_string());

        Ok(Metadata { labels })
    }
}
