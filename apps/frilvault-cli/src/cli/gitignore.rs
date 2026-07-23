use clap::{Args, Subcommand, ValueEnum};

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
    pub format: Option<GitignoreFormatArg>,

    #[arg(long, hide = true)]
    pub json: bool,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum GitignoreFormatArg {
    Text,
    Json,
}
