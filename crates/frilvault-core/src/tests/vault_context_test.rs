use std::{fs, path::Path};

use crate::{
    LineAnchor, Note, NoteAnchor, add_note_request::AddNoteRequest,
    tests::helper::create_test_vault_context,
};

#[test]
fn load_notes_populates_cache() {
    let workspace_root =
        std::env::temp_dir().join(format!("frilvault-test-{}", uuid::Uuid::new_v4()));

    fs::create_dir_all(&workspace_root).unwrap();

    let mut vault_context = create_test_vault_context(&workspace_root);

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

    fs::remove_dir_all(workspace_root).unwrap();
}
