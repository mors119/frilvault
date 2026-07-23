use std::{fs, path::Path};

use crate::{
    AddNoteRequest, LineAnchor, Note, NoteAnchor, UpdateNoteRequest,
    tests::helper::{create_test_note_service, create_test_vault_context, create_test_workspace},
    workspace::PathResolver,
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
                tags: None,
            }),
        )
        .unwrap();

    assert!(!vault_context.contains_cached_notes(Path::new("src/main.rs")));

    vault_context.load_notes(Path::new("src/main.rs")).unwrap();

    assert!(vault_context.contains_cached_notes(Path::new("src/main.rs")));
}

#[test]
fn load_notes_uses_cache_on_second_load() {
    let workspace = create_test_workspace();
    let workspace_root = workspace.root();
    let mut vault_context = create_test_vault_context(workspace_root);
    let source_file = Path::new("src/main.rs");

    vault_context
        .note_repository
        .append_note(
            source_file,
            &Note::new(AddNoteRequest {
                source_file: "src/main.rs".into(),
                anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),
                content: "cached".to_string(),
                tags: None,
            }),
        )
        .unwrap();

    let first = vault_context.load_notes(source_file).unwrap();
    let note_path = vault_context.resolve_note_path("src/main.rs");
    fs::remove_file(note_path).unwrap();

    let second = vault_context.load_notes(source_file).unwrap();

    assert_eq!(first.notes.len(), 1);
    assert_eq!(second, first);
    assert!(vault_context.contains_cached_notes(source_file));
}

#[test]
fn preload_notes_populates_cache() {
    let workspace = create_test_workspace();
    let workspace_root = workspace.root();
    let mut vault_context = create_test_vault_context(workspace_root);
    let source_file = Path::new("src/main.rs");

    vault_context
        .note_repository
        .append_note(
            source_file,
            &Note::new(AddNoteRequest {
                source_file: "src/main.rs".into(),
                anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),
                content: "preload me".to_string(),
                tags: None,
            }),
        )
        .unwrap();

    assert!(!vault_context.contains_cached_notes(source_file));

    vault_context.preload_notes(source_file).unwrap();

    assert!(vault_context.contains_cached_notes(source_file));
}

#[test]
fn preload_notes_skips_disk_when_cache_is_warm() {
    let workspace = create_test_workspace();
    let workspace_root = workspace.root();
    let mut vault_context = create_test_vault_context(workspace_root);
    let source_file = Path::new("src/main.rs");

    vault_context
        .note_repository
        .append_note(
            source_file,
            &Note::new(AddNoteRequest {
                source_file: "src/main.rs".into(),
                anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),
                content: "cached".to_string(),
                tags: None,
            }),
        )
        .unwrap();

    vault_context.preload_notes(source_file).unwrap();
    fs::remove_file(vault_context.resolve_note_path("src/main.rs")).unwrap();

    vault_context.preload_notes(source_file).unwrap();

    assert!(vault_context.contains_cached_notes(source_file));
}

#[test]
fn note_service_preloads_notes_for_source_file() {
    let workspace = create_test_workspace();
    let workspace_root = workspace.root();
    let mut service = create_test_note_service(workspace_root);
    let source_file = Path::new("src/main.rs");

    service
        .add_note(AddNoteRequest {
            source_file: source_file.to_path_buf(),
            anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),
            content: "service preload".to_string(),
            tags: None,
        })
        .unwrap();

    service.preload_notes(source_file).unwrap();

    let notes = service.list_notes(source_file).unwrap();
    assert_eq!(notes.len(), 1);
    assert_eq!(notes[0].note.content, "service preload");
}

#[test]
fn invalidate_notes_removes_cached_entry() {
    let workspace = create_test_workspace();
    let workspace_root = workspace.root();
    let mut vault_context = create_test_vault_context(workspace_root);
    let source_file = Path::new("src/main.rs");

    vault_context
        .note_repository
        .append_note(
            source_file,
            &Note::new(AddNoteRequest {
                source_file: "src/main.rs".into(),
                anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),
                content: "test".to_string(),
                tags: None,
            }),
        )
        .unwrap();

    vault_context.load_notes(source_file).unwrap();
    assert!(vault_context.contains_cached_notes(source_file));

    vault_context.invalidate_notes(source_file);

    assert!(!vault_context.contains_cached_notes(source_file));
}

#[test]
fn clear_notes_cache_removes_all_entries() {
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
                tags: None,
            }),
        )
        .unwrap();

    vault_context.load_notes(Path::new("src/main.rs")).unwrap();
    assert!(vault_context.contains_cached_notes(Path::new("src/main.rs")));

    vault_context.clear_notes_cache();

    assert!(!vault_context.contains_cached_notes(Path::new("src/main.rs")));
}

#[test]
fn add_note_invalidates_cached_entry() {
    let workspace = create_test_workspace();
    let workspace_root = workspace.root();
    let mut service = create_test_note_service(workspace_root);
    let source_file = Path::new("src/main.rs");

    service
        .add_note(AddNoteRequest {
            source_file: "src/main.rs".into(),
            anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),
            content: "first".to_string(),
            tags: None,
        })
        .unwrap();

    let first_list = service.list_notes(source_file).unwrap();
    assert_eq!(first_list.len(), 1);

    service
        .add_note(AddNoteRequest {
            source_file: "src/main.rs".into(),
            anchor: NoteAnchor::Line(LineAnchor { line: 2, column: 1 }),
            content: "second".to_string(),
            tags: None,
        })
        .unwrap();

    let second_list = service.list_notes(source_file).unwrap();
    assert_eq!(second_list.len(), 2);
}

#[test]
fn delete_note_invalidates_cached_entry() {
    let workspace = create_test_workspace();
    let workspace_root = workspace.root();
    let mut service = create_test_note_service(workspace_root);
    let source_file = Path::new("src/main.rs");

    let deleted_note = service
        .add_note(AddNoteRequest {
            source_file: source_file.to_path_buf(),
            anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),
            content: "delete me".to_string(),
            tags: None,
        })
        .unwrap();
    service
        .add_note(AddNoteRequest {
            source_file: source_file.to_path_buf(),
            anchor: NoteAnchor::Line(LineAnchor { line: 2, column: 1 }),
            content: "keep me".to_string(),
            tags: None,
        })
        .unwrap();

    assert_eq!(service.list_notes(source_file).unwrap().len(), 2);

    service.delete_note(source_file, deleted_note.id).unwrap();

    let notes = service.list_notes(source_file).unwrap();
    assert_eq!(notes.len(), 1);
    assert_eq!(notes[0].note.content, "keep me");
}

#[test]
fn update_note_invalidates_cached_entry() {
    let workspace = create_test_workspace();
    let workspace_root = workspace.root();
    let mut service = create_test_note_service(workspace_root);
    let source_file = Path::new("src/main.rs");

    let note = service
        .add_note(AddNoteRequest {
            source_file: source_file.to_path_buf(),
            anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),
            content: "old content".to_string(),
            tags: None,
        })
        .unwrap();

    let cached_notes = service.list_notes(source_file).unwrap();
    assert_eq!(cached_notes[0].note.content, "old content");

    service
        .update_note(
            source_file,
            note.id,
            UpdateNoteRequest {
                content: "new content".to_string(),
                tags: None,
                expected_updated_at: None,
            },
        )
        .unwrap();

    let notes = service.list_notes(source_file).unwrap();
    assert_eq!(notes.len(), 1);
    assert_eq!(notes[0].note.content, "new content");
}

#[test]
fn search_notes_uses_cache_for_indexed_files() {
    let workspace = create_test_workspace();
    let workspace_root = workspace.root();
    let mut service = create_test_note_service(workspace_root);

    service
        .add_note(AddNoteRequest {
            source_file: "src/main.rs".into(),
            anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),
            content: "cache me for search".to_string(),
            tags: None,
        })
        .unwrap();

    let first_search = service.search_notes("cache").unwrap();
    assert_eq!(first_search.len(), 1);

    let note_path = PathResolver::new(workspace_root).note_path_for_source_file("src/main.rs");
    fs::remove_file(note_path).unwrap();

    let second_search = service.search_notes("cache").unwrap();
    assert_eq!(second_search.len(), 1);
    assert_eq!(second_search[0].note.content, "cache me for search");
}

#[test]
fn search_notes_reflects_updated_content_after_update() {
    let workspace = create_test_workspace();
    let workspace_root = workspace.root();
    let mut service = create_test_note_service(workspace_root);
    let source_file = Path::new("src/main.rs");

    let note = service
        .add_note(AddNoteRequest {
            source_file: source_file.to_path_buf(),
            anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),
            content: "search old content".to_string(),
            tags: None,
        })
        .unwrap();

    service.list_notes(source_file).unwrap();
    assert_eq!(service.search_notes("old content").unwrap().len(), 1);

    service
        .update_note(
            source_file,
            note.id,
            UpdateNoteRequest {
                content: "search new content".to_string(),
                tags: None,
                expected_updated_at: None,
            },
        )
        .unwrap();

    let results = service.search_notes("new content").unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].note.content, "search new content");
    assert!(service.search_notes("old content").unwrap().is_empty());
}
