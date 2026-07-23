use std::fs;

use super::helper::{
    create_test_note_service, create_test_workspace, create_test_workspace_service,
};
use crate::{
    AddNoteRequest, FrilVaultError, LineAnchor, NoteAnchor, NoteUriResolver, ParsedNoteUri,
    SymbolAnchor, SymbolKind,
};

use uuid::Uuid;

#[test]
fn note_uri_round_trips_with_versioned_format() {
    let workspace = create_test_workspace();
    let note_id = Uuid::new_v4();

    let uri = ParsedNoteUri::serialize(note_id, workspace.root()).unwrap();
    let parsed = ParsedNoteUri::parse(&uri).unwrap();

    assert_eq!(parsed.note_id, note_id);
    assert_eq!(parsed.workspace, workspace.root().to_path_buf());
    assert!(uri.contains("frilvault://note/v1/"));
}

#[test]
fn note_uri_parses_legacy_unversioned_format() {
    let note_id = Uuid::new_v4();
    let workspace = "/tmp/workspace";
    let uri = format!(
        "frilvault://note/{note_id}?workspace={}",
        urlencoding_legacy(workspace)
    );

    let parsed = ParsedNoteUri::parse(&uri).unwrap();

    assert_eq!(parsed.note_id, note_id);
    assert_eq!(parsed.workspace, std::path::PathBuf::from(workspace));
}

#[test]
fn note_uri_rejects_malformed_inputs() {
    assert!(matches!(
        ParsedNoteUri::parse("https://example.com/note/v1/id?workspace=/tmp"),
        Err(FrilVaultError::MalformedNoteUri(_))
    ));
    assert!(matches!(
        ParsedNoteUri::parse("frilvault://note/not-a-uuid?workspace=/tmp"),
        Err(FrilVaultError::MalformedNoteUri(_))
    ));
    assert!(matches!(
        ParsedNoteUri::parse(
            "frilvault://note/v1/550e8400-e29b-41d4-a716-446655440000?workspace=relative/path"
        ),
        Err(FrilVaultError::MalformedNoteUri(_))
    ));
}

#[test]
fn note_uri_rejects_path_traversal_in_workspace() {
    let note_id = Uuid::new_v4();
    let uri = format!(
        "frilvault://note/v1/{note_id}?workspace={}",
        urlencoding_legacy("/tmp/workspace/../secret")
    );

    assert!(matches!(
        ParsedNoteUri::parse(&uri),
        Err(FrilVaultError::MalformedNoteUri(_))
    ));
}

#[test]
fn resolve_note_uri_returns_matching_note() {
    let workspace = create_test_workspace();
    let workspace_root = workspace.root();
    fs::create_dir_all(workspace_root.join("src")).unwrap();
    fs::write(workspace_root.join("src/main.rs"), "fn main() {}\n").unwrap();

    let mut service = create_test_note_service(workspace_root);
    let note = service
        .add_note(AddNoteRequest {
            source_file: "src/main.rs".into(),
            anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),
            content: "linked note".to_string(),
            tags: None,
        })
        .unwrap();

    let uri = service.note_uri(note.id).unwrap();
    let resolved = service.resolve_note_uri(&uri).unwrap();

    assert_eq!(resolved.note.id, note.id);
    assert_eq!(resolved.note.content, "linked note");
}

// Scenario: stable note URIs must keep working after source rename once workspace sync runs.
// 시나리오: workspace sync 이후 source rename이 있어도 stable note URI가 계속 동작해야 합니다.
#[test]
fn resolve_note_uri_survives_source_file_rename_after_sync() {
    let workspace = create_test_workspace();
    let workspace_root = workspace.root();
    fs::create_dir_all(workspace_root.join("src/parser")).unwrap();
    fs::create_dir_all(workspace_root.join("src/core")).unwrap();
    fs::write(workspace_root.join("src/parser/lib.rs"), "fn lib() {}\n").unwrap();

    let mut service = create_test_note_service(workspace_root);
    let note = service
        .add_note(AddNoteRequest {
            source_file: "src/parser/lib.rs".into(),
            anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),
            content: "rename-safe".to_string(),
            tags: None,
        })
        .unwrap();

    let uri = service.note_uri(note.id).unwrap();

    fs::rename(
        workspace_root.join("src/parser/lib.rs"),
        workspace_root.join("src/core/lib.rs"),
    )
    .unwrap();

    let mut workspace_service = create_test_workspace_service(workspace_root);
    workspace_service.warm_up().unwrap();
    assert_eq!(workspace_service.sync_source_file_changes().unwrap(), 1);

    let mut resolver_service = create_test_note_service(workspace_root);
    let resolved = resolver_service.resolve_note_uri(&uri).unwrap();

    assert_eq!(resolved.note.id, note.id);
    assert_eq!(resolved.source_file.to_string_lossy(), "src/core/lib.rs");
}

#[test]
fn resolve_note_uri_returns_stale_note_when_source_file_is_missing() {
    let workspace = create_test_workspace();
    let workspace_root = workspace.root();
    fs::create_dir_all(workspace_root.join("src")).unwrap();
    fs::write(workspace_root.join("src/main.rs"), "fn main() {}\n").unwrap();

    let mut service = create_test_note_service(workspace_root);
    let note = service
        .add_note(AddNoteRequest {
            source_file: "src/main.rs".into(),
            anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),
            content: "stale candidate".to_string(),
            tags: None,
        })
        .unwrap();

    let uri = service.note_uri(note.id).unwrap();
    fs::remove_file(workspace_root.join("src/main.rs")).unwrap();

    let index = service.load_workspace_index().unwrap();
    assert!(
        !index
            .files
            .iter()
            .find(|file| file.source_file == "src/main.rs")
            .map(|file| file.exists)
            .unwrap_or(true)
    );

    assert!(matches!(
        service.resolve_note_uri(&uri),
        Err(FrilVaultError::StaleNote(id)) if id == note.id
    ));
}

#[test]
fn resolve_note_uri_returns_unknown_workspace_for_mismatch() {
    let workspace = create_test_workspace();
    let workspace_root = workspace.root();
    fs::create_dir_all(workspace_root.join("src")).unwrap();
    fs::write(workspace_root.join("src/main.rs"), "fn main() {}\n").unwrap();

    let mut service = create_test_note_service(workspace_root);
    let note = service
        .add_note(AddNoteRequest {
            source_file: "src/main.rs".into(),
            anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),
            content: "note".to_string(),
            tags: None,
        })
        .unwrap();

    let foreign_uri =
        ParsedNoteUri::serialize(note.id, std::path::Path::new("/other/root")).unwrap();

    assert!(matches!(
        service.resolve_note_uri(&foreign_uri),
        Err(FrilVaultError::UnknownWorkspace(_))
    ));
}

#[test]
fn resolve_note_uri_returns_missing_note_error() {
    let workspace = create_test_workspace();
    let mut service = create_test_note_service(workspace.root());
    let missing_id = Uuid::new_v4();
    let uri = service.note_uri(missing_id).unwrap();

    assert!(matches!(
        service.resolve_note_uri(&uri),
        Err(FrilVaultError::NoteNotFound(id)) if id == missing_id
    ));
}

#[test]
fn resolve_note_uri_returns_unresolved_anchor_for_symbol_without_match() {
    let workspace = create_test_workspace();
    let workspace_root = workspace.root();
    fs::create_dir_all(workspace_root.join("src")).unwrap();
    fs::write(workspace_root.join("src/main.rs"), "fn main() {}\n").unwrap();

    let mut service = create_test_note_service(workspace_root);
    let note = service
        .add_note(AddNoteRequest {
            source_file: "src/main.rs".into(),
            anchor: NoteAnchor::Symbol(SymbolAnchor {
                name: "missing_fn".to_string(),
                kind: SymbolKind::Function,
                signature: None,
                line_hint: Some(1),
            }),
            content: "symbol note".to_string(),
            tags: None,
        })
        .unwrap();

    let uri = service.note_uri(note.id).unwrap();

    assert!(matches!(
        service.resolve_note_uri(&uri),
        Err(FrilVaultError::UnresolvedAnchor(id)) if id == note.id
    ));
}

#[test]
fn note_uri_resolver_serialize_matches_service_helper() {
    let workspace = create_test_workspace();
    let note_id = Uuid::new_v4();

    let from_resolver = NoteUriResolver::serialize(note_id, workspace.root()).unwrap();
    let parsed = NoteUriResolver::parse(&from_resolver).unwrap();

    assert_eq!(parsed.note_id, note_id);
}

fn urlencoding_legacy(input: &str) -> String {
    input
        .bytes()
        .map(|byte| match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' | b'/' => {
                (byte as char).to_string()
            }
            _ => format!("%{byte:02X}"),
        })
        .collect()
}
