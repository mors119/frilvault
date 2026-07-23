use clap::Args;

use super::format::FormatArg;

#[derive(Debug, Args)]
pub struct ResolveUriCommand {
    #[arg(long)]
    pub uri: String,

    #[arg(long, value_enum)]
    pub format: Option<FormatArg>,
}
