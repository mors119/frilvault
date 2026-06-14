use clap::{Args, ValueEnum};

#[derive(Debug, Args)]
pub struct ListCommand {
    #[arg(long)]
    pub file: String,

    #[arg(long, value_enum)]
    pub format: Option<ListFormatArg>,

    #[arg(long, hide = true)]
    pub json: bool,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ListFormatArg {
    Text,
    Json,
}
