use clap::Args;

use super::format::FormatArg;

#[derive(Debug, Args)]
pub struct AttachCommand {
    #[arg(long)]
    pub file: String,

    #[arg(long)]
    pub id: String,

    #[arg(long)]
    pub image: String,

    #[arg(long, value_enum)]
    pub format: Option<FormatArg>,
}
