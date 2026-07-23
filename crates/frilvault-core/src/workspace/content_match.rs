use std::path::Path;

use crate::note::{NoteAnchor, NoteFile};
use crate::symbol::symbol_marker;

pub struct ContentMatcher;

impl ContentMatcher {
    pub fn extract_markers(note_file: &NoteFile) -> Vec<String> {
        note_file
            .notes
            .iter()
            .filter_map(|note| match &note.anchor {
                NoteAnchor::Symbol(symbol) => Some(symbol_marker(symbol)),
                NoteAnchor::Line(_) => None,
            })
            .collect()
    }

    pub fn match_score(markers: &[String], content: &str) -> f32 {
        if markers.is_empty() {
            return 0.0;
        }

        let matched = markers
            .iter()
            .filter(|marker| content.contains(marker.as_str()))
            .count();

        matched as f32 / markers.len() as f32
    }
}

pub fn read_source_file_content(workspace_root: &Path, relative_path: &str) -> Option<String> {
    std::fs::read_to_string(workspace_root.join(relative_path)).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AddNoteRequest, LineAnchor, Note, NoteAnchor, SymbolAnchor, SymbolKind};

    #[test]
    fn match_score_returns_one_when_all_markers_are_present() {
        let markers = vec!["struct UserService".to_string()];

        let score = ContentMatcher::match_score(&markers, "pub struct UserService {");

        assert!((score - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn extract_markers_uses_symbol_signatures_when_available() {
        let note_file = NoteFile {
            notes: vec![Note::new(AddNoteRequest {
                source_file: "src/service.rs".into(),
                anchor: NoteAnchor::Symbol(SymbolAnchor {
                    name: "UserService".to_string(),
                    kind: SymbolKind::Struct,
                    signature: Some("pub struct UserService".to_string()),
                    line_hint: None,
                }),
                content: "note".to_string(),
                tags: None,
            })],
        };

        let markers = ContentMatcher::extract_markers(&note_file);

        assert_eq!(markers, vec!["pub struct UserService"]);
    }

    #[test]
    fn extract_markers_skips_line_anchors() {
        let note_file = NoteFile {
            notes: vec![Note::new(AddNoteRequest {
                source_file: "src/service.rs".into(),
                anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),
                content: "note".to_string(),
                tags: None,
            })],
        };

        assert!(ContentMatcher::extract_markers(&note_file).is_empty());
    }
}
