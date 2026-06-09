use std::fs;

use crate::{PathResolver, WorkspaceRepository};

#[test]
fn create_if_missing_creates_workspace_metadata() {
    let workspace_root =
        std::env::temp_dir().join(format!("frilvault-test-{}", uuid::Uuid::new_v4(),));

    fs::create_dir_all(&workspace_root).unwrap();

    let resolver = PathResolver::new(&workspace_root);

    let repository = WorkspaceRepository::new(resolver.clone());

    repository.create_if_missing().unwrap();

    assert!(resolver.workspace_metadata_path().exists());

    fs::remove_dir_all(workspace_root).unwrap();
}

#[test]
fn create_if_missing_creates_default_directories() {
    let workspace_root =
        std::env::temp_dir().join(format!("frilvault-test-{}", uuid::Uuid::new_v4(),));

    fs::create_dir_all(&workspace_root).unwrap();

    let resolver = PathResolver::new(&workspace_root);

    let repository = WorkspaceRepository::new(resolver.clone());

    repository.create_if_missing().unwrap();

    assert!(resolver.vault_root().join("notes").exists());

    assert!(resolver.vault_root().join("cache").exists());

    assert!(resolver.vault_root().join("index").exists());

    fs::remove_dir_all(workspace_root).unwrap();
}

use crate::WorkspaceMetadata;

#[test]
fn load_returns_saved_workspace_metadata() {
    let workspace_root =
        std::env::temp_dir().join(format!("frilvault-test-{}", uuid::Uuid::new_v4(),));

    fs::create_dir_all(&workspace_root).unwrap();

    let resolver = PathResolver::new(&workspace_root);

    let repository = WorkspaceRepository::new(resolver);

    let metadata = WorkspaceMetadata::default();

    repository.save(&metadata).unwrap();

    let loaded = repository.load().unwrap();

    assert_eq!(loaded.version, metadata.version,);

    fs::remove_dir_all(workspace_root).unwrap();
}

#[test]
fn create_if_missing_does_not_overwrite_existing_metadata() {
    let workspace_root =
        std::env::temp_dir().join(format!("frilvault-test-{}", uuid::Uuid::new_v4(),));

    fs::create_dir_all(&workspace_root).unwrap();

    let resolver = PathResolver::new(&workspace_root);

    let repository = WorkspaceRepository::new(resolver.clone());

    let metadata = WorkspaceMetadata {
        version: 999,
        ..Default::default()
    };

    repository.save(&metadata).unwrap();

    repository.create_if_missing().unwrap();

    let loaded = repository.load().unwrap();

    assert_eq!(loaded.version, 999,);

    fs::remove_dir_all(workspace_root).unwrap();
}
