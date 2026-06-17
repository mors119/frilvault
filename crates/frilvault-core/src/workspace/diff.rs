use std::path::Path;

use crate::{FileMove, IndexedFile, WorkspaceIndex};

pub struct IndexDiff;

impl IndexDiff {
    pub fn diff(old: &WorkspaceIndex, new: &WorkspaceIndex) -> Vec<FileMove> {
        let mut moves = Vec::new();

        for old_file in &old.files {
            let removed = !new
                .files
                .iter()
                .any(|f| f.source_file == old_file.source_file);

            if removed {
                let mut best: Option<(&IndexedFile, f32)> = None;

                for new_file in &new.files {
                    let score =
                        Self::similarity_score(&old_file.source_file, &new_file.source_file);

                    if score >= 0.7 {
                        if let Some((_, best_score)) = &best {
                            if score > *best_score {
                                best = Some((new_file, score));
                            }
                        } else {
                            best = Some((new_file, score));
                        }
                    }
                }

                if let Some((candidate, score)) = best {
                    moves.push(FileMove {
                        from: old_file.source_file.clone(),
                        to: candidate.source_file.clone(),
                        confidence: score,
                    });
                }
            }
        }

        moves
    }

    fn filename_score(a: &str, b: &str) -> f32 {
        let a_name = Path::new(a).file_name().and_then(|n| n.to_str());

        let b_name = Path::new(b).file_name().and_then(|n| n.to_str());

        match (a_name, b_name) {
            (Some(a), Some(b)) if a == b => 0.8,
            _ => 0.0,
        }
    }

    fn directory_score(a: &str, b: &str) -> f32 {
        let a_parent = Path::new(a).parent();
        let b_parent = Path::new(b).parent();

        match (a_parent, b_parent) {
            (Some(a), Some(b)) if a == b => 0.2,
            _ => 0.0,
        }
    }

    pub fn similarity_score(a: &str, b: &str) -> f32 {
        Self::filename_score(a, b) + Self::directory_score(a, b)
    }
}
