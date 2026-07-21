use clap::{Args, ValueEnum};

#[derive(Debug, Args)]
pub struct HealthCommand {
    #[arg(long, value_enum)]
    pub format: Option<HealthFormatArg>,

    #[arg(long, hide = true)]
    pub json: bool,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum HealthFormatArg {
    Text,
    Json,
}
