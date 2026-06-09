use clap::Args;

#[derive(Debug, Args)]
pub struct RepairCommand {
    #[arg(long)]
    pub apply: bool,
}
