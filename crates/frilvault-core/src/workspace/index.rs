use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceIndex {
    pub version: u32,

    pub files: Vec<IndexedFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexedFile {
    pub source_file: String,

    pub note_count: usize,

    pub exists: bool,
}

impl Default for WorkspaceIndex {
    fn default() -> Self {
        Self {
            version: 1,
            files: Vec::new(),
        }
    }
}
