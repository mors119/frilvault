use clap::{Parser, Subcommand};

pub mod add;
pub mod delete;
pub mod list;
pub mod update;

use add::AddCommand;
use delete::DeleteCommand;
use list::ListCommand;
use update::UpdateCommand;

#[derive(Parser)]
#[command(
    name = "frilvault",
    version,
    about = "Personal note vault for source code"
)]
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
}
