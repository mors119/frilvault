use crate::workspace::{IndexedFile, WorkspaceIndex};

#[test]
fn workspace_index_starts_empty() {
    let index = WorkspaceIndex::default();

    assert_eq!(index.version, 1,);

    assert!(index.files.is_empty());
}

#[test]
fn workspace_index_can_store_files() {
    let mut index = WorkspaceIndex::default();

    index.files.push(IndexedFile {
        source_file: "src/main.rs".to_string(),
        note_count: 3,
        exists: true,
    });

    assert_eq!(index.files.len(), 1,);

    assert_eq!(index.files[0].note_count, 3,);
}

#[test]
fn workspace_index_upsert_and_move_files() {
    let mut index = WorkspaceIndex::default();

    index.upsert_file(IndexedFile {
        source_file: "src/main.rs".to_string(),
        note_count: 1,
        exists: true,
    });

    index.upsert_file(IndexedFile {
        source_file: "src/main.rs".to_string(),
        note_count: 2,
        exists: false,
    });

    assert_eq!(index.files.len(), 1);
    assert_eq!(index.files[0].note_count, 2);
    assert!(!index.files[0].exists);

    assert!(index.move_file("src/main.rs", "src/main_renamed.rs"));
    assert_eq!(index.files[0].source_file, "src/main_renamed.rs");

    assert!(index.remove_file("src/main_renamed.rs"));
    assert!(index.files.is_empty());
}
