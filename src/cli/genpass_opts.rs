use crate::CmdExecutor;
use clap::Parser;
use zxcvbn::zxcvbn;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 8)]
    pub length: u8,
    #[arg(long, default_value_t = true)]
    pub lowercase: bool,
    #[arg(long, default_value_t = true)]
    pub uppercase: bool,
    #[arg(long, default_value_t = true)]
    pub numbers: bool,
    #[arg(long, default_value_t = true)]
    pub symbols: bool,
}

impl CmdExecutor for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let ret = crate::process_genpass(
            self.length,
            self.uppercase,
            self.lowercase,
            self.numbers,
            self.symbols,
        )?;
        println!("{}", ret);

        let estimate = zxcvbn(&ret, &[]);
        eprintln!("Password strength: {}", estimate.score());
        Ok(())
    }
}
