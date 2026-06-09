use anyhow::Result;

use frilvault_core::{NoteService, PathResolver, WorkspaceRepository, YamlNoteRepository};

pub fn create_note_service() -> Result<NoteService<YamlNoteRepository>> {
    let workspace_root = std::env::current_dir()?;

    let resolver = PathResolver::new(workspace_root);

    let workspace_repository = WorkspaceRepository::new(resolver.clone());
    workspace_repository.create_if_missing()?;

    let note_repository = YamlNoteRepository::new(resolver);

    Ok(NoteService::new(note_repository))
}
