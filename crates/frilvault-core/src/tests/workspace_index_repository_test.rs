use std::fs;

use crate::{
    AddNoteInput, LineAnchor, NoteAnchor, NoteService, PathResolver, SymbolAnchor, SymbolKind,
    VaultContext, WorkspaceIndex, WorkspaceIndexRepository, WorkspaceRepository, WorkspaceService,
    YamlNoteRepository,
};

fn create_vault_context(workspace_root: &std::path::Path) -> VaultContext {
    let resolver = PathResolver::new(workspace_root);
    let workspace_repository = WorkspaceRepository::new(resolver.clone());
    workspace_repository.create_if_missing().unwrap();

    let note_repository = YamlNoteRepository::new(resolver.clone());
    let index_repository = WorkspaceIndexRepository::new(resolver);
    index_repository.create_if_missing().unwrap();

    VaultContext::new(note_repository, index_repository)
}

fn create_note_service(workspace_root: &std::path::Path) -> NoteService {
    NoteService::new(create_vault_context(workspace_root))
}

fn create_workspace_service(workspace_root: &std::path::Path) -> WorkspaceService {
    let vault_context = create_vault_context(workspace_root);
    let resolver = PathResolver::new(workspace_root);
    let repository = WorkspaceIndexRepository::new(resolver);

    WorkspaceService::new(vault_context, repository)
}

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

    let mut service = create_note_service(&workspace_root);

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

    let mut service = create_note_service(&workspace_root);

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
fn health_check_detects_missing_files_from_note_repository() {
    let workspace_root =
        std::env::temp_dir().join(format!("frilvault-test-{}", uuid::Uuid::new_v4(),));

    fs::create_dir_all(&workspace_root).unwrap();

    let mut service = create_note_service(&workspace_root);

    service
        .add_note(AddNoteInput {
            source_file: "src/missing.rs".into(),

            anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),

            content: "orphan".to_string(),
        })
        .unwrap();

    let mut workspace_service = create_workspace_service(&workspace_root);

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

    let mut service = create_note_service(&workspace_root);

    let mut workspace_service = create_workspace_service(&workspace_root);

    service
        .add_note(AddNoteInput {
            source_file: "src/main.rs".into(),

            anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),

            content: "line note".to_string(),
        })
        .unwrap();

    service
        .add_note(AddNoteInput {
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

    let mut service = create_note_service(&workspace_root);

    service
        .add_note(AddNoteInput {
            source_file: "src/parser/lib.rs".into(),

            anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),

            content: "orphan".to_string(),
        })
        .unwrap();

    let mut workspace_service = create_workspace_service(&workspace_root);

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

    let mut service = create_note_service(&workspace_root);

    service
        .add_note(AddNoteInput {
            source_file: "src/parser/lib.rs".into(),

            anchor: NoteAnchor::Line(LineAnchor { line: 1, column: 1 }),

            content: "repair me".to_string(),
        })
        .unwrap();

    let old_note_path = resolver.resolve_note_path("src/parser/lib.rs");

    assert!(old_note_path.exists());

    let mut workspace_service = create_workspace_service(&workspace_root);

    let repaired = workspace_service.apply_repairs().unwrap();

    assert_eq!(repaired, 1,);

    let new_note_path = resolver.resolve_note_path("src/core/lib.rs");

    assert!(new_note_path.exists());

    assert!(!old_note_path.exists());

    fs::remove_dir_all(workspace_root).unwrap();
}
