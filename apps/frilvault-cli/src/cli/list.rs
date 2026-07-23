use clap::Args;

use super::format::FormatArg;

#[derive(Debug, Args)]
pub struct ListCommand {
    #[arg(long)]
    pub file: String,

    #[arg(long, value_enum)]
    pub format: Option<FormatArg>,
}
