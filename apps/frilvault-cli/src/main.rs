mod cli;
mod command;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add(cmd) => {
            command::add::execute(cmd)?;
        }

        Commands::List(cmd) => {
            command::list::execute(cmd)?;
        }

        Commands::Update(cmd) => {
            command::update::execute(cmd)?;
        }

        Commands::Delete(cmd) => {
            command::delete::execute(cmd)?;
        }
    }

    Ok(())
}
