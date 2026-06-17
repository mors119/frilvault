use std::path::Path;

use crate::{
    NoteService, PathResolver, VaultContext, WorkspaceIndexRepository, WorkspaceRepository,
    WorkspaceService, YamlNoteRepository,
};

pub fn create_test_note_service(workspace_root: &Path) -> NoteService {
    let resolver = PathResolver::new(workspace_root);

    let note_repository = YamlNoteRepository::new(resolver.clone());
    let index_repository = WorkspaceIndexRepository::new(resolver);

    let vault_context = VaultContext::new(note_repository, index_repository);

    NoteService::new(vault_context)
}

fn create_test_vault_context(workspace_root: &Path) -> VaultContext {
    let resolver = PathResolver::new(workspace_root);
    let workspace_repository = WorkspaceRepository::new(resolver.clone());
    workspace_repository.create_if_missing().unwrap();

    let note_repository = YamlNoteRepository::new(resolver.clone());
    let index_repository = WorkspaceIndexRepository::new(resolver);
    index_repository.create_if_missing().unwrap();

    VaultContext::new(note_repository, index_repository)
}

pub fn create_test_workspace_service(workspace_root: &Path) -> WorkspaceService {
    let vault_context = create_test_vault_context(workspace_root);
    let resolver = PathResolver::new(workspace_root);
    let repository = WorkspaceIndexRepository::new(resolver);

    WorkspaceService::new(vault_context, repository)
}
