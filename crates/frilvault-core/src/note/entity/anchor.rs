use serde::{Deserialize, Serialize};

// TODO: Regex parser
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum NoteAnchor {
    Line(LineAnchor),
    Symbol(SymbolAnchor),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LineAnchor {
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SymbolAnchor {
    pub name: String,
    pub kind: SymbolKind,
    pub signature: Option<String>,
    pub line_hint: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SymbolKind {
    Function,
    Struct,
    Enum,
    Trait,
    Impl,
    Method,
    Unknown,
}
