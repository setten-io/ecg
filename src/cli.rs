use clap::Parser;

/// Heartbeats for cosmos validators
#[derive(Parser, Debug)]
#[clap(version, about)]
pub(crate) struct Args {
    /// Path to yaml config
    #[clap(short, long, default_value = "ecg.yaml", env = "ECG_CONFIG_PATH")]
    pub(crate) path: String,
}
