// rcli csv -i input.csv -o output.json --header -d ','

use clap::Parser;
use rcli::{process_csv, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = opts.output.unwrap_or(format!("output.{}", opts.format));

            process_csv(&opts.input, &output, opts.format)?;
        }
    }
    Ok(())
}
