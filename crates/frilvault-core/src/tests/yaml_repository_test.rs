use crate::{AddNoteInput, LineAnchor, NoteAnchor, NoteService, PathResolver, YamlNoteRepository};

use std::fs;

fn create_service(workspace_root: &std::path::Path) -> NoteService<YamlNoteRepository> {
    let resolver = PathResolver::new(workspace_root);
    let repository = YamlNoteRepository::new(resolver);

    NoteService::new(repository)
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

    let service = create_service(&workspace_root);

    service
        .add_note(AddNoteInput {
            source_file: "src/main.rs".into(),
            anchor: NoteAnchor::Line(LineAnchor {
                line: 10,
                column: 5,
            }),
            content: "main note".to_string(),
        })
        .unwrap();

    service
        .add_note(AddNoteInput {
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
