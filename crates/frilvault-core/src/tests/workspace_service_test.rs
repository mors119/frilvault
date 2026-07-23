use std::{fs, path::Path};

use crate::{
    AddNoteRequest, LineAnchor, NoteAnchor,
    note::NoteRepository,
    runtime::VaultContext,
    tests::helper::{
        create_test_note_service, create_test_workspace, create_test_workspace_service,
    },
    workspace::{PathResolver, WorkspaceIndexRepository, WorkspaceService},
};

#[test]
fn warm_up_succeeds_without_notes_folder() {
    let workspace = create_test_workspace();
    let workspace_root = workspace.root();
    let resolver = PathResolver::new(workspace_root);
    let vault_context = VaultContext::new(
        NoteRepository::new(resolver.clone()),
        WorkspaceIndexRepository::new(resolver.clone()),
    );
    let mut service = WorkspaceService::new(
        vault_context,
        WorkspaceIndexRepository::new(resolver.clone()),
    );

    assert!(!resolver.notes_root().exists());

    let index = service.warm_up().unwrap();

    assert!(index.files.is_empty());
    assert!(resolver.workspace_index_path().exists());

    let loaded = service.index_repository.load().unwrap();
    assert!(loaded.files.is_empty());
}

#[test]
fn warm_up_builds_index_from_note_files() {
    let workspace = create_test_workspace();
    let workspace_root = workspace.root();
    fs::create_dir_all(workspace_root.join("src")).unwrap();
    fs::write(workspace_root.join("src/main.rs"), "").unwrap();

    let mut note_service = create_test_note_service(workspace_root);
    note_service
        .add_note(AddNoteRequest {
            source_file: "src/main.rs".into(),
            anchor: NoteAnchor::Line(LineAnchor {
                line: 10,
                column: 5,
            }),
            content: "warm up note".to_string(),
        })
        .unwrap();

    let mut workspace_service = create_test_workspace_service(workspace_root);
    let index = workspace_service.warm_up().unwrap();

    assert_eq!(index.files.len(), 1);
    assert_eq!(index.files[0].source_file, "src/main.rs");
    assert_eq!(index.files[0].note_count, 1);
    assert!(index.files[0].exists);
}

#[test]
fn warm_up_uses_persisted_index_without_rescanning() {
    let workspace = create_test_workspace();
    let workspace_root = workspace.root();
    fs::create_dir_all(workspace_root.join("src")).unwrap();
    fs::write(workspace_root.join("src/main.rs"), "").unwrap();

    let mut note_service = create_test_note_service(workspace_root);
    note_service
        .add_note(AddNoteRequest {
            source_file: "src/main.rs".into(),
            anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),
            content: "first note".to_string(),
        })
        .unwrap();

    let mut workspace_service = create_test_workspace_service(workspace_root);
    let first = workspace_service.warm_up().unwrap();
    assert_eq!(first.files.len(), 1);

    note_service
        .add_note(AddNoteRequest {
            source_file: "src/other.rs".into(),
            anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),
            content: "second note".to_string(),
        })
        .unwrap();

    let second = workspace_service.warm_up().unwrap();
    assert_eq!(second.files.len(), 1);
    assert_eq!(second.files[0].source_file, "src/main.rs");
}

#[test]
fn sync_notes_directory_changes_clears_cache_and_rebuilds_index() {
    let workspace = create_test_workspace();
    let workspace_root = workspace.root();
    fs::create_dir_all(workspace_root.join("src")).unwrap();
    fs::write(workspace_root.join("src/main.rs"), "").unwrap();

    let mut note_service = create_test_note_service(workspace_root);
    note_service
        .add_note(AddNoteRequest {
            source_file: "src/main.rs".into(),
            anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),
            content: "first note".to_string(),
        })
        .unwrap();

    let mut workspace_service = create_test_workspace_service(workspace_root);
    workspace_service.warm_up().unwrap();

    let source_file = Path::new("src/main.rs");
    workspace_service
        .vault_context
        .load_notes(source_file)
        .unwrap();
    assert!(
        workspace_service
            .vault_context
            .contains_cached_notes(source_file)
    );

    note_service
        .add_note(AddNoteRequest {
            source_file: "src/other.rs".into(),
            anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),
            content: "external note".to_string(),
        })
        .unwrap();

    workspace_service.sync_notes_directory_changes().unwrap();

    assert!(
        !workspace_service
            .vault_context
            .contains_cached_notes(source_file)
    );

    let index = workspace_service.index_repository.load().unwrap();
    assert_eq!(index.files.len(), 2);
}

#[test]
fn sync_notes_directory_changes_succeeds_without_notes_folder() {
    let workspace = create_test_workspace();
    let workspace_root = workspace.root();
    let resolver = PathResolver::new(workspace_root);
    let vault_context = VaultContext::new(
        NoteRepository::new(resolver.clone()),
        WorkspaceIndexRepository::new(resolver.clone()),
    );
    let mut service = WorkspaceService::new(
        vault_context,
        WorkspaceIndexRepository::new(resolver.clone()),
    );

    service.warm_up().unwrap();
    assert!(!resolver.notes_root().exists());

    service.sync_notes_directory_changes().unwrap();

    let index = service.index_repository.load().unwrap();
    assert!(index.files.is_empty());
}

#[test]
fn sync_notes_directory_changes_detects_renamed_note_files() {
    let workspace = create_test_workspace();
    let workspace_root = workspace.root();
    let resolver = PathResolver::new(workspace_root);
    fs::create_dir_all(workspace_root.join("src")).unwrap();
    fs::write(workspace_root.join("src/main.rs"), "").unwrap();

    let mut note_service = create_test_note_service(workspace_root);
    note_service
        .add_note(AddNoteRequest {
            source_file: "src/main.rs".into(),
            anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),
            content: "move me".to_string(),
        })
        .unwrap();

    let mut workspace_service = create_test_workspace_service(workspace_root);
    workspace_service.warm_up().unwrap();

    let old_note_path = resolver.note_path_for_source_file("src/main.rs");
    let new_note_path = resolver.note_path_for_source_file("src/main_renamed.rs");
    fs::rename(old_note_path, new_note_path).unwrap();

    workspace_service.sync_notes_directory_changes().unwrap();

    let index = workspace_service.index_repository.load().unwrap();
    assert_eq!(index.files.len(), 1);
    assert_eq!(index.files[0].source_file, "src/main_renamed.rs");
}

#[test]
fn sync_source_file_changes_relocates_notes_after_source_rename() {
    let workspace = create_test_workspace();
    let workspace_root = workspace.root();
    let resolver = PathResolver::new(workspace_root);
    fs::create_dir_all(workspace_root.join("src/parser")).unwrap();
    fs::create_dir_all(workspace_root.join("src/core")).unwrap();
    fs::write(workspace_root.join("src/parser/lib.rs"), "fn lib() {}").unwrap();

    let mut note_service = create_test_note_service(workspace_root);
    note_service
        .add_note(AddNoteRequest {
            source_file: "src/parser/lib.rs".into(),
            anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),
            content: "keep this note".to_string(),
        })
        .unwrap();

    fs::rename(
        workspace_root.join("src/parser/lib.rs"),
        workspace_root.join("src/core/lib.rs"),
    )
    .unwrap();

    let mut workspace_service = create_test_workspace_service(workspace_root);
    workspace_service.warm_up().unwrap();

    let old_note_path = resolver.note_path_for_source_file("src/parser/lib.rs");
    let new_note_path = resolver.note_path_for_source_file("src/core/lib.rs");

    assert!(old_note_path.exists());
    assert!(!new_note_path.exists());

    let repaired = workspace_service.sync_source_file_changes().unwrap();

    assert_eq!(repaired, 1);
    assert!(!old_note_path.exists());
    assert!(new_note_path.exists());

    let index = workspace_service.index_repository.load().unwrap();
    assert_eq!(index.files.len(), 1);
    assert_eq!(index.files[0].source_file, "src/core/lib.rs");
    assert!(index.files[0].exists);
}
