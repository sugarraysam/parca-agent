use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub fn init(log_level: &str) {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::new(log_level))
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
}
