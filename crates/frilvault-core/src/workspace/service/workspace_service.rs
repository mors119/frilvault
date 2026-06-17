//! Workspace-level application services.
//!
//! This module provides statistics,
//! health checks, and repair workflows.

use crate::{
    FrilVaultResult, NoteAnchor, RepairSuggestion, VaultContext, WorkspaceHealth,
    WorkspaceIndexRepository, WorkspaceStats,
};

/// Application service responsible for
/// workspace-level operations.
///
/// Examples:
/// - statistics
/// - health checks
/// - repair suggestions
pub struct WorkspaceService {
    pub vault_context: VaultContext,
    pub index_repository: WorkspaceIndexRepository,
}

impl WorkspaceService {
    pub fn new(vault_context: VaultContext, index_repository: WorkspaceIndexRepository) -> Self {
        Self {
            vault_context,
            index_repository,
        }
    }

    pub fn stats(&mut self) -> FrilVaultResult<WorkspaceStats> {
        let index = self.vault_context.rebuild_index()?;

        let records = self.vault_context.list_all_note_files()?;

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
                    NoteAnchor::Line(_) => stats.line_notes += 1,
                    NoteAnchor::Symbol(_) => stats.symbol_notes += 1,
                }
            }
        }

        Ok(stats)
    }

    pub fn health_check(&mut self) -> FrilVaultResult<WorkspaceHealth> {
        let index = self.index_repository.rebuild()?;

        let mut health = WorkspaceHealth::default();

        for file in index.files {
            if !file.exists {
                health.missing_source_files.push(file.source_file);
            }
        }

        Ok(health)
    }

    pub fn repair_suggestions(&mut self) -> FrilVaultResult<Vec<RepairSuggestion>> {
        let health = self.health_check()?;
        let workspace_files = self.vault_context.scan_workspace_files()?;

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

    pub fn apply_repairs(&mut self) -> FrilVaultResult<usize> {
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
        let source_note = self.vault_context.resolve_note_path(source_file);

        let target_note = self.vault_context.resolve_note_path(target_file);

        if let Some(parent) = target_note.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::rename(source_note, target_note)?;

        Ok(())
    }
}
