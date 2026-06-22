use super::helper::create_test_workspace;
use crate::{AddNoteRequest, FrilVault, LineAnchor, NoteAnchor};

#[test]
fn frilvault_open_creates_note_service() {
    let workspace = create_test_workspace();
    let workspace_root = workspace.root();
    let vault = FrilVault::open(workspace_root).unwrap();

    let mut notes = vault.notes().unwrap();

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

}

#[test]
fn frilvault_open_creates_workspace_service() {
    let workspace = create_test_workspace();
    let workspace_root = workspace.root();
    let vault = FrilVault::open(workspace_root).unwrap();

    let mut workspace = vault.workspace().unwrap();

    let stats = workspace.stats().unwrap();

    assert_eq!(stats.file_count, 0);

}
