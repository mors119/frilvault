use std::path::Path;

use crate::{
    AddNoteRequest, LineAnchor, Note, NoteAnchor,
    tests::helper::{create_test_vault_context, create_test_workspace},
};

#[test]
fn load_notes_populates_cache() {
    let workspace = create_test_workspace();
    let workspace_root = workspace.root();
    let mut vault_context = create_test_vault_context(workspace_root);

    vault_context
        .note_repository
        .append_note(
            Path::new("src/main.rs"),
            &Note::new(AddNoteRequest {
                source_file: "src/main.rs".into(),
                anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),
                content: "test".to_string(),
            }),
        )
        .unwrap();

    assert!(!vault_context.contains_cached_notes(Path::new("src/main.rs")));

    vault_context.load_notes(Path::new("src/main.rs")).unwrap();

    assert!(vault_context.contains_cached_notes(Path::new("src/main.rs")));
}
