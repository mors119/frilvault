use std::path::Path;

use chrono::Utc;
use uuid::Uuid;

use crate::{
    FrilVaultError, FrilVaultResult, NoteAnchor, NoteView,
    note::{AddNoteInput, Note},
};

use crate::note::NoteRepository;

pub struct NoteService<R>
where
    R: NoteRepository,
{
    repository: R,
}

impl<R> NoteService<R>
where
    R: NoteRepository,
{
    pub fn new(repository: R) -> Self {
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
        let source_file = input.source_file.clone();
        let note = Note::new(input);

        self.repository.append_note(&source_file, &note)?;

        Ok(note)
    }

    pub fn list_notes(&self, source_file: impl AsRef<Path>) -> FrilVaultResult<Vec<NoteView>> {
        let source_file = source_file.as_ref();

        let notes = self.load_notes(source_file)?;

        Ok(notes
            .into_iter()
            .map(|note| NoteView {
                source_file: source_file.to_path_buf(),

                note,
            })
            .collect())
    }

    pub fn delete_note(&self, source_file: impl AsRef<Path>, note_id: Uuid) -> FrilVaultResult<()> {
        let source_file = source_file.as_ref();

        let mut notes = self.load_notes(source_file)?;

        let before = notes.len();

        notes.retain(|note| note.id != note_id);

        if notes.len() == before {
            return Err(FrilVaultError::NoteNotFound(note_id));
        }

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

        let note = notes
            .iter_mut()
            .find(|note| note.id == note_id)
            .ok_or(FrilVaultError::NoteNotFound(note_id))?;

        note.content = content;
        note.updated_at = Utc::now();

        self.save_notes(source_file, notes)?;

        Ok(())
    }

    pub fn search_notes(&self, keyword: &str) -> FrilVaultResult<Vec<NoteView>> {
        let records = self.repository.list_all_note_files()?;

        let keyword = keyword.to_lowercase();

        let mut results = Vec::new();

        for record in records {
            for note in record.note_file.notes {
                let content_match = note.content.to_lowercase().contains(&keyword);

                let symbol_match = match &note.anchor {
                    NoteAnchor::Symbol(anchor) => anchor.name.to_lowercase().contains(&keyword),

                    _ => false,
                };

                if content_match || symbol_match {
                    results.push(NoteView {
                        source_file: record.source_file.clone(),
                        note,
                    });
                }
            }
        }

        Ok(results)
    }
}
