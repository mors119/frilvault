use crate::{
    FrilVaultResult,
    runtime::VaultContext,
    workspace::{RepairEngine, SnapshotManager, WorkspaceIndex, WorkspaceIndexRepository},
};

/// Tracks workspace index snapshots and synchronizes runtime state
/// after external note directory changes.
///
/// Long-running integrations such as the VS Code extension should
/// call `WorkspaceService::sync_notes_directory_changes` when a file
/// watcher detects changes under `.vault/notes`.
pub struct WorkspaceWatcher {
    index_repository: WorkspaceIndexRepository,
    snapshot: SnapshotManager,
}

impl WorkspaceWatcher {
    pub fn new(index_repository: WorkspaceIndexRepository) -> Self {
        Self {
            index_repository,
            snapshot: SnapshotManager::new(),
        }
    }

    pub fn seed_snapshot(&mut self, index: WorkspaceIndex) {
        self.snapshot.update(index);
    }

    pub fn sync(&mut self, vault_context: &mut VaultContext) -> FrilVaultResult<()> {
        vault_context.clear_notes_cache();

        let old = self.snapshot.previous().cloned();
        let new = self.index_repository.rebuild()?;

        if let Some(old) = old {
            let moves = self.index_repository.detect_moves(&old, &new);
            RepairEngine::apply_moves(vault_context, moves)?;
        }

        self.snapshot.update(new);

        Ok(())
    }
}
