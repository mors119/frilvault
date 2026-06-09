use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::note::Note;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteView {
    pub source_file: PathBuf,

    pub note: Note,
}
