use serde::{Deserialize, Serialize};

use crate::workspace::IndexedFile;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceIndex {
    pub version: u32,

    pub files: Vec<IndexedFile>,
}

impl Default for WorkspaceIndex {
    fn default() -> Self {
        Self {
            version: 1,
            files: Vec::new(),
        }
    }
}
