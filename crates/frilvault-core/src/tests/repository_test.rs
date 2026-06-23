use super::helper::{create_test_note_service, create_test_repository, create_test_workspace};
use crate::{AddNoteRequest, LineAnchor, NoteAnchor};

#[test]
fn list_all_note_files_returns_all_note_files() {
    let workspace = create_test_workspace();
    let workspace_root = workspace.root();
    let mut service = create_test_note_service(workspace_root);

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

    let repository = create_test_repository(workspace_root);

    let note_files = repository.list_all_note_files().unwrap();

    assert_eq!(note_files.len(), 2);
}
