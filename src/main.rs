use std::env;
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
    let checkables: Vec<Box<dyn checkable::Checkable>> = vec![
        Box::new(lcd::block::Block::new()),
        Box::new(lcd::signing_infos::SigningInfos::new()),
    ];
    let mut heart = heart::Heart::new(
        agent,
        args.beat_interval,
        args.lcd_url,
        args.heartbeat_url,
        args.valcons_addr,
        checkables,
    );
    heart.start()
}

fn init_logging() {
    match (std::env::var("ECG_LOG"), std::env::var("RUST_LOG")) {
        (Ok(_), Ok(_)) => (),
        (Ok(level), Err(_)) => env::set_var("RUST_LOG", format!("ecg={}", level)),
        (Err(_), Ok(_)) => (),
        (Err(_), Err(_)) => env::set_var("RUST_LOG", format!("ecg=info")),
    }
    pretty_env_logger::init();
    log::debug!(target: "logging", "level: {}", log::max_level())
}
