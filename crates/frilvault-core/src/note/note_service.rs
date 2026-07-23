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
    AddNoteRequest, AttachmentRepository, FrilVaultError, FrilVaultResult, NoteAnchor,
    NoteAttachment, NoteQuery, NoteView, SymbolKind,
    note::Note,
    runtime::VaultContext,
    symbol::SymbolResolver,
    workspace::{PathResolver, read_source_file_content},
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

        self.vault_context
            .sync_index_for_source_file(&source_file)?;

        Ok(note)
    }

    pub fn list_notes(&mut self, source_file: impl AsRef<Path>) -> FrilVaultResult<Vec<NoteView>> {
        self.query_notes(&NoteQuery {
            source_file: Some(source_file.as_ref().to_path_buf()),
            keyword: None,
            tag: None,
        })
    }

    pub fn search_notes_by_file(
        &mut self,
        source_file: impl AsRef<Path>,
    ) -> FrilVaultResult<Vec<NoteView>> {
        self.query_notes(&NoteQuery {
            source_file: Some(source_file.as_ref().to_path_buf()),
            keyword: None,
            tag: None,
        })
    }

    pub fn query_notes(&mut self, query: &NoteQuery) -> FrilVaultResult<Vec<NoteView>> {
        let mut results = if let Some(source_file) = &query.source_file {
            self.note_views_for_source_file(source_file)?
        } else if query.keyword.is_some() || query.tag.is_some() {
            self.all_note_views()?
        } else {
            Vec::new()
        };

        if let Some(tag) = &query.tag {
            let tag = tag.to_lowercase();
            results.retain(|view| {
                view.note
                    .tags
                    .iter()
                    .any(|note_tag| note_tag.to_lowercase() == tag)
            });
        }

        if let Some(keyword) = &query.keyword {
            let keyword = keyword.to_lowercase();
            results.retain(|view| note_matches_keyword(view, &keyword));
        }

        Ok(results)
    }

    fn note_views_for_source_file(
        &mut self,
        source_file: impl AsRef<Path>,
    ) -> FrilVaultResult<Vec<NoteView>> {
        let source_file = self
            .vault_context
            .normalize_source_file(source_file.as_ref())?;
        let notes = self.vault_context.load_notes(&source_file)?;

        Ok(notes
            .notes
            .into_iter()
            .map(|note| self.build_note_view(&source_file, note))
            .collect())
    }

    pub fn preload_notes(&mut self, source_file: impl AsRef<Path>) -> FrilVaultResult<()> {
        self.vault_context.preload_notes(source_file.as_ref())
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

        self.attachment_repository().remove_all_for_note(note_id)?;

        self.save_notes(source_file, notes)?;

        self.vault_context.sync_index_for_source_file(source_file)?;

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

        self.vault_context.sync_index_for_source_file(source_file)?;

        Ok(())
    }

    pub fn attach_image(
        &mut self,
        source_file: impl AsRef<Path>,
        note_id: Uuid,
        image_path: impl AsRef<Path>,
    ) -> FrilVaultResult<NoteAttachment> {
        let source_file = source_file.as_ref();
        let mut notes = self.load_notes(source_file)?;

        let note = notes
            .iter_mut()
            .find(|note| note.id == note_id)
            .ok_or(FrilVaultError::NoteNotFound(note_id))?;

        let attachment = self
            .attachment_repository()
            .store(note_id, image_path.as_ref())?;

        note.attachments.push(attachment.clone());
        note.updated_at = Utc::now();

        self.save_notes(source_file, notes)?;
        self.vault_context.sync_index_for_source_file(source_file)?;

        Ok(attachment)
    }

    pub fn detach_image(
        &mut self,
        source_file: impl AsRef<Path>,
        note_id: Uuid,
        attachment_id: Uuid,
    ) -> FrilVaultResult<()> {
        let source_file = source_file.as_ref();
        let mut notes = self.load_notes(source_file)?;

        let note = notes
            .iter_mut()
            .find(|note| note.id == note_id)
            .ok_or(FrilVaultError::NoteNotFound(note_id))?;

        let attachment_index = note
            .attachments
            .iter()
            .position(|attachment| attachment.id == attachment_id)
            .ok_or(FrilVaultError::AttachmentNotFound(attachment_id))?;

        let attachment = note.attachments.remove(attachment_index);
        note.updated_at = Utc::now();

        self.attachment_repository().remove(note_id, &attachment)?;

        self.save_notes(source_file, notes)?;
        self.vault_context.sync_index_for_source_file(source_file)?;

        Ok(())
    }

    fn attachment_repository(&self) -> AttachmentRepository {
        AttachmentRepository::new(PathResolver::new(
            self.vault_context
                .workspace_index_repository
                .workspace_root(),
        ))
    }

    fn all_note_views(&mut self) -> FrilVaultResult<Vec<NoteView>> {
        let records = self.vault_context.list_all_note_files()?;

        let mut results = Vec::new();

        for record in records {
            for note in record.note_file.notes {
                results.push(self.build_note_view(&record.source_file, note));
            }
        }

        Ok(results)
    }

    fn build_note_view(&self, source_file: &Path, note: Note) -> NoteView {
        let resolved = match &note.anchor {
            NoteAnchor::Symbol(anchor) => self.resolve_symbol_anchor(source_file, anchor),
            _ => None,
        };

        NoteView {
            source_file: source_file.to_path_buf(),
            note,
            resolved,
        }
    }

    fn resolve_symbol_anchor(
        &self,
        source_file: &Path,
        anchor: &crate::SymbolAnchor,
    ) -> Option<crate::ResolvedSymbol> {
        let workspace_root = self
            .vault_context
            .workspace_index_repository
            .workspace_root();
        let relative_path = source_file.to_string_lossy();

        read_source_file_content(workspace_root, relative_path.as_ref())
            .and_then(|content| SymbolResolver::resolve(anchor, &content))
    }

    pub fn find_symbol_in_source(
        &self,
        source_file: impl AsRef<Path>,
        symbol: &str,
        kind: SymbolKind,
    ) -> FrilVaultResult<Option<crate::ResolvedSymbol>> {
        let source_file = source_file.as_ref();
        let workspace_root = self
            .vault_context
            .workspace_index_repository
            .workspace_root();
        let relative_path = source_file.to_string_lossy();

        Ok(
            read_source_file_content(workspace_root, relative_path.as_ref())
                .and_then(|content| SymbolResolver::find_by_name(symbol, kind, &content)),
        )
    }

    pub fn search_notes(&mut self, keyword: &str) -> FrilVaultResult<Vec<NoteView>> {
        self.query_notes(&NoteQuery {
            source_file: None,
            keyword: Some(keyword.to_string()),
            tag: None,
        })
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

    pub fn search_by_tag(&mut self, tag: &str) -> FrilVaultResult<Vec<NoteView>> {
        self.query_notes(&NoteQuery {
            source_file: None,
            keyword: None,
            tag: Some(tag.to_string()),
        })
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

fn note_matches_keyword(view: &NoteView, keyword: &str) -> bool {
    let content_match = view.note.content.to_lowercase().contains(keyword);

    let symbol_match = matches!(
        &view.note.anchor,
        NoteAnchor::Symbol(anchor) if anchor.name.to_lowercase().contains(keyword)
    );

    content_match || symbol_match
}
