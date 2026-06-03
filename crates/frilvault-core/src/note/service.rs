use std::path::Path;

use chrono::Utc;
use uuid::Uuid;

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

    fn load_notes(&self, source_file: impl AsRef<Path>) -> FrilVaultResult<Vec<Note>> {
        Ok(self
            .repository
            .load_by_source_file(source_file.as_ref())?
            .notes)
    }

    fn save_notes(&self, source_file: impl AsRef<Path>, notes: Vec<Note>) -> FrilVaultResult<()> {
        self.repository.replace_notes(source_file.as_ref(), notes)
    }

    pub fn add_note(&self, input: AddNoteInput) -> FrilVaultResult<Note> {
        let note = Note::new(input);

        self.repository.append_note(&note)?;

        Ok(note)
    }

    pub fn list_notes(&self, source_file: impl AsRef<Path>) -> FrilVaultResult<Vec<Note>> {
        self.load_notes(source_file)
    }

    pub fn delete_note(&self, source_file: impl AsRef<Path>, note_id: Uuid) -> FrilVaultResult<()> {
        let source_file = source_file.as_ref();

        let mut notes = self.load_notes(source_file)?;

        notes.retain(|note| note.id != note_id);

        self.repository.replace_notes(source_file, notes)?;

        Ok(())
    }

    pub fn update_note(
        &self,
        source_file: impl AsRef<Path>,
        note_id: Uuid,
        content: String,
    ) -> FrilVaultResult<()> {
        let source_file = source_file.as_ref();

        let mut notes = self.load_notes(source_file)?;

        if let Some(note) = notes.iter_mut().find(|note| note.id == note_id) {
            // TODO: FrilVaultError::NoteNotFound
            note.content = content;
            note.updated_at = Utc::now();
        }

        self.save_notes(source_file, notes)?;

        Ok(())
    }
}
