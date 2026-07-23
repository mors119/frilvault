use serde::{Deserialize, Serialize};

/// Current source location resolved for a symbol anchor.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResolvedSymbol {
    pub line: u32,
    pub column: u32,
}
