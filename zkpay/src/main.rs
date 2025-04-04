mod wallet;
mod ledger;
mod circuit;
mod cli;
mod error;

use anyhow::Result;
use clap::Parser;
use cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.execute()?;
    Ok(())
}
