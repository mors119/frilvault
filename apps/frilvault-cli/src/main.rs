mod cli;
mod command;
mod output;

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

        Commands::Search(cmd) => {
            command::search::execute(cmd)?;
        }
        Commands::Doctor(cmd) => {
            command::doctor::execute(cmd)?;
        }

        Commands::Health(cmd) => {
            command::doctor::execute(cmd)?;
        }

        Commands::Stats(cmd) => command::stats::execute(cmd)?,

        Commands::Repair(cmd) => {
            command::repair::execute(cmd)?;
        }

        Commands::Gitignore(cmd) => {
            command::gitignore::execute(cmd)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests;
