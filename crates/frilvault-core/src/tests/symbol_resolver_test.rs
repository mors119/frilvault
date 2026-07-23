use std::fs;

use super::helper::{create_test_note_service, create_test_workspace};
use crate::{
    AddNoteRequest, NoteAnchor, SymbolAnchor, SymbolKind, SymbolResolver,
    workspace::read_source_file_content,
};

#[test]
fn symbol_note_survives_line_movement() {
    let workspace = create_test_workspace();
    let workspace_root = workspace.root();
    fs::create_dir_all(workspace_root.join("src")).unwrap();
    fs::write(
        workspace_root.join("src/main.rs"),
        "fn main() {\n    println!(\"hello\");\n}\n",
    )
    .unwrap();

    let mut service = create_test_note_service(workspace_root);
    service
        .add_note(AddNoteRequest {
            source_file: "src/main.rs".into(),
            anchor: NoteAnchor::Symbol(SymbolAnchor {
                name: "main".to_string(),
                kind: SymbolKind::Function,
                signature: Some("fn main()".to_string()),
                line_hint: Some(1),
            }),
            content: "entry point".to_string(),
            tags: None,
        })
        .unwrap();

    fs::write(
        workspace_root.join("src/main.rs"),
        "// added header\n// more context\n\nfn main() {\n    println!(\"hello\");\n}\n",
    )
    .unwrap();

    let notes = service.list_notes("src/main.rs").unwrap();

    assert_eq!(notes.len(), 1);
    let resolved = notes[0]
        .resolved
        .as_ref()
        .expect("expected resolved symbol");
    assert_eq!(resolved.line, 4);
}

#[test]
fn symbol_note_survives_function_relocation() {
    let workspace = create_test_workspace();
    let workspace_root = workspace.root();
    fs::create_dir_all(workspace_root.join("src")).unwrap();
    fs::write(
        workspace_root.join("src/main.rs"),
        "fn helper() {\n    true\n}\n\nfn main() {\n}\n",
    )
    .unwrap();

    let mut service = create_test_note_service(workspace_root);
    service
        .add_note(AddNoteRequest {
            source_file: "src/main.rs".into(),
            anchor: NoteAnchor::Symbol(SymbolAnchor {
                name: "helper".to_string(),
                kind: SymbolKind::Function,
                signature: Some("fn helper()".to_string()),
                line_hint: Some(1),
            }),
            content: "helper note".to_string(),
            tags: None,
        })
        .unwrap();

    fs::write(
        workspace_root.join("src/main.rs"),
        "fn main() {\n}\n\nfn helper() {\n    true\n}\n",
    )
    .unwrap();

    let notes = service.list_notes("src/main.rs").unwrap();

    assert_eq!(notes.len(), 1);
    let resolved = notes[0]
        .resolved
        .as_ref()
        .expect("expected resolved symbol");
    assert_eq!(resolved.line, 4);

    let content = read_source_file_content(workspace_root, "src/main.rs").unwrap();
    let direct = SymbolResolver::resolve(
        &SymbolAnchor {
            name: "helper".to_string(),
            kind: SymbolKind::Function,
            signature: Some("fn helper()".to_string()),
            line_hint: Some(1),
        },
        &content,
    )
    .unwrap();
    assert_eq!(direct.line, 4);
}

#[test]
fn find_symbol_in_source_locates_symbol_by_name() {
    let workspace = create_test_workspace();
    let workspace_root = workspace.root();
    fs::create_dir_all(workspace_root.join("src")).unwrap();
    fs::write(
        workspace_root.join("src/service.rs"),
        "pub struct UserService {\n    id: u64,\n}\n",
    )
    .unwrap();

    let service = create_test_note_service(workspace_root);
    let resolved = service
        .find_symbol_in_source("src/service.rs", "UserService", SymbolKind::Struct)
        .unwrap()
        .expect("expected resolved symbol");

    assert_eq!(resolved.line, 1);
}

#[test]
fn symbol_note_survives_file_relocation() {
    use crate::tests::helper::create_test_workspace_service;

    let workspace = create_test_workspace();
    let workspace_root = workspace.root();
    fs::create_dir_all(workspace_root.join("src")).unwrap();
    fs::write(workspace_root.join("src/parser.rs"), "pub struct Parser;\n").unwrap();

    let mut note_service = create_test_note_service(workspace_root);
    note_service
        .add_note(AddNoteRequest {
            source_file: "src/parser.rs".into(),
            anchor: NoteAnchor::Symbol(SymbolAnchor {
                name: "Parser".to_string(),
                kind: SymbolKind::Struct,
                signature: Some("pub struct Parser".to_string()),
                line_hint: Some(1),
            }),
            content: "parser note".to_string(),
            tags: None,
        })
        .unwrap();

    fs::create_dir_all(workspace_root.join("src/core")).unwrap();
    fs::rename(
        workspace_root.join("src/parser.rs"),
        workspace_root.join("src/core/parser.rs"),
    )
    .unwrap();

    let mut workspace_service = create_test_workspace_service(workspace_root);
    workspace_service.warm_up().unwrap();
    workspace_service.sync_source_file_changes().unwrap();

    let notes = note_service.list_notes("src/core/parser.rs").unwrap();

    assert_eq!(notes.len(), 1);
    let resolved = notes[0]
        .resolved
        .as_ref()
        .expect("expected resolved symbol");
    assert_eq!(resolved.line, 1);
}
