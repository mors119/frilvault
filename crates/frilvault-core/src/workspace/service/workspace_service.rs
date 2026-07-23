//! Workspace-level application services.
//!
//! This module provides statistics,
//! health checks, and repair workflows.

use crate::{
    FrilVaultResult, NoteAnchor, RepairSuggestion, WorkspaceHealth, WorkspaceStats,
    runtime::VaultContext,
    workspace::{
        FileMove, IndexDiff, RepairEngine, WorkspaceIndex, WorkspaceIndexRepository,
        WorkspaceWatcher,
    },
};

const REPAIR_MIN_CONFIDENCE: f32 = 0.7;

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
    notes_watcher: WorkspaceWatcher,
}

impl WorkspaceService {
    pub fn new(vault_context: VaultContext, index_repository: WorkspaceIndexRepository) -> Self {
        let notes_watcher = WorkspaceWatcher::new(index_repository.clone());

        Self {
            vault_context,
            index_repository,
            notes_watcher,
        }
    }

    pub fn warm_up(&mut self) -> FrilVaultResult<WorkspaceIndex> {
        let index = self.index_repository.load_or_rebuild()?;
        self.notes_watcher.seed_snapshot(index.clone());

        Ok(index)
    }

    /// Synchronize index and cache state after external note directory changes.
    ///
    /// Editor integrations should invoke this from a file watcher callback when
    /// `.vault/notes` is created, modified, moved, or deleted outside FrilVault.
    pub fn sync_notes_directory_changes(&mut self) -> FrilVaultResult<()> {
        self.notes_watcher.sync(&mut self.vault_context)
    }

    /// Synchronize note files after source files are renamed or moved in the workspace.
    ///
    /// Editor integrations should invoke this from a file rename/move callback when
    /// a tracked source file changes path outside FrilVault.
    pub fn sync_source_file_changes(&mut self) -> FrilVaultResult<usize> {
        self.index_repository.rebuild()?;
        let repaired = self.apply_repairs()?;

        if repaired > 0 {
            let index = self.index_repository.rebuild()?;
            self.notes_watcher.seed_snapshot(index);
        }

        Ok(repaired)
    }

    pub fn is_vault_gitignored(&self) -> FrilVaultResult<bool> {
        crate::workspace::gitignore::is_vault_gitignored(self.index_repository.workspace_root())
    }

    pub fn append_vault_to_gitignore(&self) -> FrilVaultResult<()> {
        crate::workspace::gitignore::append_vault_to_gitignore(
            self.index_repository.workspace_root(),
        )
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
            let mut candidates: Vec<(String, f32)> = workspace_files
                .iter()
                .filter_map(|candidate| {
                    let score = IndexDiff::similarity_score(&missing_file, candidate);

                    if score >= REPAIR_MIN_CONFIDENCE {
                        Some((candidate.clone(), score))
                    } else {
                        None
                    }
                })
                .collect();

            candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

            suggestions.push(RepairSuggestion {
                missing_file,
                candidates: candidates.into_iter().map(|(path, _)| path).collect(),
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
                let confidence = IndexDiff::similarity_score(&from, &to);

                Some(FileMove {
                    from,
                    to,
                    confidence,
                })
            })
            .collect();

        let repaired = RepairEngine::apply_moves_with_min_confidence(
            &mut self.vault_context,
            moves,
            REPAIR_MIN_CONFIDENCE,
        )?;

        if repaired > 0 {
            self.index_repository.rebuild()?;
        }

        Ok(repaired)
    }
}
