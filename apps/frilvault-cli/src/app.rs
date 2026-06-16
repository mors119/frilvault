use anyhow::Result;

use frilvault_core::{
    NoteService, PathResolver, VaultContext, WorkspaceIndexRepository, WorkspaceRepository,
    WorkspaceService, YamlNoteRepository,
};

pub fn create_note_service() -> Result<NoteService> {
    let workspace_root = std::env::current_dir()?;

    let resolver = PathResolver::new(workspace_root);

    let workspace_repository = WorkspaceRepository::new(resolver.clone());
    workspace_repository.create_if_missing()?;

    let index_repository = WorkspaceIndexRepository::new(resolver.clone());
    index_repository.create_if_missing()?;

    let note_repository = YamlNoteRepository::new(resolver.clone());
    let vault_context = VaultContext::new(note_repository, index_repository);

    Ok(NoteService::new(vault_context))
}

pub fn create_workspace_service() -> anyhow::Result<WorkspaceService> {
    let workspace_root = std::env::current_dir()?;

    let resolver = PathResolver::new(workspace_root);

    let workspace_repository = WorkspaceRepository::new(resolver.clone());
    workspace_repository.create_if_missing()?;

    let index_repository = WorkspaceIndexRepository::new(resolver.clone());
    index_repository.create_if_missing()?;

    let note_repository = YamlNoteRepository::new(resolver);
    let vault_context = VaultContext::new(note_repository, index_repository.clone());

    Ok(WorkspaceService::new(vault_context, index_repository))
}
