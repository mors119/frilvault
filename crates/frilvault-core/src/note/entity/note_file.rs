use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::note::Note;

#[derive(Debug, Clone)]
pub struct NoteFileRecord {
    pub source_file: PathBuf,
    pub note_file: NoteFile,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct NoteFile {
    pub notes: Vec<Note>,
}
