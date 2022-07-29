use std::env;
use std::time::Duration;

use clap::Parser;

mod cli;
mod config;
mod electrode;
mod error;
mod heart;
mod lcd;

fn main() {
    let args = cli::Args::parse();

    init_logging();
    init_signal_handler();

    let _config = match config::load("./ecg.yaml".into()) {
        Ok(config) => config,
        Err(e) => {
            log::error!("couldn't load config: {}", e);
            std::process::exit(1)
        }
    };

    log::info!("starting v{}", env!("CARGO_PKG_VERSION"));

    let agent = ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(2))
        .timeout_write(Duration::from_secs(2))
        .build();
    let electrodes: Vec<Box<dyn electrode::Electrode>> = vec![
        Box::new(electrode::BlockHeight::default()),
        Box::new(electrode::Tombstoned::default()),
        Box::new(electrode::MissedBlocks::default()),
    ];
    let lcd = lcd::Client::new(agent.clone(), args.lcd_url, args.valcons_addr);
    let mut heart = heart::Heart::new(lcd, agent, args.heartbeat_url, electrodes, args.interval);
    heart.start()
}

fn init_logging() {
    match (std::env::var("ECG_LOG"), std::env::var("RUST_LOG")) {
        (Ok(_), Ok(_)) => (),
        (Ok(level), Err(_)) => env::set_var("RUST_LOG", format!("ecg={}", level)),
        (Err(_), Ok(_)) => (),
        (Err(_), Err(_)) => env::set_var("RUST_LOG", "ecg=info"),
    }
    pretty_env_logger::init();
    log::debug!("logging level set to {}", log::max_level())
}

fn init_signal_handler() {
    log::debug!("signal handler set");
    ctrlc::set_handler(|| {
        log::info!("bye!");
        std::process::exit(0);
    })
    .expect("error setting signal handler");
}
