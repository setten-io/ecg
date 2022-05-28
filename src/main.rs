use clap::Parser;

mod cli;
mod error;

fn main() {
    cli::Args::parse();
}
