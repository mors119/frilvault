use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexedFile {
    pub source_file: String,

    pub note_count: usize,

    pub exists: bool,
}
