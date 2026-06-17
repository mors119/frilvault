use std::path::Path;

use crate::{FrilVaultResult, NoteCache, NoteFile, WorkspaceIndexRepository, YamlNoteRepository};

/// Runtime container for FrilVault.
///
/// VaultContext owns shared runtime resources:
///
/// - repositories
/// - caches
/// - indexes
///
/// Services should use VaultContext instead of
/// accessing repositories directly.
pub struct VaultContext {
    pub note_repository: YamlNoteRepository,
    pub workspace_index_repository: WorkspaceIndexRepository,
    pub note_cache: NoteCache,
}

impl VaultContext {
    pub fn new(
        note_repository: YamlNoteRepository,
        workspace_index_repository: WorkspaceIndexRepository,
    ) -> Self {
        Self {
            note_repository,
            workspace_index_repository,
            note_cache: NoteCache::default(),
        }
    }

    pub fn load_notes(&mut self, source_file: &Path) -> FrilVaultResult<NoteFile> {
        // 1. CACHE HIT
        if let Some(cached) = self.note_cache.get(source_file) {
            return Ok(cached.clone());
        }

        // 2. REPOSITORY LOAD
        let note_file = self.note_repository.load_by_source_file(source_file)?;

        // 3. CACHE STORE
        self.note_cache
            .insert(source_file.to_path_buf(), note_file.clone());

        Ok(note_file)
    }

    pub fn invalidate_notes(&mut self, source_file: &Path) {
        self.note_cache.invalidate(source_file);
    }

    pub fn contains_cached_notes(&self, source_file: &Path) -> bool {
        self.note_cache.contains(source_file)
    }
}
