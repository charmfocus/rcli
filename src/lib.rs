mod cli;
mod process;

pub use cli::{Base64Format, Base64SubCommand, HttpSubCommand, Opts, SubCommand};
pub use process::*;

#[allow(async_fn_in_trait)]
pub trait CmdExector {
    async fn execute(self) -> anyhow::Result<()>;
}
