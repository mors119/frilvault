use crate::{
    FrilVaultResult,
    workspace::{
        repair_engine::RepairEngine,
        repository::workspace_index_repository::WorkspaceIndexRepository,
        snapshot::snapshot_manager::SnapshotManager,
    },
};

pub struct WorkspaceWatcher {
    index_repository: WorkspaceIndexRepository,
    snapshot: SnapshotManager,
    repair_engine: RepairEngine,
}

impl WorkspaceWatcher {
    pub fn on_change(&mut self) -> FrilVaultResult<()> {
        let old = self.snapshot.previous().cloned();
        let new = self.index_repository.rebuild()?;

        if let Some(old) = old {
            let moves = self.index_repository.detect_moves(&old, &new);

            self.repair_engine.apply_moves(moves)?;
        }

        self.snapshot.update(new);

        Ok(())
    }
}
