use serde::Deserialize;
use std::path::PathBuf;

pub mod cli;
pub mod file;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub remote_store: RemoteStore,
    pub profiling: Profiling,
    pub debuginfo: DebugInfo,
    pub telemetry: Telemetry,
    pub otlp: Otlp,
    pub http_address: Option<String>,
    #[serde(skip)]
    pub config_path: Option<PathBuf>,
    #[serde(skip)]
    pub version: bool,
    pub log_level: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct RemoteStore {
    pub address: Option<String>,
    pub insecure: bool,
    pub insecure_skip_verify: bool,
    pub bearer_token: Option<String>,
    pub bearer_token_file: Option<PathBuf>,
}

#[derive(Debug, Default, Deserialize)]
pub struct Profiling {
    pub duration: u64,
    pub cpu_sampling_frequency: u64,
    pub probabilistic_interval: u64,
    pub probabilistic_threshold: u64,
    pub label_ttl: u64,
    pub enable_error_frames: bool,
    pub off_cpu_threshold: f64,
}

#[derive(Debug, Default, Deserialize)]
pub struct DebugInfo {
    pub strip: bool,
    pub upload_max_parallel: u64,
    pub upload_disable: bool,
    pub upload_queue_size: u64,
    pub temp_dir: Option<PathBuf>,
}

#[derive(Debug, Default, Deserialize)]
pub struct Telemetry {
    pub disable_panic_reporting: bool,
    pub stderr_buffer_size_kb: u64,
}

#[derive(Debug, Default, Deserialize)]
pub struct Otlp {
    pub exporter: Option<String>,
    pub address: Option<String>,
}

pub fn load() -> Result<Config, anyhow::Error> {
    let cli_config = cli::parse();

    let file_config = if let Some(config_path) = &cli_config.config_path {
        file::load(config_path)?
    } else {
        Config::default()
    };

    let config = Config {
        remote_store: RemoteStore {
            address: cli_config
                .remote_store
                .address
                .or(file_config.remote_store.address),
            insecure: cli_config.remote_store.insecure || file_config.remote_store.insecure,
            insecure_skip_verify: cli_config.remote_store.insecure_skip_verify
                || file_config.remote_store.insecure_skip_verify,
            bearer_token: cli_config
                .remote_store
                .bearer_token
                .or(file_config.remote_store.bearer_token),
            bearer_token_file: cli_config
                .remote_store
                .bearer_token_file
                .or(file_config.remote_store.bearer_token_file),
        },
        profiling: Profiling {
            duration: if cli_config.profiling.duration != 10 {
                cli_config.profiling.duration
            } else {
                file_config.profiling.duration
            },
            cpu_sampling_frequency: if cli_config.profiling.cpu_sampling_frequency != 19 {
                cli_config.profiling.cpu_sampling_frequency
            } else {
                file_config.profiling.cpu_sampling_frequency
            },
            ..file_config.profiling
        },
        ..cli_config
    };

    Ok(config)
}
