use clap::Parser;

/// Heartbeats for cosmos validators
#[derive(Parser, Debug)]
#[clap(version, about)]
pub struct Args {
    /// Validator tendermint consensus address
    #[clap(required = true, env = "VALCONS_ADDR")]
    pub valcons_addr: String,

    /// LCD url used to communicate with the blockchain
    #[clap(required = true, env = "LCD_URL")]
    pub lcd_url: String,

    /// Url to send heartbeats to
    #[clap(required = true, env = "HEARTBEAT_URL")]
    pub heartbeat_url: String,
}
