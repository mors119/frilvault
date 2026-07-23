use crate::{
    FrilVaultResult,
    runtime::VaultContext,
    workspace::{
        repair_engine::RepairEngine,
        repository::workspace_index_repository::WorkspaceIndexRepository,
        snapshot::snapshot_manager::SnapshotManager,
    },
};

pub struct WorkspaceWatcher {
    index_repository: WorkspaceIndexRepository,
    snapshot: SnapshotManager,
    vault_context: VaultContext,
}

impl WorkspaceWatcher {
    pub fn on_change(&mut self) -> FrilVaultResult<()> {
        let old = self.snapshot.previous().cloned();
        let new = self.index_repository.rebuild()?;

        if let Some(old) = old {
            let moves = self.index_repository.detect_moves(&old, &new);

            RepairEngine::apply_moves(&mut self.vault_context, moves)?;
        }

        self.snapshot.update(new);

        Ok(())
    }
}
