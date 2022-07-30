use std::env;
use std::path::PathBuf;
use std::time::Duration;

use clap::Parser;

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

    log::info!("starting v{}", env!("CARGO_PKG_VERSION"));

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

    let client = client::lcd::Lcd::new(
        http.clone(),
        "https://phoenix-lcd.terra.dev".into(),
        "terravalcons1hn7u8qf5z8lyufjlzr93lvtel0dp3z4z3h95da".into(),
    );

    let electrodes: Vec<Box<dyn electrode::Electrode>> = vec![
        Box::new(electrode::BlockHeight::default()),
        Box::new(electrode::Tombstoned::default()),
        Box::new(electrode::MissedBlocks::default()),
    ];

    config.targets.iter().for_each(|(n, c)| {
        let clients = c.clients.iter().map(|client_config| match client_config {
            config::ClientsConfig::Lcd { url } => {
                client::lcd::Lcd::new(http.clone(), url.to_string(), c.valcons_address.clone())
            }
            _ => todo!(),
        });
        let heart = heart::Heart::new(
            Box::new(clients.take(1).collect::<Vec<dyn client::Client>>()),
            http.clone(),
            c.url,
            electrodes,
            c.interval,
        );
        tokio::task::spawn(async {
            // perform some work here...
        });
    });

    let mut heart = heart::Heart::new(
        Box::new(client),
        http,
        "https://google.com".into(),
        electrodes,
        10,
    );
    heart.start().await
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
