use crate::{FrilVaultResult, runtime::VaultContext, workspace::FileMove};

pub struct RepairEngine;

const DEFAULT_MIN_CONFIDENCE: f32 = 1.0;

impl RepairEngine {
    pub fn apply_moves(
        vault_context: &mut VaultContext,
        moves: Vec<FileMove>,
    ) -> FrilVaultResult<usize> {
        Self::apply_moves_with_min_confidence(vault_context, moves, DEFAULT_MIN_CONFIDENCE)
    }

    pub fn apply_moves_with_min_confidence(
        vault_context: &mut VaultContext,
        moves: Vec<FileMove>,
        min_confidence: f32,
    ) -> FrilVaultResult<usize> {
        let mut repaired = 0;

        for mv in moves {
            if mv.confidence < min_confidence {
                continue;
            }

            let old_path = vault_context.resolve_note_path(&mv.from);

            let new_path = vault_context.resolve_note_path(&mv.to);

            if let Some(parent) = new_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            if old_path.exists() {
                std::fs::rename(&old_path, &new_path)?;
            }

            vault_context.invalidate_notes(std::path::Path::new(&mv.from));
            vault_context.invalidate_notes(std::path::Path::new(&mv.to));

            let _ = vault_context.load_notes(std::path::Path::new(&mv.to));

            repaired += 1;
        }

        Ok(repaired)
    }
}
