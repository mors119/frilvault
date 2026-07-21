use clap::{Args, ValueEnum};

#[derive(Debug, Args)]
pub struct StatsCommand {
    #[arg(long, value_enum)]
    pub format: Option<StatsFormatArg>,

    #[arg(long, hide = true)]
    pub json: bool,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum StatsFormatArg {
    Text,
    Json,
}
