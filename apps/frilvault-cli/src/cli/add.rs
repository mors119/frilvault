use clap::{Args, ValueEnum};

use super::format::FormatArg;

#[derive(Debug, Args)]
pub struct AddCommand {
    #[arg(long)]
    pub file: String,

    #[arg(long, conflicts_with = "symbol")]
    pub line: Option<u32>,

    #[arg(long, requires = "line")]
    pub column: Option<u32>,

    #[arg(long, conflicts_with = "line")]
    pub symbol: Option<String>,

    #[arg(long, requires = "symbol", default_value = "unknown")]
    pub kind: SymbolKindArg,

    #[arg(long)]
    pub signature: Option<String>,

    #[arg(long)]
    pub line_hint: Option<u32>,

    #[arg(long)]
    pub content: String,

    #[arg(long = "tag")]
    pub tags: Vec<String>,

    #[arg(long, value_enum)]
    pub format: Option<FormatArg>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum SymbolKindArg {
    Function,
    Struct,
    Enum,
    Trait,
    Impl,
    Method,
    Unknown,
}
