use crate::{
    AddNoteInput, LineAnchor, NoteAnchor, NoteService, PathResolver, SymbolAnchor, SymbolKind,
    VaultContext, WorkspaceIndexRepository, YamlNoteRepository, constants::NOTE_FILE_EXTENSION,
};

use std::{fs, path::Path};

pub fn return_service(workspace_root: &Path) -> NoteService {
    let resolver = PathResolver::new(workspace_root);

    let note_repository = YamlNoteRepository::new(resolver.clone());
    let index_repository = WorkspaceIndexRepository::new(resolver);

    let vault_context = VaultContext::new(note_repository, index_repository);

    NoteService::new(vault_context)
}

#[test]
fn add_line_type_note_creates_yaml_file() {
    let workspace_root =
        std::env::temp_dir().join(format!("frilvault-test-{}", uuid::Uuid::new_v4()));

    fs::create_dir_all(&workspace_root).unwrap();

    let mut service = return_service(&workspace_root);

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
        .join("notes")
        .join(format!("src/main.rs.{}", NOTE_FILE_EXTENSION));

    assert!(note_path.exists());

    let content = fs::read_to_string(note_path).unwrap();

    assert!(content.contains("여기서 스캔이 시작된다."));

    fs::remove_dir_all(workspace_root).unwrap();
}

#[test]
fn add_symbol_type_note_creates_yaml_file() {
    let workspace_root =
        std::env::temp_dir().join(format!("frilvault-test-{}", uuid::Uuid::new_v4()));

    fs::create_dir_all(&workspace_root).unwrap();

    let mut service = return_service(&workspace_root);

    let input = AddNoteInput {
        source_file: "src/main.rs".into(),
        anchor: crate::note::NoteAnchor::Symbol(crate::note::SymbolAnchor {
            name: "NoteService::add_note".to_string(),
            kind: SymbolKind::Method,
            signature: Some(
                "fn add_note(&self, input: AddNoteInput) -> FrilVaultResult<Note>".to_owned(),
            ),
            line_hint: Some(34),
        }),
        content: "symbol anchor test".to_string(),
    };

    service.add_note(input).unwrap();

    let note_path = workspace_root
        .join(".vault")
        .join("notes")
        .join(format!("src/main.rs.{}", NOTE_FILE_EXTENSION));

    assert!(note_path.exists());

    let notes = service.list_notes("src/main.rs").unwrap();
    assert_eq!(notes.len(), 1);
    assert_eq!(notes[0].note.content, "symbol anchor test");
    match &notes[0].note.anchor {
        NoteAnchor::Symbol(anchor) => {
            assert_eq!(anchor.name, "NoteService::add_note");
            assert_eq!(anchor.kind, SymbolKind::Method);
            assert_eq!(
                anchor.signature.as_deref(),
                Some("fn add_note(&self, input: AddNoteInput) -> FrilVaultResult<Note>")
            );
            assert_eq!(anchor.line_hint, Some(34));
        }
        _ => panic!("Expected SymbolAnchor"),
    }

    fs::remove_dir_all(workspace_root).unwrap();
}

#[test]
fn load_notes_from_existing_yaml() {
    let workspace_root =
        std::env::temp_dir().join(format!("frilvault-test-{}", uuid::Uuid::new_v4()));

    fs::create_dir_all(&workspace_root).unwrap();

    let mut service = return_service(&workspace_root);

    service
        .add_note(AddNoteInput {
            source_file: "src/lib.rs".into(),
            anchor: crate::note::NoteAnchor::Line(crate::note::LineAnchor { line: 1, column: 1 }),
            content: "lib 진입점".to_string(),
        })
        .unwrap();

    let notes = service.list_notes("src/lib.rs").unwrap();

    assert_eq!(notes.len(), 1);
    assert_eq!(notes[0].note.content, "lib 진입점");

    fs::remove_dir_all(workspace_root).unwrap();
}

#[test]
fn add_note_and_load_note() {
    let workspace_root =
        std::env::temp_dir().join(format!("frilvault-test-{}", uuid::Uuid::new_v4()));

    fs::create_dir_all(&workspace_root).unwrap();

    let mut service = return_service(&workspace_root);

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

    assert_eq!(notes[0].note.content, "여기서 스캔 시작");

    match &notes[0].note.anchor {
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

    let mut service = return_service(&workspace_root);

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

    let mut service = return_service(&workspace_root);

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

    assert_eq!(notes[0].note.content, "new content");

    fs::remove_dir_all(workspace_root).unwrap();
}

#[test]
fn search_notes_finds_matching_notes() {
    let workspace_root =
        std::env::temp_dir().join(format!("frilvault-test-{}", uuid::Uuid::new_v4()));

    fs::create_dir_all(&workspace_root).unwrap();

    let mut service = return_service(&workspace_root);

    service
        .add_note(AddNoteInput {
            source_file: "src/main.rs".into(),

            anchor: NoteAnchor::Line(LineAnchor {
                line: 10,
                column: 5,
            }),

            content: "parser 개선 필요".to_string(),
        })
        .unwrap();

    service
        .add_note(AddNoteInput {
            source_file: "src/lib.rs".into(),

            anchor: NoteAnchor::Line(LineAnchor { line: 3, column: 1 }),

            content: "Parser trait 검토".to_string(),
        })
        .unwrap();

    service
        .add_note(AddNoteInput {
            source_file: "src/mod.rs".into(),

            anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),

            content: "hello world".to_string(),
        })
        .unwrap();

    let notes = service.search_notes("parser").unwrap();

    assert_eq!(notes.len(), 2,);

    assert!(
        notes
            .iter()
            .any(|result| result.source_file == std::path::Path::new("src/main.rs"))
    );

    // case_insensitive
    let notes = service.search_notes("PARSER").unwrap();

    assert_eq!(notes.len(), 2,);

    fs::remove_dir_all(workspace_root).unwrap();
}

#[test]
fn search_finds_symbol_anchor() {
    let workspace_root =
        std::env::temp_dir().join(format!("frilvault-test-{}", uuid::Uuid::new_v4(),));

    fs::create_dir_all(&workspace_root).unwrap();

    let mut service = return_service(&workspace_root);

    service
        .add_note(AddNoteInput {
            source_file: "src/service.rs".into(),

            anchor: NoteAnchor::Symbol(SymbolAnchor {
                name: "NoteService::add_note".to_string(),

                kind: SymbolKind::Method,

                signature: None,

                line_hint: None,
            }),

            content: "service logic".to_string(),
        })
        .unwrap();

    let results = service.search_notes("add_note").unwrap();

    assert_eq!(results.len(), 1,);

    fs::remove_dir_all(workspace_root).unwrap();
}
