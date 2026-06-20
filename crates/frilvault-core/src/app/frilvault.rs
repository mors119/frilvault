use std::path::{Path, PathBuf};

use crate::{
    FrilVaultResult,
    note::NoteService,
    runtime::VaultContext,
    storage::YamlNoteRepository,
    workspace::{PathResolver, WorkspaceIndexRepository, WorkspaceRepository, WorkspaceService},
};

pub struct FrilVault {
    workspace_root: PathBuf,
}

impl FrilVault {
    pub fn open(workspace_root: impl AsRef<Path>) -> FrilVaultResult<Self> {
        Ok(Self {
            workspace_root: workspace_root.as_ref().to_path_buf(),
        })
    }

    pub fn create_note_service(&self) -> FrilVaultResult<NoteService> {
        let resolver = PathResolver::new(&self.workspace_root);

        let workspace_repository = WorkspaceRepository::new(resolver.clone());
        workspace_repository.create_if_missing()?;

        let index_repository = WorkspaceIndexRepository::new(resolver.clone());
        index_repository.create_if_missing()?;

        let note_repository = YamlNoteRepository::new(resolver.clone());
        let vault_context = VaultContext::new(note_repository, index_repository);

        Ok(NoteService::new(vault_context))
    }

    pub fn create_workspace_service(&self) -> FrilVaultResult<WorkspaceService> {
        let resolver = PathResolver::new(&self.workspace_root);

        let workspace_repository = WorkspaceRepository::new(resolver.clone());
        workspace_repository.create_if_missing()?;

        let index_repository = WorkspaceIndexRepository::new(resolver.clone());
        index_repository.create_if_missing()?;

        let note_repository = YamlNoteRepository::new(resolver);
        let vault_context = VaultContext::new(note_repository, index_repository.clone());

        Ok(WorkspaceService::new(vault_context, index_repository))
    }
}
