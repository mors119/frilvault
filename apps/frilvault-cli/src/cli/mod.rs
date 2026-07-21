use clap::{Parser, Subcommand};

pub mod add;
pub mod delete;
pub mod health;
pub mod list;
pub mod repair;
pub mod search;
pub mod stats;
pub mod update;

use add::AddCommand;
use delete::DeleteCommand;
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
}
