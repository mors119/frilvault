//! Application services for note operations.
//!
//! This module contains high-level note workflows
//! such as CRUD operations and searching.
//!
//! Services should access storage through
//! VaultContext rather than directly interacting
//! with repositories.

use std::path::Path;

use chrono::Utc;
use uuid::Uuid;

use crate::{
    AddNoteRequest, FrilVaultError, FrilVaultResult, NoteAnchor, NoteView, note::Note,
    runtime::VaultContext,
};

/// Application service responsible for note operations.
///
/// Coordinates repositories, caching,
/// and future runtime behaviors.
pub struct NoteService {
    vault_context: VaultContext,
}

impl NoteService {
    pub fn new(vault_context: VaultContext) -> Self {
        Self { vault_context }
    }

    fn load_notes(&mut self, source_file: impl AsRef<Path>) -> FrilVaultResult<Vec<Note>> {
        Ok(self.vault_context.load_notes(source_file.as_ref())?.notes)
    }

    fn save_notes(
        &mut self,
        source_file: impl AsRef<Path>,
        notes: Vec<Note>,
    ) -> FrilVaultResult<()> {
        let source_file = source_file.as_ref();

        self.vault_context
            .note_repository
            .replace_notes(source_file.as_ref(), notes)?;

        self.vault_context.invalidate_notes(source_file.as_ref());

        Ok(())
    }

    pub fn add_note(&mut self, input: AddNoteRequest) -> FrilVaultResult<Note> {
        let source_file = input.source_file.clone();
        let note = Note::new(input);

        self.vault_context
            .note_repository
            .append_note(&source_file, &note)?;

        self.vault_context.invalidate_notes(&source_file);

        Ok(note)
    }

    pub fn list_notes(&mut self, source_file: impl AsRef<Path>) -> FrilVaultResult<Vec<NoteView>> {
        let source_file = source_file.as_ref();

        let notes = self.vault_context.load_notes(source_file)?;

        Ok(notes
            .notes
            .into_iter()
            .map(|note| NoteView {
                source_file: source_file.to_path_buf(),
                note,
            })
            .collect())
    }

    pub fn delete_note(
        &mut self,
        source_file: impl AsRef<Path>,
        note_id: Uuid,
    ) -> FrilVaultResult<()> {
        let source_file = source_file.as_ref();

        let mut notes = self.load_notes(source_file)?;

        let before = notes.len();

        notes.retain(|note| note.id != note_id);

        if notes.len() == before {
            return Err(FrilVaultError::NoteNotFound(note_id));
        }

        self.save_notes(source_file, notes)?;

        self.vault_context.invalidate_notes(source_file);

        Ok(())
    }

    pub fn update_note(
        &mut self,
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

        self.vault_context.invalidate_notes(source_file);

        Ok(())
    }

    fn all_note_views(&self) -> FrilVaultResult<Vec<NoteView>> {
        let records = self.vault_context.note_repository.list_all_note_files()?;

        let mut results = Vec::new();

        for record in records {
            for note in record.note_file.notes {
                results.push(NoteView {
                    source_file: record.source_file.clone(),
                    note,
                });
            }
        }

        Ok(results)
    }

    pub fn search_notes(&mut self, keyword: &str) -> FrilVaultResult<Vec<NoteView>> {
        let keyword = keyword.to_lowercase();

        Ok(self
            .all_note_views()?
            .into_iter()
            .filter(|view| {
                let content_match = view.note.content.to_lowercase().contains(&keyword);

                let symbol_match = match &view.note.anchor {
                    NoteAnchor::Symbol(anchor) => anchor.name.to_lowercase().contains(&keyword),
                    _ => false,
                };

                content_match || symbol_match
            })
            .collect())
    }

    pub fn search_by_symbol(&mut self, symbol: &str) -> FrilVaultResult<Vec<NoteView>> {
        let symbol = symbol.to_lowercase();

        Ok(self
            .all_note_views()?
            .into_iter()
            .filter(|view| {
                matches!(
                    &view.note.anchor,
                    NoteAnchor::Symbol(anchor)
                        if anchor.name.to_lowercase().contains(&symbol)
                )
            })
            .collect())
    }

    pub fn list_symbol_notes(
        &mut self,
        source_file: impl AsRef<Path>,
    ) -> FrilVaultResult<Vec<NoteView>> {
        Ok(self
            .list_notes(source_file)?
            .into_iter()
            .filter(|view| matches!(view.note.anchor, NoteAnchor::Symbol(_)))
            .collect())
    }

    pub fn find_symbol_note(
        &mut self,
        source_file: impl AsRef<Path>,
        symbol: &str,
    ) -> FrilVaultResult<Option<NoteView>> {
        let symbol = symbol.to_lowercase();

        Ok(self
            .list_symbol_notes(source_file)?
            .into_iter()
            .find(|view| match &view.note.anchor {
                NoteAnchor::Symbol(anchor) => anchor.name.to_lowercase() == symbol,
                _ => false,
            }))
    }
}
