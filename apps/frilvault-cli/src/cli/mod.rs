use clap::{Parser, Subcommand};

pub mod add;
pub mod delete;
pub mod explorer;
pub mod format;
pub mod gitignore;
pub mod health;
pub mod list;
pub mod repair;
pub mod search;
pub mod stats;
pub mod update;

use add::AddCommand;
use delete::DeleteCommand;
use explorer::ExplorerCommand;
use gitignore::GitignoreCommand;
use health::HealthCommand;
use list::ListCommand;
use repair::RepairCommand;
use search::SearchCommand;
use stats::StatsCommand;
use update::UpdateCommand;

#[derive(Parser)]
#[command(name = "flvt", version, about = "Personal note vault for source code")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add(AddCommand),
    List(ListCommand),
    Update(UpdateCommand),
    Delete(DeleteCommand),
    Search(SearchCommand),
    Repair(RepairCommand),
    Doctor(HealthCommand),
    Health(HealthCommand),
    Stats(StatsCommand),
    Explorer(ExplorerCommand),
    Gitignore(GitignoreCommand),
}
