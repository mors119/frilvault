use std::{fs, path::Path};

use crate::{
    FrilVaultResult, RepairSuggestion,
    note::NoteRepository,
    workspace::{FileMove, IndexDiff, IndexedFile, PathResolver, WorkspaceIndex},
};

#[derive(Debug, Clone)]
pub struct WorkspaceIndexRepository {
    path_resolver: PathResolver,
}

impl WorkspaceIndexRepository {
    pub fn new(path_resolver: PathResolver) -> Self {
        Self { path_resolver }
    }

    pub fn workspace_root(&self) -> &Path {
        self.path_resolver.workspace_root()
    }

    pub fn load(&self) -> FrilVaultResult<WorkspaceIndex> {
        let path = self.path_resolver.workspace_index_path();

        if !path.exists() {
            return Ok(WorkspaceIndex::default());
        }

        let content = fs::read_to_string(path)?;

        let index = serde_json::from_str(&content)?;

        Ok(index)
    }

    pub fn save(&self, index: &WorkspaceIndex) -> FrilVaultResult<()> {
        let path = self.path_resolver.workspace_index_path();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string(index)?;

        fs::write(path, json)?;

        Ok(())
    }

    pub fn create_if_missing(&self) -> FrilVaultResult<()> {
        let path = self.path_resolver.workspace_index_path();

        if path.exists() {
            return Ok(());
        }

        self.save(&WorkspaceIndex::default())?;

        Ok(())
    }

    pub fn rebuild(&self) -> FrilVaultResult<WorkspaceIndex> {
        let note_repository = NoteRepository::new(self.path_resolver.clone());

        let records = note_repository.list_all_note_files()?;

        let files = records
            .into_iter()
            .map(|record| {
                let source_file = record.source_file.to_string_lossy().to_string();

                let exists = self
                    .path_resolver
                    .workspace_root()
                    .join(&record.source_file)
                    .exists();

                IndexedFile {
                    source_file,
                    note_count: record.note_file.notes.len(),
                    exists,
                }
            })
            .collect();

        let index = WorkspaceIndex { version: 1, files };

        self.save(&index)?;

        Ok(index)
    }

    pub fn detect_moves(
        &self,
        old_index: &WorkspaceIndex,
        new_index: &WorkspaceIndex,
    ) -> Vec<FileMove> {
        IndexDiff::diff(old_index, new_index)
    }

    pub fn repair_suggestions(
        &self,
        old_index: &WorkspaceIndex,
        new_index: &WorkspaceIndex,
    ) -> Vec<RepairSuggestion> {
        let moves = self.detect_moves(old_index, new_index);

        RepairSuggestion::from_moves(moves)
    }
}
