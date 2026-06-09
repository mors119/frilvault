use serde::{Deserialize, Serialize};

use crate::note::Note;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct NoteFile {
    pub notes: Vec<Note>,
}
