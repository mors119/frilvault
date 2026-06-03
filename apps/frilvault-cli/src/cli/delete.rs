use clap::Args;

#[derive(Debug, Args)]
pub struct DeleteCommand {
    #[arg(long)]
    pub file: String,

    #[arg(long)]
    pub id: String,
}
