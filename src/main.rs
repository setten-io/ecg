use std::env;
use std::path::PathBuf;
use std::time::Duration;

use clap::Parser;

mod cli;
mod client;
mod config;
mod electrode;
mod error;
// mod heart;

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();

    init_logging();
    init_signal_handler();

    let _config = match config::load(&PathBuf::from(args.path)) {
        Ok(config) => config,
        Err(e) => {
            log::error!("{}", e);
            std::process::exit(1)
        }
    };

    log::info!("starting v{}", env!("CARGO_PKG_VERSION"));

    let _http = match reqwest::Client::builder()
        .timeout(Duration::from_secs(2))
        .build()
    {
        Ok(http) => http,
        Err(e) => {
            log::error!("couldn't build http client: {}", e);
            std::process::exit(1)
        }
    };

    let _electrodes: Vec<Box<dyn electrode::Electrode>> = vec![
        Box::new(electrode::BlockHeight::default()),
        Box::new(electrode::Tombstoned::default()),
        Box::new(electrode::MissedBlocks::default()),
    ];

    // let lcd = lcd::Client::new(agent.clone(), args.lcd_url, args.valcons_addr);
    // let mut heart = heart::Heart::new(lcd, agent, args.heartbeat_url, electrodes, args.interval);
    // heart.start()
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
