use serde::{Deserialize, Serialize};

use crate::note::SymbolKind;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SymbolAnchor {
    pub name: String,
    pub kind: SymbolKind,
    pub signature: Option<String>,
    pub line_hint: Option<u32>,
}
