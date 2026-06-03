use anyhow::Result;

use frilvault_core::{NoteService, PathResolver, YamlNoteRepository};

pub mod add;
pub mod delete;
pub mod list;
pub mod update;

pub fn create_note_service() -> Result<NoteService> {
    let workspace_root = std::env::current_dir()?;

    let resolver = PathResolver::new(workspace_root);

    let repository = YamlNoteRepository::new(resolver);

    Ok(NoteService::new(repository))
}
