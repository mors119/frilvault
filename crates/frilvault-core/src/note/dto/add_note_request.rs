use std::path::PathBuf;

use crate::note::NoteAnchor;

#[derive(Debug, Clone)]
pub struct AddNoteRequest {
    /// source_file: the path to the source file where the note is located.
    pub source_file: PathBuf,

    /// anchor: the position where the note is anchored. it can be either line-based or symbol-based.
    pub anchor: NoteAnchor,

    /// content: the content of the note to be saved.
    pub content: String,
}
