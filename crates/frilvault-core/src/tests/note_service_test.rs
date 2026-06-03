use crate::{
    AddNoteInput, NoteService, PathResolver, YamlNoteRepository, constants::NOTE_FILE_EXTENSION,
};

use std::fs;

#[test]
fn add_note_creates_yaml_file() {
    let workspace_root =
        std::env::temp_dir().join(format!("frilvault-test-{}", uuid::Uuid::new_v4()));

    fs::create_dir_all(&workspace_root).unwrap();

    let resolver = PathResolver::new(&workspace_root);
    let repository = YamlNoteRepository::new(resolver);
    let service = NoteService::new(repository);

    let input = AddNoteInput {
        source_file: "src/main.rs".into(),
        anchor: crate::note::NoteAnchor::Line(crate::note::LineAnchor {
            line: 10,
            column: 5,
        }),
        content: "여기서 스캔이 시작된다.".to_string(),
    };

    service.add_note(input).unwrap();

    let note_path = workspace_root
        .join(".vault")
        .join(format!("src/main.rs.{}", NOTE_FILE_EXTENSION));

    assert!(note_path.exists());

    let content = fs::read_to_string(note_path).unwrap();

    assert!(content.contains("여기서 스캔이 시작된다."));

    fs::remove_dir_all(workspace_root).unwrap();
}

#[test]
fn load_notes_from_existing_yaml() {
    let workspace_root =
        std::env::temp_dir().join(format!("frilvault-test-{}", uuid::Uuid::new_v4()));

    fs::create_dir_all(&workspace_root).unwrap();

    let resolver = PathResolver::new(&workspace_root);
    let repository = YamlNoteRepository::new(resolver);
    let service = NoteService::new(repository);

    service
        .add_note(AddNoteInput {
            source_file: "src/lib.rs".into(),
            anchor: crate::note::NoteAnchor::Line(crate::note::LineAnchor { line: 1, column: 1 }),
            content: "lib 진입점".to_string(),
        })
        .unwrap();

    let notes = service.list_notes("src/lib.rs").unwrap();

    assert_eq!(notes.len(), 1);
    assert_eq!(notes[0].content, "lib 진입점");

    fs::remove_dir_all(workspace_root).unwrap();
}

#[test]
fn add_note_and_load_note() {
    let workspace_root =
        std::env::temp_dir().join(format!("frilvault-test-{}", uuid::Uuid::new_v4()));

    fs::create_dir_all(&workspace_root).unwrap();

    let resolver = PathResolver::new(&workspace_root);
    let repository = YamlNoteRepository::new(resolver);
    let service = NoteService::new(repository);

    service
        .add_note(AddNoteInput {
            source_file: "src/main.rs".into(),
            anchor: crate::note::NoteAnchor::Line(crate::note::LineAnchor {
                line: 10,
                column: 5,
            }),
            content: "여기서 스캔 시작".to_string(),
        })
        .unwrap();

    let notes = service.list_notes("src/main.rs").unwrap();

    assert_eq!(notes.len(), 1);

    assert_eq!(notes[0].content, "여기서 스캔 시작");

    match &notes[0].anchor {
        crate::note::NoteAnchor::Line(anchor) => {
            assert_eq!(anchor.line, 10);
            assert_eq!(anchor.column, 5);
        }
        _ => panic!("Expected LineAnchor"),
    }

    fs::remove_dir_all(workspace_root).unwrap();
}

#[test]
fn delete_note_removes_note() {
    let workspace_root =
        std::env::temp_dir().join(format!("frilvault-test-{}", uuid::Uuid::new_v4()));

    fs::create_dir_all(&workspace_root).unwrap();

    let resolver = PathResolver::new(&workspace_root);

    let repository = YamlNoteRepository::new(resolver);

    let service = NoteService::new(repository);

    let note = service
        .add_note(AddNoteInput {
            source_file: "src/main.rs".into(),
            anchor: crate::note::NoteAnchor::Line(crate::note::LineAnchor {
                line: 10,
                column: 5,
            }),
            content: "delete me".to_string(),
        })
        .unwrap();

    service.delete_note("src/main.rs", note.id).unwrap();

    let notes = service.list_notes("src/main.rs").unwrap();

    assert_eq!(notes.len(), 0);

    fs::remove_dir_all(workspace_root).unwrap();
}

#[test]
fn update_note_changes_content() {
    let workspace_root =
        std::env::temp_dir().join(format!("frilvault-test-{}", uuid::Uuid::new_v4()));

    fs::create_dir_all(&workspace_root).unwrap();

    let resolver = PathResolver::new(&workspace_root);

    let repository = YamlNoteRepository::new(resolver);

    let service = NoteService::new(repository);

    let note = service
        .add_note(AddNoteInput {
            source_file: "src/main.rs".into(),
            anchor: crate::note::NoteAnchor::Line(crate::note::LineAnchor {
                line: 10,
                column: 5,
            }),
            content: "old content".to_string(),
        })
        .unwrap();

    service
        .update_note("src/main.rs", note.id, "new content".to_string())
        .unwrap();

    let notes = service.list_notes("src/main.rs").unwrap();

    assert_eq!(notes.len(), 1);

    assert_eq!(notes[0].content, "new content");

    fs::remove_dir_all(workspace_root).unwrap();
}
