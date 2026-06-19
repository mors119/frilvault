use std::{fs, path::Path};
use uuid::Uuid;

use crate::{
    FileMove, LineAnchor, NoteAnchor, NoteService, PathResolver, VaultContext,
    WorkspaceIndexRepository, YamlNoteRepository, add_note_request::AddNoteRequest,
    repair_engine::RepairEngine,
};

#[test]
fn repair_engine_moves_note_files() {
    let workspace_root = std::env::temp_dir().join(format!("frilvault-test-{}", Uuid::new_v4()));

    fs::create_dir_all(&workspace_root).unwrap();

    let resolver = PathResolver::new(&workspace_root);

    let note_repository = YamlNoteRepository::new(resolver.clone());

    let index_repository = WorkspaceIndexRepository::new(resolver.clone());

    let vault_context = VaultContext::new(note_repository, index_repository);

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

    fs::remove_dir_all(workspace_root).unwrap();
}

#[test]
fn repair_engine_invalidates_cache_correctly() {
    let workspace_root =
        std::env::temp_dir().join(format!("frilvault-test-{}", uuid::Uuid::new_v4()));

    fs::create_dir_all(&workspace_root).unwrap();

    let resolver = PathResolver::new(&workspace_root);

    let note_repository = YamlNoteRepository::new(resolver.clone());

    let index_repository = WorkspaceIndexRepository::new(resolver.clone());

    let mut vault_context = VaultContext::new(note_repository, index_repository);

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

    fs::remove_dir_all(workspace_root).unwrap();
}
