use std::time::Duration;

use clap::Parser;

mod checkable;
mod cli;
mod error;
mod heart;
mod lcd;

fn main() {
    init_logging();
    log::info!("starting v{}", env!("CARGO_PKG_VERSION"));
    let args = cli::Args::parse();
    let agent = ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(2))
        .timeout_write(Duration::from_secs(2))
        .build();
    let checkables: Vec<Box<dyn checkable::Checkable>> = vec![Box::new(lcd::block::Block::new())];
    let mut heart = heart::Heart::new(
        agent,
        args.beat_interval,
        args.lcd_url,
        args.heartbeat_url,
        checkables,
    );
    heart.start()
}

fn init_logging() {
    if std::env::var("ECG_LOG").is_err() {
        std::env::set_var("ECG_LOG", "info");
    }
    pretty_env_logger::init_custom_env("ECG_LOG");
    log::debug!(target: "logging", "level: {}", log::max_level())
}
