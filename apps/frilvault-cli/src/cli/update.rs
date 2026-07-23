use clap::Args;

use super::format::FormatArg;

#[derive(Debug, Args)]
pub struct UpdateCommand {
    #[arg(long)]
    pub file: String,

    #[arg(long)]
    pub id: String,

    #[arg(long)]
    pub content: String,

    #[arg(long = "tag")]
    pub tags: Vec<String>,

    #[arg(long)]
    pub expected_updated_at: Option<String>,

    #[arg(long, value_enum)]
    pub format: Option<FormatArg>,
}
