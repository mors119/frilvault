use std::fs;

use super::helper::{create_test_note_service, create_test_workspace_service};
use crate::{
    IndexDiff, IndexedFile, LineAnchor, NoteAnchor, PathResolver, SymbolAnchor, SymbolKind,
    WorkspaceIndex, WorkspaceIndexRepository, add_note_request::AddNoteRequest,
    tests::helper::create_test_index_repository,
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

    let mut service = create_test_note_service(&workspace_root);

    service
        .add_note(AddNoteRequest {
            source_file: "src/main.rs".into(),
            anchor: NoteAnchor::Line(LineAnchor {
                line: 10,
                column: 5,
            }),
            content: "main note".to_string(),
        })
        .unwrap();

    service
        .add_note(AddNoteRequest {
            source_file: "src/lib.rs".into(),
            anchor: NoteAnchor::Line(LineAnchor { line: 3, column: 1 }),
            content: "lib note".to_string(),
        })
        .unwrap();

    service
        .add_note(AddNoteRequest {
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

    let mut service = create_test_note_service(&workspace_root);

    service
        .add_note(AddNoteRequest {
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
fn health_check_detects_missing_files_from_note_repository() {
    let workspace_root =
        std::env::temp_dir().join(format!("frilvault-test-{}", uuid::Uuid::new_v4(),));

    fs::create_dir_all(&workspace_root).unwrap();

    let mut service = create_test_note_service(&workspace_root);

    service
        .add_note(AddNoteRequest {
            source_file: "src/missing.rs".into(),

            anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),

            content: "orphan".to_string(),
        })
        .unwrap();

    let mut workspace_service = create_test_workspace_service(&workspace_root);

    let health = workspace_service.health_check().unwrap();

    assert_eq!(health.missing_source_files.len(), 1,);

    assert_eq!(health.missing_source_files[0], "src/missing.rs",);

    fs::remove_dir_all(workspace_root).unwrap();
}

#[test]
fn stats_counts_line_and_symbol_notes() {
    let workspace_root =
        std::env::temp_dir().join(format!("frilvault-test-{}", uuid::Uuid::new_v4()));

    fs::create_dir_all(&workspace_root).unwrap();

    let mut service = create_test_note_service(&workspace_root);

    let mut workspace_service = create_test_workspace_service(&workspace_root);

    service
        .add_note(AddNoteRequest {
            source_file: "src/main.rs".into(),

            anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),

            content: "line note".to_string(),
        })
        .unwrap();

    service
        .add_note(AddNoteRequest {
            source_file: "src/main.rs".into(),

            anchor: NoteAnchor::Symbol(SymbolAnchor {
                name: "main".to_string(),

                kind: SymbolKind::Function,

                signature: None,

                line_hint: None,
            }),

            content: "symbol note".to_string(),
        })
        .unwrap();

    let stats = workspace_service.stats().unwrap();

    assert_eq!(stats.file_count, 1);

    assert_eq!(stats.existing_files, 0);

    assert_eq!(stats.missing_files, 1);

    assert_eq!(stats.total_notes, 2);

    assert_eq!(stats.line_notes, 1);

    assert_eq!(stats.symbol_notes, 1);

    fs::remove_dir_all(workspace_root).unwrap();
}

#[test]
fn repair_suggests_matching_file_names() {
    let workspace_root =
        std::env::temp_dir().join(format!("frilvault-test-{}", uuid::Uuid::new_v4(),));

    fs::create_dir_all(workspace_root.join("src/core")).unwrap();

    fs::write(workspace_root.join("src/core/lib.rs"), "").unwrap();

    let mut service = create_test_note_service(&workspace_root);

    service
        .add_note(AddNoteRequest {
            source_file: "src/parser/lib.rs".into(),

            anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),

            content: "orphan".to_string(),
        })
        .unwrap();

    let mut workspace_service = create_test_workspace_service(&workspace_root);

    let suggestions = workspace_service.repair_suggestions().unwrap();

    assert_eq!(suggestions.len(), 1,);

    assert_eq!(suggestions[0].candidates.len(), 1,);

    assert_eq!(suggestions[0].candidates[0], "src/core/lib.rs",);

    fs::remove_dir_all(workspace_root).unwrap();
}

#[test]
fn apply_repairs_moves_note_file() {
    let workspace_root =
        std::env::temp_dir().join(format!("frilvault-test-{}", uuid::Uuid::new_v4(),));

    fs::create_dir_all(workspace_root.join("src/core")).unwrap();

    fs::write(workspace_root.join("src/core/lib.rs"), "").unwrap();

    let resolver = PathResolver::new(&workspace_root);

    let mut service = create_test_note_service(&workspace_root);

    service
        .add_note(AddNoteRequest {
            source_file: "src/parser/lib.rs".into(),

            anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),

            content: "repair me".to_string(),
        })
        .unwrap();

    let old_note_path = resolver.resolve_note_path("src/parser/lib.rs");

    assert!(old_note_path.exists());

    let mut workspace_service = create_test_workspace_service(&workspace_root);

    let repaired = workspace_service.apply_repairs().unwrap();

    assert_eq!(repaired, 1,);

    let new_note_path = resolver.resolve_note_path("src/core/lib.rs");

    assert!(new_note_path.exists());

    assert!(!old_note_path.exists());

    fs::remove_dir_all(workspace_root).unwrap();
}

#[test]
fn detects_renamed_file_by_name_similarity() {
    let workspace_root =
        std::env::temp_dir().join(format!("frilvault-test-{}", uuid::Uuid::new_v4()));

    fs::create_dir_all(&workspace_root).unwrap();

    let old = WorkspaceIndex {
        version: 1,
        files: vec![IndexedFile {
            source_file: "src/parser.rs".to_string(),
            note_count: 1,
            exists: true,
        }],
    };

    let new = WorkspaceIndex {
        version: 1,
        files: vec![IndexedFile {
            source_file: "src/core/parser.rs".to_string(),
            note_count: 1,
            exists: true,
        }],
    };

    let repo = create_test_index_repository(&workspace_root);

    let moves = repo.detect_moves(&old, &new);

    assert_eq!(moves.len(), 1);
    assert!(moves[0].confidence > 0.5);

    fs::remove_dir_all(workspace_root).unwrap();
}

#[test]
fn detects_removed_and_added_files() {
    let workspace_root =
        std::env::temp_dir().join(format!("frilvault-test-{}", uuid::Uuid::new_v4()));

    fs::create_dir_all(&workspace_root).unwrap();

    let old = WorkspaceIndex {
        version: 1,
        files: vec![IndexedFile {
            source_file: "a.rs".to_string(),
            note_count: 1,
            exists: true,
        }],
    };

    let new = WorkspaceIndex {
        version: 1,
        files: vec![IndexedFile {
            source_file: "b.rs".to_string(),
            note_count: 1,
            exists: true,
        }],
    };

    let repo = create_test_index_repository(&workspace_root);

    let moves = repo.detect_moves(&old, &new);

    assert!(moves.is_empty());

    fs::remove_dir_all(workspace_root).unwrap();
}

#[test]
fn detects_strong_rename_by_name_and_path() {
    let old = "src/parser.rs";
    let new = "src/parser.rs";

    let score = IndexDiff::similarity_score(old, new);

    assert!(score >= 1.0);
}

#[test]
fn rejects_unrelated_files() {
    let old = "src/parser.rs";
    let new = "src/ui/button.rs";

    let score = IndexDiff::similarity_score(old, new);

    assert!(score < 1.0);
}
