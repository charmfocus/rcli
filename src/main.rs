// rcli csv -i input.csv -o output.json --header -d ','

use clap::Parser;
use rcli::{CmdExector, Opts};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    Opts::parse().cmd.execute().await
}
