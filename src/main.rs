use std::env;
use std::path::PathBuf;
use std::time::Duration;

use clap::Parser;
use futures::future;
use tokio::task::JoinHandle;

use crate::{
    client::{lcd::Lcd, Client},
    config::TargetConfig,
    heart::Heart,
};

mod cli;
mod client;
mod config;
mod electrode;
mod error;
mod heart;

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();

    init_logging();
    init_signal_handler();

    let config = match config::load(&PathBuf::from(args.path)) {
        Ok(config) => config,
        Err(e) => {
            log::error!("{}", e);
            std::process::exit(1)
        }
    };

    let http = match reqwest::Client::builder()
        .timeout(Duration::from_secs(2))
        .build()
    {
        Ok(http) => http,
        Err(e) => {
            log::error!("couldn't build http client: {}", e);
            std::process::exit(1)
        }
    };

    log::info!("starting v{}", env!("CARGO_PKG_VERSION"));

    let handlers: Vec<_> = config
        .targets
        .into_iter()
        .map(|(name, target)| start_heart(name, target, http))
        .collect();
    future::join_all(handlers).await;
}

fn start_heart(_: String, target: TargetConfig, http: reqwest::Client) -> JoinHandle<()> {
    let clients: Vec<Box<dyn Client>> = target
        .clients
        .into_iter()
        .map(|client_config| {
            let client: Box<dyn Client> = match client_config {
                config::ClientsConfig::Lcd { url } => {
                    Box::new(Lcd::new(http, url, target.valcons_address.clone()))
                }
            };
            client
        })
        .collect();

    let electrodes: Vec<Box<dyn electrode::Electrode>> = vec![
        Box::new(electrode::BlockHeight::default()),
        Box::new(electrode::Tombstoned::default()),
        Box::new(electrode::MissedBlocks::default()),
    ];

    let mut heart = Heart::new(
        clients,
        http,
        target.url.clone(),
        electrodes,
        target.interval,
    );
    tokio::task::spawn(async move { heart.start().await })
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
