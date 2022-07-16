use std::time::Duration;

use clap::Parser;

mod checker;
mod cli;
mod error;
mod heart;
mod lcd;

fn main() {
    let args = cli::Args::parse();
    let agent = ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(2))
        .timeout_write(Duration::from_secs(2))
        .build();
    let checkables: Vec<Box<dyn checker::Checker>> = vec![Box::new(lcd::block::Block::new())];
    let mut heart = heart::Heart::new(agent, args.beat_interval, checkables);
    heart.start()
}
