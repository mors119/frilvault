use std::path::PathBuf;

use crate::note::NoteFile;

#[derive(Debug, Clone)]
pub struct NoteFileRecord {
    pub source_file: PathBuf,
    pub note_file: NoteFile,
}
