use crate::{
    LineAnchor, NoteAnchor, NoteService, PathResolver, VaultContext, WorkspaceIndexRepository,
    WorkspaceRepository, YamlNoteRepository, add_note_request::AddNoteRequest,
};

use std::fs;

fn create_service(workspace_root: &std::path::Path) -> NoteService {
    let resolver = PathResolver::new(workspace_root);
    let workspace_repository = WorkspaceRepository::new(resolver.clone());
    workspace_repository.create_if_missing().unwrap();

    let index_repository = WorkspaceIndexRepository::new(resolver.clone());
    index_repository.create_if_missing().unwrap();

    let repository = YamlNoteRepository::new(resolver);
    let vault_context = VaultContext::new(repository, index_repository);

    NoteService::new(vault_context)
}

fn create_repository(workspace_root: &std::path::Path) -> YamlNoteRepository {
    let resolver = PathResolver::new(workspace_root);

    YamlNoteRepository::new(resolver)
}

#[test]
fn list_all_note_files_returns_all_note_files() {
    let workspace_root =
        std::env::temp_dir().join(format!("frilvault-test-{}", uuid::Uuid::new_v4()));

    fs::create_dir_all(&workspace_root).unwrap();

    let mut service = create_service(&workspace_root);

    service
        .add_note(AddNoteRequest {
            source_file: "src/main.rs".into(),
            anchor: NoteAnchor::Line(LineAnchor {
                line: 10,
                column: 5,
            }),
            content: "main note".to_string(),
        })
        .unwrap();

    service
        .add_note(AddNoteRequest {
            source_file: "src/lib.rs".into(),
            anchor: NoteAnchor::Line(LineAnchor { line: 3, column: 1 }),
            content: "lib note".to_string(),
        })
        .unwrap();

    let repository = create_repository(&workspace_root);

    let note_files = repository.list_all_note_files().unwrap();

    assert_eq!(note_files.len(), 2);

    fs::remove_dir_all(workspace_root).unwrap();
}
