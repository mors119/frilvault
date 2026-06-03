use clap::Args;

#[derive(Debug, Args)]
pub struct AddCommand {
    #[arg(long)]
    pub file: String,

    #[arg(long)]
    pub line: u32,

    #[arg(long)]
    pub column: u32,

    #[arg(long)]
    pub content: String,
}
