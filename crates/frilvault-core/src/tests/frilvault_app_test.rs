use std::fs;

use crate::{AddNoteRequest, FrilVault, LineAnchor, NoteAnchor};

#[test]
fn frilvault_open_creates_note_service() {
    let workspace_root =
        std::env::temp_dir().join(format!("frilvault-test-{}", uuid::Uuid::new_v4()));

    fs::create_dir_all(&workspace_root).unwrap();

    let vault = FrilVault::open(&workspace_root).unwrap();

    let mut notes = vault.create_note_service().unwrap();

    notes
        .add_note(AddNoteRequest {
            source_file: "src/main.rs".into(),
            anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),
            content: "facade note".to_string(),
        })
        .unwrap();

    let result = notes.list_notes("src/main.rs").unwrap();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].note.content, "facade note");

    fs::remove_dir_all(workspace_root).unwrap();
}

#[test]
fn frilvault_open_creates_workspace_service() {
    let workspace_root =
        std::env::temp_dir().join(format!("frilvault-test-{}", uuid::Uuid::new_v4()));

    fs::create_dir_all(&workspace_root).unwrap();

    let vault = FrilVault::open(&workspace_root).unwrap();

    let mut workspace = vault.create_workspace_service().unwrap();

    let stats = workspace.stats().unwrap();

    assert_eq!(stats.file_count, 0);

    fs::remove_dir_all(workspace_root).unwrap();
}
