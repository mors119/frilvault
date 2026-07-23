use clap::{Args, Subcommand};

use super::format::FormatArg;

#[derive(Debug, Args)]
pub struct GitignoreCommand {
    #[command(subcommand)]
    pub action: GitignoreAction,
}

#[derive(Debug, Subcommand)]
pub enum GitignoreAction {
    Check(GitignoreCheckCommand),
    Add,
}

#[derive(Debug, Args)]
pub struct GitignoreCheckCommand {
    #[arg(long, value_enum)]
    pub format: Option<FormatArg>,
}
