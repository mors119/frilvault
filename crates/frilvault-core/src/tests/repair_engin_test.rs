use std::path::Path;

use crate::{
    AddNoteRequest, LineAnchor, NoteAnchor,
    note::NoteService,
    tests::helper::{create_test_vault_context, create_test_workspace},
    workspace::{FileMove, PathResolver, repair_engine::RepairEngine},
};

#[test]
fn repair_engine_moves_note_files() {
    let workspace = create_test_workspace();
    let workspace_root = workspace.root();
    let resolver = PathResolver::new(workspace_root);
    let vault_context = create_test_vault_context(workspace_root);
    let mut service = NoteService::new(vault_context.clone());

    // 1. create note for original file
    service
        .add_note(AddNoteRequest {
            source_file: "src/main.rs".into(),
            anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),
            content: "test note".to_string(),
        })
        .unwrap();

    // 2. simulate repair move
    let moves = vec![FileMove {
        from: "src/main.rs".to_string(),
        to: "src/main_renamed.rs".to_string(),
        confidence: 1.0,
    }];

    let mut engine = RepairEngine { vault_context };

    let repaired = engine.apply_moves(moves).unwrap();

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

    // preload cache
    let _ = vault_context.load_notes("src/main.rs".as_ref());

    assert!(vault_context.note_cache.contains(Path::new("src/main.rs")));

    let mut engine = RepairEngine { vault_context };

    let moves = vec![FileMove {
        from: "src/main.rs".to_string(),
        to: "src/main_renamed.rs".to_string(),
        confidence: 1.0,
    }];

    let _ = engine.apply_moves(moves).unwrap();

    assert!(
        !engine
            .vault_context
            .note_cache
            .contains(Path::new("src/main.rs"))
    );

    assert!(
        engine
            .vault_context
            .note_cache
            .contains(Path::new("src/main_renamed.rs"))
    );
}
