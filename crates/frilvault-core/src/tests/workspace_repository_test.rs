use super::helper::create_test_workspace;
use crate::workspace::{PathResolver, WorkspaceMetadata, WorkspaceRepository};

#[test]
fn create_if_missing_creates_workspace_metadata() {
    let workspace = create_test_workspace();
    let workspace_root = workspace.root();
    let resolver = PathResolver::new(workspace_root);

    let repository = WorkspaceRepository::new(resolver.clone());

    repository.create_if_missing().unwrap();

    assert!(resolver.workspace_metadata_path().exists());
}

#[test]
fn create_if_missing_creates_default_directories() {
    let workspace = create_test_workspace();
    let workspace_root = workspace.root();
    let resolver = PathResolver::new(workspace_root);

    let repository = WorkspaceRepository::new(resolver.clone());

    repository.create_if_missing().unwrap();

    assert!(resolver.vault_root().join("notes").exists());

    assert!(resolver.vault_root().join("cache").exists());

    assert!(resolver.vault_root().join("index").exists());
}

#[test]
fn load_returns_saved_workspace_metadata() {
    let workspace = create_test_workspace();
    let workspace_root = workspace.root();
    let resolver = PathResolver::new(workspace_root);

    let repository = WorkspaceRepository::new(resolver);

    let metadata = WorkspaceMetadata::default();

    repository.save(&metadata).unwrap();

    let loaded = repository.load().unwrap();

    assert_eq!(loaded.version, metadata.version,);
}

#[test]
fn create_if_missing_does_not_overwrite_existing_metadata() {
    let workspace = create_test_workspace();
    let workspace_root = workspace.root();
    let resolver = PathResolver::new(workspace_root);

    let repository = WorkspaceRepository::new(resolver.clone());

    let metadata = WorkspaceMetadata {
        version: 999,
        ..Default::default()
    };

    repository.save(&metadata).unwrap();

    repository.create_if_missing().unwrap();

    let loaded = repository.load().unwrap();

    assert_eq!(loaded.version, 999,);
}
