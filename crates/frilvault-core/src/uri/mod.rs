mod note_uri;

pub use note_uri::{NOTE_URI_SCHEME, NOTE_URI_VERSION, ParsedNoteUri};

use std::path::Path;

use uuid::Uuid;

use crate::{
    FrilVaultError, FrilVaultResult, NoteAnchor, NoteView, note::NoteService,
    workspace::WorkspaceIndex,
};

pub struct NoteUriResolver;

impl NoteUriResolver {
    pub fn serialize(note_id: Uuid, workspace_root: &Path) -> FrilVaultResult<String> {
        ParsedNoteUri::serialize(note_id, workspace_root)
    }

    pub fn parse(uri: &str) -> FrilVaultResult<ParsedNoteUri> {
        ParsedNoteUri::parse(uri)
    }

    pub fn resolve(service: &mut NoteService, uri: &str) -> FrilVaultResult<NoteView> {
        let parsed = ParsedNoteUri::parse(uri)?;
        let current_workspace = service.workspace_root();

        if parsed.workspace != current_workspace {
            return Err(FrilVaultError::UnknownWorkspace(
                parsed.workspace.display().to_string(),
            ));
        }

        let view = service.find_note_by_id(parsed.note_id)?;
        let index = service.load_workspace_index()?;

        ensure_source_file_is_current(&view, &index, parsed.note_id)?;
        ensure_anchor_is_resolvable(&view, parsed.note_id)?;

        Ok(view)
    }
}

fn ensure_source_file_is_current(
    view: &NoteView,
    index: &WorkspaceIndex,
    note_id: Uuid,
) -> FrilVaultResult<()> {
    let source_file = view.source_file.to_string_lossy();
    let indexed = index
        .files
        .iter()
        .find(|file| file.source_file == source_file);

    match indexed {
        Some(file) if file.exists => Ok(()),
        _ => Err(FrilVaultError::StaleNote(note_id)),
    }
}

fn ensure_anchor_is_resolvable(view: &NoteView, note_id: Uuid) -> FrilVaultResult<()> {
    if matches!(view.note.anchor, NoteAnchor::Symbol(_)) && view.resolved.is_none() {
        return Err(FrilVaultError::UnresolvedAnchor(note_id));
    }

    Ok(())
}
