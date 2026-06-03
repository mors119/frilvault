use clap::Args;

#[derive(Debug, Args)]
pub struct SearchCommand {
    pub keyword: String,
}
