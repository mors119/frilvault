use std::path::{Path, PathBuf};

use crate::{
    FrilVaultResult,
    note::NoteFile,
    runtime::NoteCache,
    storage::{NoteFileRecord, YamlNoteRepository},
    workspace::{WorkspaceIndex, WorkspaceIndexRepository},
};

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
#[derive(Clone)]
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

    pub fn rebuild_index(&self) -> FrilVaultResult<WorkspaceIndex> {
        self.workspace_index_repository.rebuild()
    }

    pub fn contains_cached_notes(&self, source_file: &Path) -> bool {
        self.note_cache.contains(source_file)
    }

    pub fn list_all_note_files(&self) -> FrilVaultResult<Vec<NoteFileRecord>> {
        self.note_repository.list_all_note_files()
    }

    pub fn scan_workspace_files(&self) -> FrilVaultResult<Vec<String>> {
        let mut files = Vec::new();

        self.collect_workspace_files(self.workspace_index_repository.workspace_root(), &mut files)?;

        Ok(files)
    }

    pub fn resolve_note_path(&self, source_file: &str) -> PathBuf {
        self.note_repository.resolve_note_path(source_file)
    }

    fn collect_workspace_files(
        &self,
        directory: &Path,
        files: &mut Vec<String>,
    ) -> FrilVaultResult<()> {
        for entry in std::fs::read_dir(directory)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let path = entry.path();

            if file_type.is_symlink() {
                continue;
            }

            if file_type.is_dir() {
                if path.file_name().and_then(|n| n.to_str()) == Some(".vault") {
                    continue;
                }

                self.collect_workspace_files(&path, files)?;
                continue;
            }

            let relative = path
                .strip_prefix(self.workspace_index_repository.workspace_root())
                .map_err(|_| crate::FrilVaultError::SourcePathOutsideWorkspace)?;

            files.push(relative.to_string_lossy().to_string());
        }

        Ok(())
    }
}
