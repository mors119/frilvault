use clap::Args;

use super::format::FormatArg;

#[derive(Debug, Args)]
pub struct ExplorerCommand {
    #[arg(long, value_enum)]
    pub format: Option<FormatArg>,
}
