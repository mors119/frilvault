use std::path::Path;

use crate::{
    FrilVaultResult,
    note::{Note, NoteFile},
    storage::NoteFileRecord,
};

pub trait NoteRepository {
    fn append_note(&self, source_file: &Path, note: &Note) -> FrilVaultResult<()>;

    fn load_by_source_file(&self, source_file: &Path) -> FrilVaultResult<NoteFile>;

    fn replace_notes(&self, source_file: &Path, notes: Vec<Note>) -> FrilVaultResult<()>;

    fn list_all_note_files(&self) -> FrilVaultResult<Vec<NoteFileRecord>>;
}
