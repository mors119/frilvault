use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LineAnchor {
    pub line: u32,
    pub column: u32,
}
