use std::path::Path;

use crate::{
    FrilVaultResult, NoteAnchor, RepairSuggestion, WorkspaceHealth, WorkspaceIndexRepository,
    WorkspaceStats, YamlNoteRepository,
};

pub struct WorkspaceService {
    note_repository: YamlNoteRepository,
    index_repository: WorkspaceIndexRepository,
}

impl WorkspaceService {
    pub fn new(
        note_repository: YamlNoteRepository,
        index_repository: WorkspaceIndexRepository,
    ) -> Self {
        Self {
            note_repository,
            index_repository,
        }
    }

    pub fn stats(&self) -> FrilVaultResult<WorkspaceStats> {
        let index = self.index_repository.rebuild()?;

        let records = self.note_repository.list_all_note_files()?;

        let mut stats = WorkspaceStats {
            file_count: index.files.len(),
            ..Default::default()
        };

        for file in index.files {
            stats.total_notes += file.note_count;

            if file.exists {
                stats.existing_files += 1;
            } else {
                stats.missing_files += 1;
            }
        }

        for record in records {
            for note in record.note_file.notes {
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

    pub fn health_check(&self) -> FrilVaultResult<WorkspaceHealth> {
        let index = self.index_repository.rebuild()?;
        // TODO:
        // Use cached index after index invalidation support is implemented.
        let mut health = WorkspaceHealth::default();

        for file in index.files {
            if !file.exists {
                health.missing_source_files.push(file.source_file);
            }
        }

        Ok(health)
    }

    fn scan_workspace_files(&self) -> FrilVaultResult<Vec<String>> {
        let mut files = Vec::new();

        self.collect_workspace_files(self.index_repository.workspace_root(), &mut files)?;

        Ok(files)
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
                .strip_prefix(self.index_repository.workspace_root())
                .map_err(|_| crate::FrilVaultError::SourcePathOutsideWorkspace)?;

            files.push(relative.to_string_lossy().to_string());
        }

        Ok(())
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

    fn move_note_file(&self, source_file: &str, target_file: &str) -> FrilVaultResult<()> {
        let source_note = self.note_repository.resolve_note_path(source_file);

        let target_note = self.note_repository.resolve_note_path(target_file);

        if let Some(parent) = target_note.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::rename(source_note, target_note)?;

        Ok(())
    }
}
