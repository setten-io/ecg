use std::time::Duration;

use clap::Parser;

mod cli;
mod error;
mod heart;

fn main() {
    let args = cli::Args::parse();
    let _agent = ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(5))
        .timeout_write(Duration::from_secs(5))
        .build();
    let heart = heart::Heart::new(args.beat_interval);
    heart.beat()
}
