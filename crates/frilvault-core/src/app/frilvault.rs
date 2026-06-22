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

    fn build_context(&self) -> FrilVaultResult<(VaultContext, WorkspaceIndexRepository)> {
        let resolver = PathResolver::new(&self.workspace_root);

        let workspace_repository = WorkspaceRepository::new(resolver.clone());
        workspace_repository.create_if_missing()?;

        let index_repository = WorkspaceIndexRepository::new(resolver.clone());
        index_repository.create_if_missing()?;

        let note_repository = YamlNoteRepository::new(resolver.clone());

        let vault_context = VaultContext::new(note_repository, index_repository.clone());

        Ok((vault_context, index_repository))
    }

    pub fn notes(&self) -> FrilVaultResult<NoteService> {
        let (context, _) = self.build_context()?;
        Ok(NoteService::new(context))
    }

    pub fn workspace(&self) -> FrilVaultResult<WorkspaceService> {
        let (context, index_repository) = self.build_context()?;
        Ok(WorkspaceService::new(context, index_repository))
    }
}
