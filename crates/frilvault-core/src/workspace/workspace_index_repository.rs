use std::{fs, path::Path};

use crate::{
    FrilVaultResult, IndexedFile, NoteAnchor, PathResolver, RepairSuggestion, WorkspaceHealth,
    WorkspaceIndex, WorkspaceStats, YamlNoteRepository,
};

#[derive(Debug, Clone)]
pub struct WorkspaceIndexRepository {
    path_resolver: PathResolver,
}

impl WorkspaceIndexRepository {
    pub fn new(path_resolver: PathResolver) -> Self {
        Self { path_resolver }
    }

    pub fn load(&self) -> FrilVaultResult<WorkspaceIndex> {
        let path = self.path_resolver.workspace_index_path();

        if !path.exists() {
            return Ok(WorkspaceIndex::default());
        }

        let content = fs::read_to_string(path)?;

        let index = serde_yml::from_str(&content)?;

        Ok(index)
    }

    pub fn save(&self, index: &WorkspaceIndex) -> FrilVaultResult<()> {
        let path = self.path_resolver.workspace_index_path();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let yaml = serde_yml::to_string(index)?;

        fs::write(path, yaml)?;

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
        let note_repository = YamlNoteRepository::new(self.path_resolver.clone());

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

    pub fn health_check(&self) -> FrilVaultResult<WorkspaceHealth> {
        // TODO:
        // Use cached index after index invalidation support is implemented.
        let index = self.rebuild()?;

        let mut health = WorkspaceHealth::default();

        for file in index.files {
            if !file.exists {
                health.missing_source_files.push(file.source_file);
            }
        }

        Ok(health)
    }

    pub fn stats(&self) -> FrilVaultResult<WorkspaceStats> {
        let note_repository = YamlNoteRepository::new(self.path_resolver.clone());

        let records = note_repository.list_all_note_files()?;

        let mut stats = WorkspaceStats {
            file_count: records.len(),
            ..Default::default()
        };

        for record in records {
            if self
                .path_resolver
                .workspace_root()
                .join(&record.source_file)
                .exists()
            {
                stats.existing_files += 1;
            } else {
                stats.missing_files += 1;
            }

            for note in record.note_file.notes {
                stats.total_notes += 1;

                match note.anchor {
                    NoteAnchor::Line(_) => {
                        stats.line_notes += 1;
                    }

                    NoteAnchor::Symbol(_) => {
                        stats.symbol_notes += 1;
                    }
                }
            }
        }

        Ok(stats)
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
                .strip_prefix(self.path_resolver.workspace_root())
                .map_err(|_| crate::FrilVaultError::SourcePathOutsideWorkspace)?;

            files.push(relative.to_string_lossy().to_string());
        }

        Ok(())
    }

    fn scan_workspace_files(&self) -> FrilVaultResult<Vec<String>> {
        let mut files = Vec::new();

        self.collect_workspace_files(self.path_resolver.workspace_root(), &mut files)?;

        Ok(files)
    }

    pub fn repair_suggestions(&self) -> FrilVaultResult<Vec<RepairSuggestion>> {
        let health = self.health_check()?;

        let workspace_files = self.scan_workspace_files()?;

        let mut suggestions = Vec::new();

        for missing_file in health.missing_source_files {
            let missing_name = std::path::Path::new(&missing_file)
                .file_name()
                .and_then(|name| name.to_str());

            let Some(missing_name) = missing_name else {
                continue;
            };

            let candidates = workspace_files
                .iter()
                .filter(|candidate| {
                    std::path::Path::new(candidate)
                        .file_name()
                        .and_then(|name| name.to_str())
                        == Some(missing_name)
                })
                .cloned()
                .collect();

            suggestions.push(RepairSuggestion {
                missing_file,
                candidates,
            });
        }

        Ok(suggestions)
    }

    fn move_note_file(&self, source_file: &str, target_file: &str) -> FrilVaultResult<()> {
        let source_note = self.path_resolver.note_path_for_source_file(source_file);

        let target_note = self.path_resolver.note_path_for_source_file(target_file);

        if let Some(parent) = target_note.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::rename(source_note, target_note)?;

        Ok(())
    }

    pub fn apply_repairs(&self) -> FrilVaultResult<usize> {
        let suggestions = self.repair_suggestions()?;

        let mut repaired = 0;

        for suggestion in suggestions {
            let Some(candidate) = suggestion.best_candidate() else {
                continue;
            };

            self.move_note_file(&suggestion.missing_file, candidate)?;

            repaired += 1;
        }

        Ok(repaired)
    }
}
