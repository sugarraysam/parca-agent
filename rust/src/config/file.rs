use crate::config::Config;
use anyhow::Result;
use std::path::Path;

pub fn load(path: &Path) -> Result<Config> {
    let content = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
