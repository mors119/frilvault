use crate::{FrilVaultResult, runtime::VaultContext, workspace::FileMove};

pub struct RepairEngine {
    vault_context: VaultContext,
}

impl RepairEngine {
    pub fn apply_moves(&mut self, moves: Vec<FileMove>) -> FrilVaultResult<usize> {
        let mut repaired = 0;

        for mv in moves {
            if mv.confidence < 1.0 {
                continue;
            }

            let old_path = self.vault_context.resolve_note_path(&mv.from);

            let new_path = self.vault_context.resolve_note_path(&mv.to);

            if let Some(parent) = new_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            if old_path.exists() {
                std::fs::rename(&old_path, &new_path)?;
            }

            self.vault_context.invalidate_notes(mv.from.as_ref());
            self.vault_context.invalidate_notes(mv.to.as_ref());

            let _ = self.vault_context.load_notes(mv.to.as_ref());

            repaired += 1;
        }

        Ok(repaired)
    }
}
