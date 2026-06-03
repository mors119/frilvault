use clap::Args;

#[derive(Debug, Args)]
pub struct ListCommand {
    #[arg(long)]
    pub file: String,
}
