use clap::Args;

use super::format::FormatArg;

#[derive(Debug, Args)]
pub struct SyncCommand {
    #[arg(long, help = "Sync only note directory changes")]
    pub notes_only: bool,

    #[arg(long, help = "Sync only source file changes")]
    pub sources_only: bool,

    #[arg(long, value_enum)]
    pub format: Option<FormatArg>,
}
