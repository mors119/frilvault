use std::path::Path;

use crate::{
    FrilVaultResult,
    note::{AddNoteInput, Note},
    storage::YamlNoteRepository,
};

pub struct NoteService {
    repository: YamlNoteRepository,
}

impl NoteService {
    pub fn new(repository: YamlNoteRepository) -> Self {
        Self { repository }
    }

    pub fn add_note(&self, input: AddNoteInput) -> FrilVaultResult<Note> {
        let note = Note::new(input);

        self.repository.append_note(&note)?;

        Ok(note)
    }

    pub fn list_notes(&self, source_file: impl AsRef<Path>) -> FrilVaultResult<Vec<Note>> {
        let note_file = self.repository.load_by_source_file(source_file.as_ref())?;

        Ok(note_file.notes)
    }
}
