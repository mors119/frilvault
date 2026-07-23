use crate::{FrilVaultResult, runtime::VaultContext, workspace::FileMove};

pub struct RepairEngine;

const DEFAULT_MIN_CONFIDENCE: f32 = 1.0;

impl RepairEngine {
    pub fn apply_moves(
        vault_context: &mut VaultContext,
        moves: Vec<FileMove>,
    ) -> FrilVaultResult<usize> {
        Ok(
            Self::apply_moves_with_min_confidence(vault_context, moves, DEFAULT_MIN_CONFIDENCE)?
                .len(),
        )
    }

    pub fn apply_moves_with_min_confidence(
        vault_context: &mut VaultContext,
        moves: Vec<FileMove>,
        min_confidence: f32,
    ) -> FrilVaultResult<Vec<FileMove>> {
        let mut applied = Vec::new();

        for mv in moves {
            if mv.confidence < min_confidence {
                continue;
            }

            let old_path = vault_context.resolve_note_path(&mv.from);

            let new_path = vault_context.resolve_note_path(&mv.to);

            let to = mv.to.clone();

            vault_context.invalidate_notes(std::path::Path::new(&mv.from));
            vault_context.invalidate_notes(std::path::Path::new(&to));

            if old_path.exists() {
                if let Some(parent) = new_path.parent() {
                    std::fs::create_dir_all(parent)?;
                }

                std::fs::rename(&old_path, &new_path)?;
                applied.push(mv);
            }

            let _ = vault_context.load_notes(std::path::Path::new(&to));
        }

        Ok(applied)
    }
}
