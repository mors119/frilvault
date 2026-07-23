use std::fs;

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
