use std::env;
use std::path::PathBuf;
use std::time::Duration;

use clap::Parser;
use futures::future;

use crate::client::lcd::Lcd;
use crate::client::Client;
use crate::config::TargetConfig;
use crate::heart::Heart;

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

    let mut hearts: Vec<_> = config
        .targets
        .into_iter()
        .map(|(name, target)| start_heart(name, target, http.clone()))
        .collect();
    future::join_all(hearts.iter_mut().map(|h| h.start())).await;
}

fn start_heart(name: String, target: TargetConfig, http: reqwest::Client) -> Heart {
    let clients: Vec<Box<dyn Client>> = target
        .clients
        .into_iter()
        .map(|client_config| {
            let client: Box<dyn Client> = match client_config {
                config::ClientsConfig::Lcd { url } => {
                    Box::new(Lcd::new(http.clone(), url, target.valcons_address.clone()))
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

    Heart::new(
        name,
        clients,
        http.clone(),
        target.url.clone(),
        electrodes,
        target.interval,
    )
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
