use crate::{IndexedFile, WorkspaceIndex};

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
