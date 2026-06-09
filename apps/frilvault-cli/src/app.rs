use anyhow::Result;

use frilvault_core::{
    NoteService, PathResolver, WorkspaceIndexRepository, WorkspaceRepository, WorkspaceService,
    YamlNoteRepository,
};

pub fn create_note_service() -> Result<NoteService<YamlNoteRepository>> {
    let workspace_root = std::env::current_dir()?;

    let resolver = PathResolver::new(workspace_root);

    let workspace_repository = WorkspaceRepository::new(resolver.clone());
    workspace_repository.create_if_missing()?;

    let note_repository = YamlNoteRepository::new(resolver);

    Ok(NoteService::new(note_repository))
}

pub fn create_workspace_service() -> anyhow::Result<WorkspaceService> {
    let workspace_root = std::env::current_dir()?;

    let resolver = PathResolver::new(workspace_root);

    let note_repository = YamlNoteRepository::new(resolver.clone());

    let index_repository = WorkspaceIndexRepository::new(resolver);

    Ok(WorkspaceService::new(note_repository, index_repository))
}
