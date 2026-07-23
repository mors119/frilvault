use clap::Args;

use super::format::FormatArg;

#[derive(Debug, Args)]
pub struct SearchCommand {
    pub keyword: Option<String>,

    #[arg(long)]
    pub file: Option<String>,

    #[arg(long)]
    pub tag: Option<String>,

    #[arg(long, value_enum)]
    pub format: Option<FormatArg>,
}
