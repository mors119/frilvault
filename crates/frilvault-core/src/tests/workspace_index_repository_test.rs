use std::fs;

use crate::{
    AddNoteInput, IndexedFile, LineAnchor, NoteAnchor, NoteService, PathResolver, WorkspaceIndex,
    WorkspaceIndexRepository, YamlNoteRepository,
};

#[test]
fn load_returns_default_index_when_missing() {
    let workspace_root =
        std::env::temp_dir().join(format!("frilvault-test-{}", uuid::Uuid::new_v4(),));

    fs::create_dir_all(&workspace_root).unwrap();

    let resolver = PathResolver::new(&workspace_root);

    let repository = WorkspaceIndexRepository::new(resolver);

    let index = repository.load().unwrap();

    assert!(index.files.is_empty());

    fs::remove_dir_all(workspace_root).unwrap();
}

#[test]
fn save_and_load_index() {
    let workspace_root =
        std::env::temp_dir().join(format!("frilvault-test-{}", uuid::Uuid::new_v4(),));

    fs::create_dir_all(&workspace_root).unwrap();

    let resolver = PathResolver::new(&workspace_root);

    let repository = WorkspaceIndexRepository::new(resolver);

    let mut index = WorkspaceIndex::default();

    index.files.push(crate::IndexedFile {
        source_file: "src/main.rs".to_string(),
        note_count: 3,
        exists: true,
    });

    repository.save(&index).unwrap();

    let loaded = repository.load().unwrap();

    assert_eq!(loaded.files.len(), 1,);

    assert_eq!(loaded.files[0].note_count, 3,);

    fs::remove_dir_all(workspace_root).unwrap();
}

#[test]
fn create_if_missing_creates_index_file() {
    let workspace_root =
        std::env::temp_dir().join(format!("frilvault-test-{}", uuid::Uuid::new_v4(),));

    fs::create_dir_all(&workspace_root).unwrap();

    let resolver = PathResolver::new(&workspace_root);
    let repository = WorkspaceIndexRepository::new(resolver.clone());

    repository.create_if_missing().unwrap();

    assert!(resolver.workspace_index_path().exists());

    fs::remove_dir_all(workspace_root).unwrap();
}

#[test]
fn rebuild_creates_index_from_note_files() {
    let workspace_root =
        std::env::temp_dir().join(format!("frilvault-test-{}", uuid::Uuid::new_v4()));

    fs::create_dir_all(&workspace_root).unwrap();
    fs::create_dir_all(workspace_root.join("src")).unwrap();
    fs::write(workspace_root.join("src/main.rs"), "").unwrap();
    fs::write(workspace_root.join("src/lib.rs"), "").unwrap();

    let resolver = PathResolver::new(&workspace_root);

    let note_repository = YamlNoteRepository::new(resolver.clone());
    let service = NoteService::new(note_repository);

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

    service
        .add_note(AddNoteInput {
            source_file: "src/lib.rs".into(),
            anchor: NoteAnchor::Line(LineAnchor { line: 7, column: 2 }),
            content: "second lib note".to_string(),
        })
        .unwrap();

    let index_repository = WorkspaceIndexRepository::new(resolver);

    let index = index_repository.rebuild().unwrap();

    assert_eq!(index.files.len(), 2);

    assert!(
        index
            .files
            .iter()
            .any(|file| file.source_file == "src/main.rs" && file.note_count == 1)
    );

    assert!(
        index
            .files
            .iter()
            .any(|file| file.source_file == "src/lib.rs" && file.note_count == 2)
    );

    assert!(index.files.iter().all(|file| file.exists));

    fs::remove_dir_all(workspace_root).unwrap();
}

#[test]
fn rebuild_marks_missing_files_as_not_existing() {
    let workspace_root =
        std::env::temp_dir().join(format!("frilvault-test-{}", uuid::Uuid::new_v4()));

    fs::create_dir_all(&workspace_root).unwrap();

    let resolver = PathResolver::new(&workspace_root);

    let note_repository = YamlNoteRepository::new(resolver.clone());
    let service = NoteService::new(note_repository);

    service
        .add_note(AddNoteInput {
            source_file: "src/missing.rs".into(),
            anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),
            content: "orphan".to_string(),
        })
        .unwrap();

    let index_repository = WorkspaceIndexRepository::new(resolver);

    let index = index_repository.rebuild().unwrap();

    assert_eq!(index.files.len(), 1);
    assert_eq!(index.files[0].source_file, "src/missing.rs");
    assert!(!index.files[0].exists);

    fs::remove_dir_all(workspace_root).unwrap();
}

#[test]
fn health_check_detects_missing_files() {
    let workspace_root =
        std::env::temp_dir().join(format!("frilvault-test-{}", uuid::Uuid::new_v4()));

    fs::create_dir_all(&workspace_root).unwrap();

    let resolver = PathResolver::new(&workspace_root);
    let repository = WorkspaceIndexRepository::new(resolver);

    let index = WorkspaceIndex {
        version: 1,
        files: vec![IndexedFile {
            source_file: "src/missing.rs".to_string(),
            note_count: 1,
            exists: false,
        }],
    };

    repository.save(&index).unwrap();

    let health = repository.health_check().unwrap();

    assert_eq!(health.missing_source_files.len(), 1);
    assert_eq!(health.missing_source_files[0], "src/missing.rs");

    fs::remove_dir_all(workspace_root).unwrap();
}
