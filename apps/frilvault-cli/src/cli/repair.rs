use clap::Args;

use super::format::FormatArg;

#[derive(Debug, Args)]
pub struct RepairCommand {
    #[arg(long)]
    pub apply: bool,

    #[arg(long, value_enum)]
    pub format: Option<FormatArg>,
}
