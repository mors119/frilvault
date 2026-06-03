use clap::Args;

#[derive(Debug, Args)]
pub struct UpdateCommand {
    #[arg(long)]
    pub file: String,

    #[arg(long)]
    pub id: String,

    #[arg(long)]
    pub content: String,
}
