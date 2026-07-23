//! Application services for note operations.
//!
//! This module contains high-level note workflows such as CRUD operations,
//! searching, attachments, and URI resolution.
//!
//! Services access storage through `VaultContext` rather than repositories directly.
//!
//! CRUD, 검색, 첨부, URI 해석 같은 고수준 note 워크플로를 제공하는
//! 애플리케이션 서비스 모듈입니다.
//!
//! 서비스는 저장소를 직접 사용하지 않고 `VaultContext`를 통해 접근합니다.

use std::path::{Path, PathBuf};

use chrono::Utc;
use uuid::Uuid;

use crate::{
    AddNoteRequest, AttachmentRepository, FrilVaultError, FrilVaultResult, NoteAnchor,
    NoteAttachment, NoteQuery, NoteView, SymbolKind, UpdateNoteRequest,
    note::Note,
    runtime::VaultContext,
    symbol::SymbolResolver,
    workspace::{PathResolver, read_source_file_content},
};

/// Application service responsible for note operations.
///
/// Coordinates repositories, caching, symbol resolution, and workspace index updates.
///
/// 노트 연산을 담당하는 애플리케이션 서비스입니다.
///
/// 저장소, 캐시, symbol 해석, workspace index 갱신을 조율합니다.
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

    /// Creates and persists a new note for the requested source file.
    ///
    /// Updates the workspace index note count for that file.
    ///
    /// # Errors
    ///
    /// Returns repository or index errors when vault JSON cannot be written.
    ///
    /// 요청한 source file에 새 note를 생성하고 저장합니다.
    ///
    /// 해당 파일의 workspace index note count를 갱신합니다.
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

    /// Returns note views for one source file, including resolved symbol locations.
    ///
    /// 하나의 source file에 대한 note view를 반환하며, symbol 위치 해석 결과를 포함합니다.
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

    /// Applies optional file, keyword, and tag filters to note views.
    ///
    /// 선택적 file, keyword, tag 필터를 note view에 적용합니다.
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

    /// Deletes a note and removes any stored attachments for its id.
    ///
    /// # Errors
    ///
    /// Returns `NoteNotFound` when the id is absent from the source note file.
    ///
    /// note를 삭제하고 해당 id의 저장된 첨부를 함께 제거합니다.
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

    /// Updates note content and optionally tags.
    ///
    /// When `expected_updated_at` is provided, the update is rejected if another
    /// writer changed the note first.
    ///
    /// # Errors
    ///
    /// Returns `NoteNotFound` or `ConcurrentModification` when applicable.
    ///
    /// note content와 선택적으로 tags를 수정합니다.
    ///
    /// `expected_updated_at`가 주어지면 다른 writer가 먼저 수정한 경우 거부됩니다.
    pub fn update_note(
        &mut self,
        source_file: impl AsRef<Path>,
        note_id: Uuid,
        request: UpdateNoteRequest,
    ) -> FrilVaultResult<Note> {
        let source_file = source_file.as_ref();

        let mut notes = self.load_notes(source_file)?;

        let note = notes
            .iter_mut()
            .find(|note| note.id == note_id)
            .ok_or(FrilVaultError::NoteNotFound(note_id))?;

        if let Some(expected) = request.expected_updated_at
            && note.updated_at != expected
        {
            return Err(FrilVaultError::ConcurrentModification(note_id));
        }

        note.content = request.content;

        if let Some(tags) = request.tags {
            note.tags = tags;
        }

        note.updated_at = Utc::now();

        let updated = note.clone();
        self.save_notes(source_file, notes)?;

        self.vault_context.sync_index_for_source_file(source_file)?;

        Ok(updated)
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

    /// Returns the absolute workspace root associated with this service.
    ///
    /// 이 서비스와 연결된 절대 workspace root를 반환합니다.
    pub fn workspace_root(&self) -> PathBuf {
        self.vault_context
            .workspace_index_repository
            .workspace_root()
            .to_path_buf()
    }

    /// Loads the workspace index and refreshes source-file existence flags.
    ///
    /// workspace index를 불러오고 source file 존재 여부 플래그를 갱신합니다.
    pub fn load_workspace_index(&self) -> FrilVaultResult<crate::workspace::WorkspaceIndex> {
        self.vault_context.load_index()
    }

    /// Finds a note by stable id across all indexed source files.
    ///
    /// # Errors
    ///
    /// Returns `NoteNotFound` when no note with the id exists in the workspace.
    ///
    /// 인덱스된 모든 source file에서 stable id로 note를 찾습니다.
    pub fn find_note_by_id(&mut self, note_id: Uuid) -> FrilVaultResult<NoteView> {
        self.all_note_views()?
            .into_iter()
            .find(|view| view.note.id == note_id)
            .ok_or(FrilVaultError::NoteNotFound(note_id))
    }

    /// Resolves a stable note URI into a current `NoteView`.
    ///
    /// # Errors
    ///
    /// Returns URI, workspace, stale-note, or unresolved-anchor errors when validation fails.
    ///
    /// stable note URI를 현재 `NoteView`로 해석합니다.
    pub fn resolve_note_uri(&mut self, uri: &str) -> FrilVaultResult<NoteView> {
        crate::uri::NoteUriResolver::resolve(self, uri)
    }

    /// Serializes a versioned note URI for the current workspace root.
    ///
    /// 현재 workspace root 기준 versioned note URI를 직렬화합니다.
    pub fn note_uri(&self, note_id: Uuid) -> FrilVaultResult<String> {
        crate::uri::NoteUriResolver::serialize(note_id, &self.workspace_root())
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
