use clap::Parser;
use std::path::PathBuf;

use super::{Config, DebugInfo, Otlp, Profiling, RemoteStore, Telemetry};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(long, value_name = "FILE", help = "Path to the configuration file.")]
    config_path: Option<PathBuf>,

    #[clap(long, help = "Remote store address.")]
    remote_store_address: Option<String>,

    #[clap(long, help = "Connect to the remote store in insecure mode.")]
    remote_store_insecure: bool,

    #[clap(long, help = "Skip verifying the remote store's certificate.")]
    remote_store_insecure_skip_verify: bool,

    #[clap(long, help = "Bearer token for the remote store.")]
    remote_store_bearer_token: Option<String>,

    #[clap(
        long,
        value_name = "FILE",
        help = "File containing the bearer token for the remote store."
    )]
    remote_store_bearer_token_file: Option<PathBuf>,

    #[clap(long, help = "Profiling duration.", default_value = "10")]
    profiling_duration: u64,

    #[clap(long, help = "CPU sampling frequency.", default_value = "19")]
    profiling_cpu_sampling_frequency: u64,

    #[clap(long, help = "Show version information.")]
    version: bool,

    #[clap(long, help = "Log level.", default_value = "info")]
    log_level: String,

    #[clap(
        long,
        help = "HTTP address to expose metrics and pprof.",
        default_value = "127.0.0.1:7071"
    )]
    http_address: String,
}

pub fn parse() -> Config {
    let cli = Cli::parse();

    Config {
        config_path: cli.config_path,
        remote_store: RemoteStore {
            address: cli.remote_store_address,
            insecure: cli.remote_store_insecure,
            insecure_skip_verify: cli.remote_store_insecure_skip_verify,
            bearer_token: cli.remote_store_bearer_token,
            bearer_token_file: cli.remote_store_bearer_token_file,
        },
        profiling: Profiling {
            duration: cli.profiling_duration,
            cpu_sampling_frequency: cli.profiling_cpu_sampling_frequency,
            ..Default::default()
        },
        debuginfo: DebugInfo::default(),
        telemetry: Telemetry::default(),
        otlp: Otlp::default(),
        http_address: Some(cli.http_address),
        version: cli.version,
        log_level: cli.log_level,
    }
}
