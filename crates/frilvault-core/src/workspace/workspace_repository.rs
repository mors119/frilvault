use std::fs;

use crate::{
    FrilVaultResult,
    constants::{CACHE_DIR_NAME, INDEX_DIR_NAME, NOTES_DIR_NAME},
    workspace::{PathResolver, WorkspaceMetadata},
};

#[derive(Debug, Clone)]
pub struct WorkspaceRepository {
    path_resolver: PathResolver,
}

impl WorkspaceRepository {
    pub fn new(path_resolver: PathResolver) -> Self {
        Self { path_resolver }
    }

    pub fn load(&self) -> FrilVaultResult<WorkspaceMetadata> {
        let path = self.path_resolver.workspace_metadata_path();

        let content = fs::read_to_string(path)?;

        let metadata = serde_yml::from_str(&content)?;

        Ok(metadata)
    }

    pub fn save(&self, metadata: &WorkspaceMetadata) -> FrilVaultResult<()> {
        let path = self.path_resolver.workspace_metadata_path();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let yaml = serde_yml::to_string(metadata)?;

        fs::write(path, yaml)?;

        Ok(())
    }

    pub fn create_if_missing(&self) -> FrilVaultResult<()> {
        let vault_root = self.path_resolver.vault_root();

        fs::create_dir_all(vault_root.join(NOTES_DIR_NAME))?;
        fs::create_dir_all(vault_root.join(CACHE_DIR_NAME))?;
        fs::create_dir_all(vault_root.join(INDEX_DIR_NAME))?;

        let path = self.path_resolver.workspace_metadata_path();

        if path.exists() {
            return Ok(());
        }

        let metadata = WorkspaceMetadata::default();

        self.save(&metadata)?;

        Ok(())
    }
}
