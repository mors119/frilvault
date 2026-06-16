use crate::{FrilVaultResult, NoteCache, NoteFile, WorkspaceIndexRepository, YamlNoteRepository};

pub struct VaultContext {
    pub note_repository: YamlNoteRepository,
    pub workspace_index_repository: WorkspaceIndexRepository,
    pub cache: NoteCache,
}

impl VaultContext {
    pub fn new(
        note_repository: YamlNoteRepository,
        workspace_index_repository: WorkspaceIndexRepository,
    ) -> Self {
        Self {
            note_repository,
            workspace_index_repository,
            cache: NoteCache::default(),
        }
    }

    pub fn load_notes(&mut self, source_file: &std::path::Path) -> FrilVaultResult<NoteFile> {
        let key = source_file.to_path_buf();

        // 1. CACHE HIT
        if let Some(cached) = self.cache.get(&key) {
            return Ok(cached.clone());
        }

        // 2. REPOSITORY LOAD
        let note_file = self.note_repository.load_by_source_file(source_file)?;

        // 3. CACHE STORE
        self.cache.insert(key, note_file.clone());

        Ok(note_file)
    }

    pub fn invalidate_notes(&mut self, source_file: &std::path::Path) {
        self.cache.invalidate(&source_file.to_path_buf());
    }
}
