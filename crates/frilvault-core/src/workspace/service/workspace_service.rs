//! Workspace-level application services.
//!
//! This module provides statistics,
//! health checks, and repair workflows.

use crate::{
    FrilVaultResult, NoteAnchor, RepairSuggestion, WorkspaceHealth, WorkspaceStats,
    runtime::VaultContext,
    workspace::{FileMove, RepairEngine, WorkspaceIndex, WorkspaceIndexRepository},
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

    pub fn warm_up(&mut self) -> FrilVaultResult<WorkspaceIndex> {
        self.index_repository.rebuild()
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

        let moves = suggestions
            .into_iter()
            .filter_map(|suggestion| {
                let from = suggestion.missing_file.clone();
                let to = suggestion.best_candidate()?.clone();
                Some(FileMove {
                    from,
                    to,
                    confidence: 1.0,
                })
            })
            .collect();

        RepairEngine::apply_moves(&mut self.vault_context, moves)
    }
}
