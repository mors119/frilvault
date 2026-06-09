use clap::Args;

#[derive(Debug, Args)]
pub struct ListCommand {
    #[arg(long)]
    pub file: String,

    #[arg(long)]
    pub json: bool,
}
