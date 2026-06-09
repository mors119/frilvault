use serde::{Deserialize, Serialize};

use crate::note::{LineAnchor, SymbolAnchor};

// TODO: Regex parser
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum NoteAnchor {
    Line(LineAnchor),
    Symbol(SymbolAnchor),
}
