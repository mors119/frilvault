use std::path::Path;

use crate::{
    AddNoteRequest, LineAnchor, Note, NoteAnchor,
    tests::helper::{create_test_vault_context, create_test_workspace},
    workspace::{FileMove, PathResolver, RepairEngine},
};

#[test]
fn repair_engine_moves_note_files() {
    let workspace = create_test_workspace();
    let workspace_root = workspace.root();
    let resolver = PathResolver::new(workspace_root);
    let mut vault_context = create_test_vault_context(workspace_root);

    vault_context
        .note_repository
        .append_note(
            Path::new("src/main.rs"),
            &Note::new(AddNoteRequest {
                source_file: "src/main.rs".into(),
                anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),
                content: "test note".to_string(),
                tags: None,
            }),
        )
        .unwrap();

    let moves = vec![FileMove {
        from: "src/main.rs".to_string(),
        to: "src/main_renamed.rs".to_string(),
        confidence: 1.0,
    }];

    let repaired = RepairEngine::apply_moves(&mut vault_context, moves).unwrap();

    assert_eq!(repaired, 1);

    let old_path = resolver.note_path_for_source_file("src/main.rs");
    let new_path = resolver.note_path_for_source_file("src/main_renamed.rs");

    assert!(!old_path.exists());
    assert!(new_path.exists());
}

#[test]
fn repair_engine_invalidates_cache_correctly() {
    let workspace = create_test_workspace();
    let workspace_root = workspace.root();
    let mut vault_context = create_test_vault_context(workspace_root);

    let _ = vault_context.load_notes("src/main.rs".as_ref());

    assert!(vault_context.contains_cached_notes(Path::new("src/main.rs")));

    let moves = vec![FileMove {
        from: "src/main.rs".to_string(),
        to: "src/main_renamed.rs".to_string(),
        confidence: 1.0,
    }];

    RepairEngine::apply_moves(&mut vault_context, moves).unwrap();

    assert!(!vault_context.contains_cached_notes(Path::new("src/main.rs")));
    assert!(vault_context.contains_cached_notes(Path::new("src/main_renamed.rs")));
}

#[test]
fn repair_engine_applies_high_confidence_moves_when_threshold_allows() {
    let workspace = create_test_workspace();
    let workspace_root = workspace.root();
    let resolver = PathResolver::new(workspace_root);
    let mut vault_context = create_test_vault_context(workspace_root);

    vault_context
        .note_repository
        .append_note(
            Path::new("src/parser/lib.rs"),
            &Note::new(AddNoteRequest {
                source_file: "src/parser/lib.rs".into(),
                anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),
                content: "test note".to_string(),
                tags: None,
            }),
        )
        .unwrap();

    let moves = vec![FileMove {
        from: "src/parser/lib.rs".to_string(),
        to: "src/core/lib.rs".to_string(),
        confidence: 0.8,
    }];

    let repaired =
        RepairEngine::apply_moves_with_min_confidence(&mut vault_context, moves, 0.7).unwrap();

    assert_eq!(repaired.len(), 1);

    let old_path = resolver.note_path_for_source_file("src/parser/lib.rs");
    let new_path = resolver.note_path_for_source_file("src/core/lib.rs");

    assert!(!old_path.exists());
    assert!(new_path.exists());
}
