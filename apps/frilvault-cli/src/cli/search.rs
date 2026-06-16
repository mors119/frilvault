use clap::{Args, ValueEnum};

#[derive(Debug, Args)]
pub struct SearchCommand {
    pub keyword: Option<String>,

    #[arg(long)]
    pub file: Option<String>,

    #[arg(long, value_enum)]
    pub format: Option<SearchFormatArg>,

    #[arg(long, hide = true)]
    pub json: bool,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum SearchFormatArg {
    Text,
    Json,
}
