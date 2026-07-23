use std::path::{Path, PathBuf};

use crate::{
    FrilVaultResult,
    note::{NoteFile, NoteFileRecord, NoteRepository},
    runtime::NoteCache,
    workspace::{WorkspaceIndex, WorkspaceIndexRepository},
};

/// Runtime container for FrilVault.
///
/// `VaultContext` owns shared runtime resources such as repositories, the note cache,
/// and workspace index access. Services should use it instead of touching repositories
/// directly so cache invalidation stays consistent.
///
/// FrilVault 런타임 컨테이너입니다.
///
/// `VaultContext`는 저장소, note cache, workspace index 접근 같은 공유 런타임
/// 리소스를 소유합니다. 캐시 무효화 일관성을 위해 서비스는 저장소를 직접
/// 건드리지 않고 이 타입을 사용해야 합니다.
#[derive(Clone)]
pub struct VaultContext {
    pub note_repository: NoteRepository,
    pub workspace_index_repository: WorkspaceIndexRepository,
    pub note_cache: NoteCache,
}

impl VaultContext {
    pub fn new(
        note_repository: NoteRepository,
        workspace_index_repository: WorkspaceIndexRepository,
    ) -> Self {
        Self {
            note_repository,
            workspace_index_repository,
            note_cache: NoteCache::default(),
        }
    }

    pub fn load_notes(&mut self, source_file: &Path) -> FrilVaultResult<NoteFile> {
        // Serve cached note files when possible to avoid repeated JSON reads.
        // 가능하면 캐시된 note file을 제공해 JSON 반복 읽기를 줄입니다.
        if let Some(cached) = self.note_cache.get(source_file) {
            return Ok(cached.clone());
        }

        // 2. REPOSITORY LOAD
        let note_file = self.note_repository.load_by_source_file(source_file)?;

        // Populate the cache after a repository miss.
        // repository miss 이후 cache를 채웁니다.
        self.note_cache
            .insert(source_file.to_path_buf(), note_file.clone());

        Ok(note_file)
    }

    pub fn preload_notes(&mut self, source_file: &Path) -> FrilVaultResult<()> {
        if self.note_cache.contains(source_file) {
            return Ok(());
        }

        let note_file = self.note_repository.load_by_source_file(source_file)?;
        self.note_cache.insert(source_file.to_path_buf(), note_file);

        Ok(())
    }

    pub fn invalidate_notes(&mut self, source_file: &Path) {
        // Drop cached note JSON after writes or external vault changes.
        // 쓰기 또는 외부 vault 변경 후 캐시된 note JSON을 제거합니다.
        self.note_cache.invalidate(source_file);
    }

    pub fn clear_notes_cache(&mut self) {
        self.note_cache.clear();
    }

    pub fn rebuild_index(&self) -> FrilVaultResult<WorkspaceIndex> {
        self.workspace_index_repository.rebuild()
    }

    pub fn load_index(&self) -> FrilVaultResult<WorkspaceIndex> {
        self.workspace_index_repository.load_and_refresh_exists()
    }

    pub fn sync_index_for_source_file(&self, source_file: &Path) -> FrilVaultResult<()> {
        let note_file = self.note_repository.load_by_source_file(source_file)?;
        let source_file = source_file.to_string_lossy();

        self.workspace_index_repository
            .sync_source_file(source_file.as_ref(), note_file.notes.len())
    }

    pub fn contains_cached_notes(&self, source_file: &Path) -> bool {
        self.note_cache.contains(source_file)
    }

    pub fn list_all_note_files(&mut self) -> FrilVaultResult<Vec<NoteFileRecord>> {
        let index = self.load_index()?;
        let mut records = Vec::new();

        for indexed_file in index.files {
            if indexed_file.note_count == 0 {
                continue;
            }

            let source_file = PathBuf::from(&indexed_file.source_file);
            let note_file = self.load_notes(&source_file)?;

            records.push(NoteFileRecord {
                source_file,
                note_file,
            });
        }

        Ok(records)
    }

    pub fn scan_workspace_files(&self) -> FrilVaultResult<Vec<String>> {
        let mut files = Vec::new();

        self.collect_workspace_files(self.workspace_index_repository.workspace_root(), &mut files)?;

        Ok(files)
    }

    pub fn resolve_note_path(&self, source_file: &str) -> PathBuf {
        self.note_repository.resolve_note_path(source_file)
    }

    pub fn normalize_source_file(&self, source_file: &Path) -> FrilVaultResult<PathBuf> {
        if source_file.is_absolute() {
            return source_file
                .strip_prefix(self.workspace_index_repository.workspace_root())
                .map(|path| path.to_path_buf())
                .map_err(|_| crate::FrilVaultError::SourcePathOutsideWorkspace);
        }

        Ok(source_file.to_path_buf())
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
